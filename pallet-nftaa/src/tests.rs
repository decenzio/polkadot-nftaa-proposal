use crate::{mock::*, Error, Something};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::AccountId32 as AccountId;

pub const ALICE: AccountId = AccountId::new([0u8; 32]);

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(NFTAA::do_something(RuntimeOrigin::signed(ALICE), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(Something::<Test>::get().map(|v| v.block_number), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            NFTAA::cause_error(RuntimeOrigin::signed(ALICE)),
            Error::<Test>::NoneValue
        );
    });
}
