use super::*;
use crate as pallet_chainlink_feed;

use frame_support::weights::Weight;
use frame_support::{assert_noop, assert_ok, impl_outer_origin, parameter_types};
use sp_core::H256;

use frame_system as system;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
type System = frame_system::Module<Test>;

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Trait for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}
type Balances = pallet_balances::Module<Test>;

pub struct MockWeightInfo;
impl WeightInfo for MockWeightInfo {
	fn create_feed() -> Weight {
		42
	}
	fn submit() -> Weight {
		42
	}
}

parameter_types! {
	pub const StringLimit: u32 = 30;
	pub const OracleLimit: u32 = 10;
}

impl Trait for Test {
	type Currency = Balances;
	type Event = ();
	type Balance = u64;
	type FeedId = u32;
	type RoundId = u32;
	type Value = u64;
	type StringLimit = StringLimit;
	type OracleCountLimit = OracleLimit;
	type WeightInfo = MockWeightInfo;
}
type ChainlinkFeed = crate::Module<Test>;

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

#[test]
fn median_works() {
	let mut values = vec![4u32, 6, 2, 7];
	assert_eq!(median(&mut values), 5);
	let mut values = vec![4u32, 6, 2, 7, 9];
	assert_eq!(median(&mut values), 6);
}

#[test]
fn feed_creation_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(ChainlinkFeed::create_feed(
			Origin::signed(1),
			20,
			10,
			10,
			1_000,
			(3, 8),
			5,
			b"desc".to_vec(),
			2,
			vec![(1, 4), (2, 4), (3, 4)]
		));
	});
}

#[test]
fn submit_should_work() {
	new_test_ext().execute_with(|| {
		let payment_amount = 20;
		let timeout = 10;
		let submission_count_bounds = (2, 3);
		assert_ok!(ChainlinkFeed::create_feed(
			Origin::signed(1),
			payment_amount,
			timeout,
			10,
			1_000,
			submission_count_bounds,
			5,
			b"desc".to_vec(),
			0,
			vec![(1, 4), (2, 4), (3, 4)]
		));

		let submission = 42;
		assert_ok!(ChainlinkFeed::submit(Origin::signed(2), 0, 1, submission));
		let round = ChainlinkFeed::round(0, 1).expect("first round should be present");
		assert_eq!(
			round,
			Round {
				started_at: 0,
				..Default::default()
			}
		);
		let details =
			ChainlinkFeed::round_details(0, 1).expect("details for first round should be present");
		assert_eq!(
			details,
			RoundDetails {
				submissions: vec![submission],
				submission_count_bounds,
				payment_amount,
				timeout
			}
		);
	});
}
