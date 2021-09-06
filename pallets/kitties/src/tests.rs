#![cfg(test)]

use crate::{
	mock::*, pallet::{ Error }
};
use frame_support::{assert_ok, assert_noop};

#[test]
fn should_build_genesis_kitties() {
	new_test_ext().execute_with(|| {
		// Check we have 2 kitties, as specified
		assert_eq!(Kitties::kitty_cnt(), 2);

		// Check owners own the correct amount of kitties
		let kitties_owned_by_1 = Kitties::kitty_owned(1);
		assert_eq!(kitties_owned_by_1.len(), 1);

		let kitties_owned_by_2 = Kitties::kitty_owned(2);
		assert_eq!(kitties_owned_by_2.len(), 1);

		// Check that kitties are owned correctly
		let kid1 = kitties_owned_by_1[0];
		let kitty1 = Kitties::kitties(kid1)
			.expect("Could have this kitty ID owned by acct 1");
		assert_eq!(kitty1.owner, 1);

		let kid2 = kitties_owned_by_2[0];
		let kitty2 = Kitties::kitties(kid2)
			.expect("Could have this kitty ID owned by acct 2");
		assert_eq!(kitty2.owner, 2);
	});
}

// #[test]
// fn create_kitty_should_work() {
// 	new_test_ext().execute_with(|| {
// 		// create a kitty with account #10.
// 		assert_ok!(Kitties::create_kitty(Origin::signed(1)));

// 		// check that 2 kitty existed already in genesis
// 		assert_eq!(Kitties::kitty_cnt(), 1);

// 		// check that account #1 owns 1 kitty
// 		assert_eq!(Kitties::owned_kitty_count(10), 1);

// 		// check that some random account #5 does not own a kitty
// 		assert_eq!(Kitties::owned_kitty_count(5), 0);

// 		// check that this kitty is specifically owned by account #10
// 		let hash = Kitties::kitty_by_index(2);
// 		assert_eq!(Kitties::owner_of(hash), Some(10));

// 		let other_hash = Kitties::kitty_of_owner_by_index((10, 0));
// 		assert_eq!(hash, other_hash);
// 	});
// }

// #[test]
// fn transfer_kitty_should_work() {
// 	new_test_ext().execute_with(|| {
// 		// check that 10 own a kitty
// 		assert_ok!(Kitties::create_kitty(Origin::signed(10)));

// 		assert_eq!(Kitties::owned_kitty_count(10), 1);
// 		let hash = Kitties::kitty_of_owner_by_index((10, 0));

// 		// send kitty to 3
// 		assert_ok!(Kitties::transfer(Origin::signed(10), 3, hash));

// 		// 10 now has nothing
// 		assert_eq!(Kitties::owned_kitty_count(10), 0);
// 		// but 3 does
// 		assert_eq!(Kitties::owned_kitty_count(3), 1);
// 		let new_hash = Kitties::kitty_of_owner_by_index((3, 0));
// 		// and it has the same hash
// 		assert_eq!(hash, new_hash);
// 	});
// }

// #[test]
// fn transfer_not_owned_kitty_should_fail() {
// 	new_test_ext().execute_with(|| {
// 		// check that 10 own a kitty
// 		assert_ok!(Kitties::create_kitty(Origin::signed(10)));
// 		let hash = Kitties::kitty_of_owner_by_index((10, 0));

// 		// account 0 cannot transfer a kitty with this hash.
// 		assert_noop!(
// 			Kitties::transfer(Origin::signed(9), 1, hash),
// 			"You do not own this kitty"
// 		);
// 	});
// }
