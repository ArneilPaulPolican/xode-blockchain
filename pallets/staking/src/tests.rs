use crate::{mock::*, CandidateInfo, Status, ActualAuthors,
	DesiredCandidates, ProposedCandidates, WaitingCandidates,
};
use frame_support::traits::Hooks;
use pallet_session::SessionManager;
use frame_support::traits::Currency;
use sp_core::sr25519;

#[test]
fn test_pallet_xode_staking() {
	test1_ext().execute_with(|| {
		// ========================================================================
		// SCENE 1 (Initialization): At Block 0 and Session 1 initialization
		// ------------------------------------------------------------------------
		// 1. There are three (3) desired candidates as set in the mock runtime.
		// 2. Provide balances for the three (3) desired candidates.
		// 3. We expect them to author so we provide session keys.
		// 4. Then advance the block and session so that these desired candidates 
		//    will be sent to invulnerable in the collator selection at the same 
		//	  time queued keys and authorities are updated.
		// 5. The authorities is still 0 at Session 1 
		// ========================================================================
		System::set_block_number(0);

		XodeStaking::on_initialize(System::block_number());
		
		let desired_candidates = DesiredCandidates::<Test>::get();
		assert_eq!(desired_candidates.len(), 3, "There should be exactly three desired candidates");

		let _ = Balances::deposit_creating(&desired_candidates[0], 1000);
		let key = sr25519::Public::from_raw([1u8; 32]);
		let session_keys = SessionKeys { aura: key.into(),};
		let result = Session::set_keys(RuntimeOrigin::signed(desired_candidates[0]), session_keys.clone(), Vec::new());
		println!("{:?} free balance: {:?}, {:?}: {:?}",desired_candidates[0], Balances::free_balance(&desired_candidates[0]), session_keys, result);
		assert!(result.is_ok(), "Failed to set session keys: {:?}", result);
		let _ = Balances::deposit_creating(&desired_candidates[1], 1000);
		let key = sr25519::Public::from_raw([2u8; 32]);
		let session_keys = SessionKeys { aura: key.into(),};
		let result = Session::set_keys(RuntimeOrigin::signed(desired_candidates[1]), session_keys.clone(), Vec::new());
		println!("{:?} free balance: {:?}, {:?}: {:?}",desired_candidates[1], Balances::free_balance(&desired_candidates[1]), session_keys, result);
		assert!(result.is_ok(), "Failed to set session keys: {:?}", result);
		let _ = Balances::deposit_creating(&desired_candidates[2], 1000);
		let key = sr25519::Public::from_raw([3u8; 32]);
		let session_keys = SessionKeys { aura: key.into(),};
		let result = Session::set_keys(RuntimeOrigin::signed(desired_candidates[2]), session_keys.clone(), Vec::new());
		println!("{:?} free balance: {:?}, {:?}: {:?}",desired_candidates[2], Balances::free_balance(&desired_candidates[2]), session_keys, result);
		assert!(result.is_ok(), "Failed to set session keys: {:?}", result);

		System::set_block_number((1 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(1);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 0, "There are no authorities yet (Taken from the last session).");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 3, "Invulerables after new session must have 3 entries, equal to desired candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 3, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 0, "No proposed candidates yet.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(desired_candidates, waiting_candidates, "The waiting candidates is equal to the desired candidates");

		// =======================================================================
		// SCENE 2 (Registering): Within Session 1 and Session 2 initialization
		// -----------------------------------------------------------------------
		// 1. Register two proposed candidates (Candidate-1 and Candidate-2).
		// 2. Bond these candidates and check the free balance. 
		// 3. Check also the sorting of proposed candidates.
		// 4. At Session 2 initialization, we have 3 authorities which is taken 
		//    from Session 1.
		// =======================================================================
		let _ = XodeStaking::register_candidate(RuntimeOrigin::signed(1));
		let _ = XodeStaking::register_candidate(RuntimeOrigin::signed(2));
		let mut candidate_1 = CandidateInfo {
			who: 1,
			bond: 0,
			total_stake: 0,
			last_updated: System::block_number(),
			leaving: false,
			offline: false,
			commission: 0,
			status: Status::Online,
			status_level: 0,
		};
		let mut candidate_2 = CandidateInfo {
			who: 2,
			bond: 0,
			total_stake: 0,
			last_updated: System::block_number(),
			leaving: false,
			offline: false,
			commission: 0,
			status: Status::Online,
			status_level: 0,
		};

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 2, "The number of proposed candidates should be 2");
		assert_eq!(proposed_candidates[0], candidate_1, "The first candidate data does not match");
		assert_eq!(proposed_candidates[1], candidate_2, "The second candidate data does not match");

		let _ = Balances::deposit_creating(&1, 1000);
		let key = sr25519::Public::from_raw([11u8; 32]);
		let session_keys = SessionKeys { aura: key.into(),};
		let result = Session::set_keys(RuntimeOrigin::signed(1), session_keys.clone(), Vec::new());
		assert!(result.is_ok(), "Failed to set session keys: {:?}", result);

		let _ = Balances::deposit_creating(&2, 1000);
		let key = sr25519::Public::from_raw([12u8; 32]);
		let session_keys = SessionKeys { aura: key.into(),};
		let result = Session::set_keys(RuntimeOrigin::signed(2), session_keys.clone(), Vec::new());
		assert!(result.is_ok(), "Failed to set session keys: {:?}", result);

		let _ = XodeStaking::bond_candidate(RuntimeOrigin::signed(1), 100);
		let _ = XodeStaking::bond_candidate(RuntimeOrigin::signed(2), 200);

		assert_eq!(Balances::free_balance(&1), 900);
		assert_eq!(Balances::free_balance(&2), 800);

		candidate_1.bond = 100;
		candidate_1.last_updated = System::block_number();
		candidate_2.bond = 200;
		candidate_2.last_updated = System::block_number();
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2, "The first candidate data does not match");
		assert_eq!(proposed_candidates[1], candidate_1, "The second candidate data does not match");

		System::set_block_number((2 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(2);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 3, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 3, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 3, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "The waiting candidates is equal to the desired candidates and proposed candidates");

		candidate_1.status = Status::Waiting;
		candidate_2.status = Status::Waiting;

		// =======================================================================
		// SCENE 3 (Bonding): Within Session 2 and Session 3 initialization
		// -----------------------------------------------------------------------
		// 1. Increase and decrease bonds of the proposed candidates.
		// 2. While decreasing the bond try incrementing the block number
		// 3. Take note of the actual authors on this session.  The proposed 
		//    candidates must author a block within Session 2 or it will be set to
		//    offline.  Todo: Slashed if not authoring.
		// =======================================================================
		let _ = XodeStaking::bond_candidate(RuntimeOrigin::signed(1), 400);

		assert_eq!(Balances::free_balance(&1), 600);

		candidate_1.bond = 400;
		candidate_1.last_updated = System::block_number();
		
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_1, "The first candidate data does not match");
		assert_eq!(proposed_candidates[1], candidate_2, "The second candidate data does not match");

		System::set_block_number(System::block_number() + 1);

		XodeStaking::on_initialize(System::block_number());

		let _ = XodeStaking::bond_candidate(RuntimeOrigin::signed(1), 200);

		assert_eq!(Balances::free_balance(&1), 800);

		candidate_1.bond = 200;
		candidate_1.last_updated = System::block_number();
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2, "The first candidate data does not match");
		assert_eq!(proposed_candidates[1], candidate_1, "The second candidate data does not match");

		System::set_block_number((3 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(3);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 3, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "The waiting candidates is equal to the desired candidates and proposed candidates");

		// No more queuing, immediately sent to authoring at level 0
		candidate_1.status = Status::Authoring;
		candidate_2.status = Status::Authoring;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2, "The first candidate data does not match");
		assert_eq!(proposed_candidates[1], candidate_1, "The second candidate data does not match");

		// =======================================================================
		// SCENE 4 (Staking): Within Session 3 and Session 4 initialization
		// ----------------------------------------------------------------------- 
		// 1. Add stakes on the proposed candidates
		// 2. Try to increase the block and execute un-stake
		// 3. Todo: We need to check if one of the proposed candidates did not
		//          author a block, we cannot assume!
		// 4. Todo: After un-staking (Separate Test)
		//			test_pallet_xode_staking_unstaked()
		//		4.1. Unreserved the balance
		//		4.2. If the stake is zero, remove the delegation
		//		4.3. Stake the same candidate again
		// =======================================================================
		let _ = Balances::deposit_creating(&11, 1000);
		let _ = Balances::deposit_creating(&12, 1000);
		let _ = Balances::deposit_creating(&13, 1000);
		let _ = Balances::deposit_creating(&21, 1000);
		let _ = Balances::deposit_creating(&22, 1000);
		let _ = Balances::deposit_creating(&23, 1000);

		let _ = XodeStaking::stake_candidate(RuntimeOrigin::signed(11), 1, 10);
		let _ = XodeStaking::stake_candidate(RuntimeOrigin::signed(12), 1, 20);
		let _ = XodeStaking::stake_candidate(RuntimeOrigin::signed(13), 1, 30);

		let _ = XodeStaking::stake_candidate(RuntimeOrigin::signed(21), 2, 10);
		let _ = XodeStaking::stake_candidate(RuntimeOrigin::signed(22), 2, 20);
		let _ = XodeStaking::stake_candidate(RuntimeOrigin::signed(23), 2, 30);

		assert_eq!(Balances::free_balance(&11), 990);
		assert_eq!(Balances::free_balance(&12), 980);
		assert_eq!(Balances::free_balance(&13), 970);

		candidate_1.total_stake = 60;
		candidate_1.last_updated = System::block_number();
		candidate_2.total_stake = 60;
		candidate_2.last_updated = System::block_number();

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_1);
		assert_eq!(proposed_candidates[1], candidate_2);	

		System::set_block_number(System::block_number() + 1);

		XodeStaking::on_initialize(System::block_number());

		let _ = XodeStaking::unstake_candidate(RuntimeOrigin::signed(13), 1);
		
		candidate_1.total_stake = 30;
		candidate_1.last_updated = System::block_number();

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2);
		assert_eq!(proposed_candidates[1], candidate_1);

		System::set_block_number((4 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(4);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 5, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "The waiting candidates is equal to the desired candidates and proposed candidates");

		candidate_1.status = Status::Authoring;
		candidate_1.status_level = 1;
		candidate_2.status = Status::Authoring;
		candidate_2.status_level = 1;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2, "The first candidate data does not match");
		assert_eq!(proposed_candidates[1], candidate_1, "The second candidate data does not match");

		// =======================================================================
		// SCENE 5 (Offline): Within Session 4, 5 and Session 6 initialization
		// ----------------------------------------------------------------------- 
		// 1. Make one proposed candidate go offline
		// 2. Downgraded after set to offline after two sessions.
		// 3. Todo: Test to online if the status is not yet downgraded
		// 4. Todo: If the candidate is offline or in the process of offline. (Separate
		//          Test).
		//          test_pallet_xode_staking_offline()
		//		4.1. Cannot bond
		//		4.2. Cannot stake
		//		4.3. Can un-stake
		// =======================================================================
		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());

		let _ = XodeStaking::offline_candidate(RuntimeOrigin::signed(2));

		candidate_2.offline = true;
		candidate_2.last_updated = System::block_number();

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_1);
		assert_eq!(proposed_candidates[1], candidate_2);		

		System::set_block_number((5 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(5);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 5, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 4, "The waiting candidates is equal to the desired candidates and online proposed candidates");	

		System::set_block_number((6 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(6);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 5, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 4, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 4, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 4, "The waiting candidates is equal to the desired candidates and online proposed candidates");	

		candidate_2.status = Status::Queuing;
		candidate_2.status_level = 0;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_1);
		assert_eq!(proposed_candidates[1], candidate_2);
		
		// =======================================================================
		// SCENE 6 (Online): Within Session 6, 7, 8 and Session 9 initialization 
		// ----------------------------------------------------------------------- 
		// 1. Make one proposed candidate go online after being offline
		// 2. Todo: Control, set online only if the status is already queuing
		// =======================================================================
		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());

		let _ = XodeStaking::online_candidate(RuntimeOrigin::signed(2));

		candidate_2.offline = false;
		candidate_2.last_updated = System::block_number();

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2);
		assert_eq!(proposed_candidates[1], candidate_1);	

		System::set_block_number((7 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(7);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 4, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 4, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 4, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "The waiting candidates is equal to the desired candidates and online proposed candidates");		

		candidate_2.status = Status::Queuing;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2);
		assert_eq!(proposed_candidates[1], candidate_1);

		System::set_block_number((8 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(8);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 4, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "The waiting candidates is equal to the desired candidates and online proposed candidates");	

		candidate_2.status = Status::Authoring;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2);
		assert_eq!(proposed_candidates[1], candidate_1);

		System::set_block_number((9 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(9);
		
		Session::on_initialize(System::block_number()); 

		let authorities = pallet_aura::Authorities::<Test>::get();
		println!("Authorities {:?}",authorities);
		assert_eq!(authorities.len(), 5, "Authorities are exactly equal to the previous invulnerables.");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "Invulerables after new session must have 5 entries, equal to desired candidates plus 2 proposed candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Keys are exactly equal to invulnerables.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Two (2) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "The waiting candidates is equal to the desired candidates and online proposed candidates");	

		candidate_2.status_level = 1;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2);
		assert_eq!(proposed_candidates[1], candidate_1);

		// =======================================================================
		// SCENE 7 (Leaving): Within Session 9, 10, 11, 12, 13 and Session 14 initialization
		// ----------------------------------------------------------------------- 
		// 1. Add a new candidate (Candidate-3), bond then leave
		// 2. Make sure to first offline the proposed candidate before leaving
		// 3. Todo: Control, if on the process of leaving: (Separate Test)
		//          test_pallet_xode_staking_leaving()
		//		3.1. Cannot online/offline
		//		3.2. Cannot register
		//		3.3. Cannot bond
		//		3.4. Cannot stake
		//		3.5. Can still un-stake
		// 4. Todo: Once the candidate has been removed in the authorities: (Separate
		//		    Test)
		//          test_pallet_xode_staking_left()
		//		4.1. Unreserve the bond
		//		4.2. Unreserve the stakes
		//		4.3. Remove all the delegation
		//		4.4. Remove the proposed candidate
		//		4.5. Test adding the same candidate again
		// =======================================================================
		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());

		let _ = XodeStaking::register_candidate(RuntimeOrigin::signed(3));
		let mut candidate_3 = CandidateInfo {
			who: 3,
			bond: 0,
			total_stake: 0,
			last_updated: System::block_number(),
			leaving: false,
			offline: false,
			commission: 0,
			status: Status::Online,
			status_level: 0,
		};

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 3, "The number of proposed candidates should be 3");
		assert_eq!(proposed_candidates[2], candidate_3, "Must match with the last proposed candidate");		

		let _ = Balances::deposit_creating(&3, 1000);
		let key = sr25519::Public::from_raw([13u8; 32]);
		let session_keys = SessionKeys { aura: key.into(),};
		let result = Session::set_keys(RuntimeOrigin::signed(3), session_keys.clone(), Vec::new());
		assert!(result.is_ok(), "Failed to set session keys: {:?}", result);

		let _ = XodeStaking::bond_candidate(RuntimeOrigin::signed(3), 300);
		assert_eq!(Balances::free_balance(&3), 700);

		candidate_3.bond = 300;
		candidate_3.last_updated = System::block_number();
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_3, "After bonding it will be the first proposed candidate.");

		System::set_block_number((10 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(10);
		
		Session::on_initialize(System::block_number()); 

		let validators = pallet_session::Validators::<Test>::get();
		println!("Validators {:?}",validators);
		assert_eq!(validators.len(), 5, "First session, still unchanged");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "On first session, un-change!");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Must be equal to invulnerables always.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 3, "Three (3) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 6, "On first session, equal to the desired and proposed candidates immediately.");		

		candidate_3.status = Status::Waiting;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_3, "Still waiting, do not leave");

		System::set_block_number((10 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(10);
		
		Session::on_initialize(System::block_number()); 

		let validators = pallet_session::Validators::<Test>::get();
		println!("Validators {:?}",validators);
		assert_eq!(validators.len(), 5, "First session, still unchanged");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 6, "On first session, un-change!");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 6, "Must be equal to invulnerables always.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 3, "Three (3) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 6, "On first session, equal to the desired and proposed candidates immediately.");			

		// No queuing status, if new waiting candidate.  Immediately goes to authoring
		// level 0.
		candidate_3.status = Status::Authoring;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_3, "Status is now authoring level 0 (no queuing).");

		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());

		let _ = XodeStaking::offline_candidate(RuntimeOrigin::signed(3));

		candidate_3.offline = true;
		candidate_3.last_updated = System::block_number();

		System::set_block_number((11 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(11);
		
		Session::on_initialize(System::block_number()); 

		let validators = pallet_session::Validators::<Test>::get();
		println!("Validators {:?}",validators);
		assert_eq!(validators.len(), 6, "First session, still unchanged");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 6, "On first session, un-change!");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 6, "Must be equal to invulnerables always.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 3, "Three (3) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "On first session, equal to the desired and proposed candidates immediately.");		

		candidate_3.status = Status::Authoring;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[2], candidate_3, "Status is still authoring.");

		System::set_block_number((12 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(12);
		
		Session::on_initialize(System::block_number()); 

		let validators = pallet_session::Validators::<Test>::get();
		println!("Validators {:?}",validators);
		assert_eq!(validators.len(), 6, "First session, still unchanged");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "On first session, un-change!");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Must be equal to invulnerables always.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 3, "Three (3) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "On first session, equal to the desired and proposed candidates immediately.");		

		candidate_3.status = Status::Queuing;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[2], candidate_3, "Status is now queuing after being offline.");

		System::set_block_number((13 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(13);
		
		Session::on_initialize(System::block_number()); 

		let validators = pallet_session::Validators::<Test>::get();
		println!("Validators {:?}",validators);
		assert_eq!(validators.len(), 5, "First session, still unchanged");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "On first session, un-change!");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Must be equal to invulnerables always.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 3, "Three (3) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "On first session, equal to the desired and proposed candidates immediately.");		

		candidate_3.status = Status::Waiting;
		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[2], candidate_3, "Status is now waiting (Ready to leave)");

		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());

		let leaving = XodeStaking::leave_candidate(RuntimeOrigin::signed(3));
		println!("Leaving {:?}",leaving);

		candidate_3.leaving = true;
		candidate_3.last_updated = System::block_number();

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates.len(), 3, "The number of proposed candidates should be 3");
		assert_eq!(proposed_candidates[2], candidate_3, "Must match with the last proposed candidate, because it is leaving.");	

		System::set_block_number((14 * MINUTES).into());

		XodeStaking::on_initialize(System::block_number());
		XodeStaking::new_session(14);
		
		Session::on_initialize(System::block_number()); 

		let validators = pallet_session::Validators::<Test>::get();
		println!("Validators {:?}",validators);
		assert_eq!(validators.len(), 5, "First session, still unchanged");

		let invulnerables = pallet_collator_selection::Invulnerables::<Test>::get();
		println!("Invulnerables {:?}",invulnerables);
		assert_eq!(invulnerables.len(), 5, "On second session, it will get the previous waiting candidates");

		let queued_keys = pallet_session::QueuedKeys::<Test>::get();
		println!("Keys {:?}",queued_keys);
		assert_eq!(queued_keys.len(), 5, "Must be equal to invulnerables always.");

		let proposed_candidates = ProposedCandidates::<Test>::get();
		println!("Proposed Candidates {:?}",proposed_candidates);
		assert_eq!(proposed_candidates.len(), 2, "Three (3) proposed candidates.");

		let waiting_candidates = WaitingCandidates::<Test>::get();
		println!("Waiting Candidates {:?}",waiting_candidates);
		assert_eq!(waiting_candidates.len(), 5, "On first session, less than one, since one candidate is leaving");		

		// =======================================================================
		// SCENE 8 (Authoring): Within Session 14 and Session 15 initialization
		// ----------------------------------------------------------------------- 
		// 1. Set the commission for both candidate
		// 2. Try authoring a block, do not call Authorship::author()
		// 3. Modified the Authorship FindAuthor setting
		// =======================================================================
		AuthorGiven::set_author(proposed_candidates[1].who);

		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());

		Authorship::on_initialize(System::block_number());
		Authorship::on_finalize(System::block_number());
		
		let _ = XodeStaking::set_commission_of_candidate(RuntimeOrigin::signed(1),10);
		let _ = XodeStaking::set_commission_of_candidate(RuntimeOrigin::signed(2),20);

		candidate_1.commission = 10;
		candidate_1.status = Status::Authoring;
		candidate_1.status_level = 1;
		candidate_1.last_updated = System::block_number();
		candidate_2.commission = 20;
		candidate_2.status = Status::Authoring;
		candidate_2.status_level = 1;
		candidate_2.last_updated = System::block_number();

		let proposed_candidates = ProposedCandidates::<Test>::get();
		assert_eq!(proposed_candidates[0], candidate_2);
		assert_eq!(proposed_candidates[1], candidate_1);

		let actual_authors = ActualAuthors::<Test>::get();
		println!("Actual Authors {:?}",actual_authors);
		assert_eq!(actual_authors.len(), 1);

		assert_eq!(Balances::free_balance(&1), 800);
		
		AuthorGiven::set_author(proposed_candidates[0].who);

		System::set_block_number(System::block_number() + 1);
		XodeStaking::on_initialize(System::block_number());		

		Authorship::on_initialize(System::block_number());
		Authorship::on_finalize(System::block_number());

		let author = Authorship::author();
		assert_eq!(author, Some(proposed_candidates[0].who)); 

		let actual_authors = ActualAuthors::<Test>::get();
		println!("Actual Authors {:?}",actual_authors);
		assert_eq!(actual_authors.len(), 2);
	});
}

