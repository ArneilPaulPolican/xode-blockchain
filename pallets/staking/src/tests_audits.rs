use crate::{
	mock::*, 
	ProposedCandidates,
};
use frame_support::{
	assert_ok,
};
use frame_support::traits::{
	Currency, 
	Imbalance,
	Hooks,
};

// https://github.com/Xode-DAO/xode-blockchain/issues/111
// https://github.com/Xode-DAO/xode-blockchain/issues/110
#[test]
fn test_pallet_xode_staking_last_updated_and_incorrect_balance_works() {
	test1_ext().execute_with(|| {
		System::set_block_number(0);
		XodeStaking::on_initialize(System::block_number());

        let candidate = 1;
		assert_ok!(XodeStaking::register_candidate(RuntimeOrigin::signed(candidate)));

		// Scenario 0: when the current bond is zero
		let imbalance = Balances::deposit_creating(&candidate, 100_000_000_000_000_000);
		assert!(imbalance.peek() > 0, "Expected a positive imbalance for deposit creation");

		assert_ok!(XodeStaking::bond_candidate(RuntimeOrigin::signed(candidate), 11_000_000_000_000_000));	
		assert_eq!(89_000_000_000_000_000, Balances::free_balance(&candidate), "Must match");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 1, "The number of proposed candidates should be 1");	

		assert_eq!(proposed_candidates[0].last_updated, 0, "Must match");
		assert_eq!(proposed_candidates[0].bond, 11_000_000_000_000_000u128, "Must match");

		// Scenario 1: when the bond is lesser than the current bond
		System::set_block_number(1);
		XodeStaking::on_initialize(System::block_number());

		assert_ok!(XodeStaking::bond_candidate(RuntimeOrigin::signed(candidate), 5_000_000_000_000_000));
		assert_eq!(95_000_000_000_000_000, Balances::free_balance(&candidate), "Must match");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 1, "The number of proposed candidates should be 1");	

		assert_eq!(proposed_candidates[0].last_updated, 1, "Must match");
		assert_eq!(proposed_candidates[0].bond, 5_000_000_000_000_000u128, "Must match");

		// Scenario 2: when the new bond is greater than the current bond
		System::set_block_number(2);
		XodeStaking::on_initialize(System::block_number());

		assert_ok!(XodeStaking::bond_candidate(RuntimeOrigin::signed(candidate), 15_000_000_000_000_000));
		assert_eq!(85_000_000_000_000_000, Balances::free_balance(&candidate), "Must match");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 1, "The number of proposed candidates should be 1");	

		assert_eq!(proposed_candidates[0].last_updated, 2, "Must match");
		assert_eq!(proposed_candidates[0].bond, 15_000_000_000_000_000u128, "Must match");

		// Scenario 3: when the new bond is equal to the current bond
		System::set_block_number(3);
		XodeStaking::on_initialize(System::block_number());

		assert_ok!(XodeStaking::bond_candidate(RuntimeOrigin::signed(candidate), 15_000_000_000_000_000));
		assert_eq!(85_000_000_000_000_000, Balances::free_balance(&candidate), "Must match");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 1, "The number of proposed candidates should be 1");	

		assert_eq!(proposed_candidates[0].last_updated, 2, "Must match");
		assert_eq!(proposed_candidates[0].bond, 15_000_000_000_000_000u128, "Must match");
	});
}

// https://github.com/Xode-DAO/xode-blockchain/issues/109
#[test]
fn test_pallet_xode_staking_assignment_operator_works() {
	test1_ext().execute_with(|| {
        let candidate = 1;
		assert_ok!(XodeStaking::register_candidate(RuntimeOrigin::signed(candidate)));
	});
}

// https://github.com/Xode-DAO/xode-blockchain/issues/108
#[test]
fn test_pallet_xode_staking_on_initialize_weight_works() {
	test1_ext().execute_with(|| {
        let candidate = 1;
		assert_ok!(XodeStaking::register_candidate(RuntimeOrigin::signed(candidate)));
	});
}
