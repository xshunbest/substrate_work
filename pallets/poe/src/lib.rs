#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

    #[pallet::config]  // <-- Step 2. code block will replace this.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The maximum length of a proof stored on-chain.
		type StringLimit: Get<u32>;
    }


    #[pallet::event]   // <-- Step 3. code block will replace this.
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(T::AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(T::AccountId, Vec<u8>),
    }
    
    #[pallet::error]   // <-- Step 4. code block will replace this.
    pub enum Error<T> {
        /// The proof has already been claimed.
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked.
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it.
        NotProofOwner,
        /// Bad proof args
        BadMetadata
    }
    
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[pallet::storage] // <-- Step 5. code block will replace this.
    pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;  

    
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    #[pallet::call]   // <-- Step 6. code block will replace this.
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub(super) fn create_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
        ) -> DispatchResultWithPostInfo {

            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;
        
            ensure!(proof.len() <= T::StringLimit::get() as usize, Error::<T>::BadMetadata);

            // Verify that the specified proof has not already been claimed.         
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Get the block number from the FRAME System module.
            let current_block = <frame_system::Module<T>>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, proof));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;


            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&proof);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, proof));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
            dest: T::AccountId,
            //dest: <T>::Source,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;
            //let (owner, _) = Proofs::<T>::get(&proof);

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Get the block number from the FRAME System module.
            let current_block = <frame_system::Module<T>>::block_number();


            // Remove claim from storage.
            Proofs::<T>::remove(&proof);

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (&dest, current_block));

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, proof.clone()));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(dest, proof.clone()));


            Ok(().into())
        }


    }

}



