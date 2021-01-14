use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitties_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
    });
}
