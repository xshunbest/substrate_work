use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn  create_claim_works() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone() ));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Option::Some((1, frame_system::Pallet::<Test>::block_number()))
        );
                 
	});
}

#[test]
fn  create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        PoeModule::create_claim(Origin::signed(1), claim.clone() );
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone() ),
            Error::<Test>::ProofAlreadyClaimed
        );
                 
	});
}

#[test]
fn  revoke_claim_works() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone() );
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone() ));
        assert_eq!(Proofs::<Test>::get(&claim), None);
                 
	});
}

#[test]
fn  revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
 
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone() ),
            Error::<Test>::NoSuchProof
        );
                 
	});
}

#[test]
fn  transfer_claim_work() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1];
        PoeModule::create_claim(Origin::signed(1), claim.clone() );
       
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Option::Some((1, frame_system::Pallet::<Test>::block_number()))
        );
         
        PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2);
 
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Option::Some((2, frame_system::Pallet::<Test>::block_number()))
        );
                 
	});
}

#[test]
fn  create_claim_badmeta() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,0];//beyond of 10  //StringLimit: u32 = 10

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone() ),
            Error::<Test>::BadMetadata
        );
 
                 
	});
}