#[test]
fn test_pallet_xode_staking_unstaked() {
	test1_ext().execute_with(|| {
		System::set_block_number(0);

		XodeStaking::on_initialize(System::block_number());
		
		let desired_candidates = DesiredCandidates::<Test>::get();
		assert_eq!(desired_candidates.len(), 3, "There should be exactly three desired candidates");
	});
}

#[test]
fn test_pallet_xode_staking_offline() {
	test1_ext().execute_with(|| {
		System::set_block_number(0);

		XodeStaking::on_initialize(System::block_number());
		
		let desired_candidates = DesiredCandidates::<Test>::get();
		assert_eq!(desired_candidates.len(), 3, "There should be exactly three desired candidates");
	});
}

#[test]
fn test_pallet_xode_staking_leaving() {
	test1_ext().execute_with(|| {
		System::set_block_number(0);

		XodeStaking::on_initialize(System::block_number());
		
		let desired_candidates = DesiredCandidates::<Test>::get();
		assert_eq!(desired_candidates.len(), 3, "There should be exactly three desired candidates");
	});
}

#[test]
fn test_pallet_xode_staking_left() {
	test1_ext().execute_with(|| {
		System::set_block_number(0);

		XodeStaking::on_initialize(System::block_number());
		
		let desired_candidates = DesiredCandidates::<Test>::get();
		assert_eq!(desired_candidates.len(), 3, "There should be exactly three desired candidates");
	});
}