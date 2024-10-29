// This module provides a mock runtime specific for testing the pallet's functionality.
#![cfg(test)]

use frame_support::{parameter_types, traits::{ConstU32, Everything}};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};

pub type AccountId = u64;
pub type Balance = u128;

frame_support::construct_runtime! {
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Config<T>, Call, Storage, Event<T>},
        TemplatePallet: pallet_template::{Pallet, Call, Storage, Event<T>},
    }
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type Origin = Origin;
    type Call = Call;
    type Index = u32;
    type BlockNumber = u32;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = ConstU32<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
}

parameter_types! {
    pub const BlockWeights: frame_support::weights::Weight = frame_support::weights::Weight::from_ref_time(1024);
    pub const BlockLength: frame_support::limits::BlockLength = frame_support::limits::BlockLength::max_with_normal_ratio(1024, 1);
}

impl pallet_template::Config for TestRuntime {
    type RuntimeEvent = Event;
    type WeightInfo = ();
}

pub type Block = frame_system::Block<TestRuntime>;
pub type UncheckedExtrinsic = frame_system::UncheckedExtrinsic<TestRuntime>;
