use frame_support::traits::{ConstU16, ConstU32, ConstU64, ConstU8, Everything};
use sp_runtime::{
	testing::{Header, H256},
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};

use super::*;

#[frame_support::pallet]
pub mod pallet {
	#![allow(non_camel_case_types)] // makes it easier to read the long type names

	use codec::FullCodec;
	use frame_support::{pallet_prelude::*, traits::Get};
	use sp_runtime::traits::{One, Zero};
	use sp_std::ops::Add;

	use crate::{
		abstractions::{
			counter::{Counter, StorageValue},
			utils::{
				decrement::SafeDecrement,
				increment::{IncrementToMax, SafeIncrement},
				start_at::{DefaultInit, OneInit, ZeroInit},
			},
		},
		error_to_pallet_error,
		math::safe::SafeAdd,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type ZeroInit_SafeIncrement: Copy + Zero + SafeAdd + One + TypeInfo + Member + FullCodec;
		type OneInit_SafeIncrement: Copy + One + SafeAdd + One + TypeInfo + Member + FullCodec;
		type DefaultInit_SafeIncrement: Copy
			+ Default
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec;

		type ZeroInit_SafeDecrement: Copy + Zero + SafeAdd + One + TypeInfo + Member + FullCodec;
		type OneInit_SafeDecrement: Copy + One + SafeAdd + One + TypeInfo + Member + FullCodec;
		type DefaultInit_SafeDecrement: Copy
			+ Default
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec;

		type ZeroInit_IncrementToMax: Copy
			+ Zero
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec
			+ PartialOrd
			+ Add<Output = Self::ZeroInit_IncrementToMax>
			+ 'static;
		type OneInit_IncrementToMax: Copy
			+ One
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec
			+ PartialOrd
			+ Add<Output = Self::OneInit_IncrementToMax>
			+ 'static;
		type DefaultInit_IncrementToMax: Copy
			+ Default
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec
			+ PartialOrd
			+ Add<Output = Self::DefaultInit_IncrementToMax>
			+ 'static;

		#[pallet::constant]
		type ZeroInit_IncrementToMax_MaximumValue: Get<Self::ZeroInit_IncrementToMax>;
		#[pallet::constant]
		type OneInit_IncrementToMax_MaximumValue: Get<Self::OneInit_IncrementToMax>;
		#[pallet::constant]
		type DefaultInit_IncrementToMax_MaximumValue: Get<Self::DefaultInit_IncrementToMax>;
	}

	#[pallet::error]
	#[derive(PartialEqNoBound)]
	pub enum Error<T> {
		ZeroInit_IncrementToMax_ValueTooLarge,
		OneInit_IncrementToMax_ValueTooLarge,
		DefaultInit_IncrementToMax_ValueTooLarge,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// SafeIncrement

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // counter
	pub type Counter_ZeroInit_SafeIncrement<T: Config> = StorageValue<
		_,
		T::ZeroInit_SafeIncrement,
		ValueQuery,
		Counter<ZeroInit, SafeIncrement, SafeDecrement>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // counter
	pub type Counter_OneInit_SafeIncrement<T: Config> = StorageValue<
		_,
		T::OneInit_SafeIncrement,
		ValueQuery,
		Counter<OneInit, SafeIncrement, SafeDecrement>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // counter
	pub type Counter_DefaultInit_SafeIncrement<T: Config> = StorageValue<
		_,
		T::DefaultInit_SafeIncrement,
		ValueQuery,
		Counter<DefaultInit, SafeIncrement, SafeDecrement>,
	>;

	// IncrementToMax

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // counter
	pub type Counter_ZeroInit_ToMax<T: Config> = StorageValue<
		_,
		T::ZeroInit_IncrementToMax,
		ValueQuery,
		Counter<
			ZeroInit,
			IncrementToMax<
				T::ZeroInit_IncrementToMax_MaximumValue,
				ZeroInit_IncrementToMax_ValueTooLarge,
				Error<T>,
			>,
			SafeDecrement,
		>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // counter
	pub type Counter_OneInit_ToMax<T: Config> = StorageValue<
		_,
		T::OneInit_IncrementToMax,
		ValueQuery,
		Counter<
			OneInit,
			IncrementToMax<
				T::OneInit_IncrementToMax_MaximumValue,
				OneInit_IncrementToMax_ValueTooLarge,
				Error<T>,
			>,
			SafeDecrement,
		>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // counter
	pub type Counter_DefaultInit_ToMax<T: Config> = StorageValue<
		_,
		T::DefaultInit_IncrementToMax,
		ValueQuery,
		Counter<
			DefaultInit,
			IncrementToMax<
				T::DefaultInit_IncrementToMax_MaximumValue,
				DefaultInit_IncrementToMax_ValueTooLarge,
				Error<T>,
			>,
			SafeDecrement,
		>,
	>;

	error_to_pallet_error!(
		ZeroInit_IncrementToMax_ValueTooLarge,
		OneInit_IncrementToMax_ValueTooLarge,
		DefaultInit_IncrementToMax_ValueTooLarge,
	);
}

type MockUncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type MockBlock = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = MockBlock,
		NodeBlock = MockBlock,
		UncheckedExtrinsic = MockUncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TestPallet: pallet::{Pallet, Call, Storage},
	}
);

// mostly copied from dutch-auction
impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = u32;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet::Config for Test {
	type ZeroInit_SafeIncrement = u8;
	type OneInit_SafeIncrement = u8;
	type DefaultInit_SafeIncrement = u8;

	type ZeroInit_SafeDecrement = u8;
	type OneInit_SafeDecrement = u8;
	type DefaultInit_SafeDecrement = u8;

	type ZeroInit_IncrementToMax = u8;
	type OneInit_IncrementToMax = u8;
	type DefaultInit_IncrementToMax = u8;

	type ZeroInit_IncrementToMax_MaximumValue = ConstU8<20>;
	type OneInit_IncrementToMax_MaximumValue = ConstU8<20>;
	type DefaultInit_IncrementToMax_MaximumValue = ConstU8<20>;
}

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

mod safe {
	use frame_support::assert_noop;
	use sp_runtime::{
		traits::{One, Zero},
		ArithmeticError,
	};

	use super::*;

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Counter_OneInit_SafeIncrement::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..254 {
				pallet::Counter_OneInit_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Counter_OneInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// go down one further than the above loop since we started at 1
			for _ in 0..255 {
				pallet::Counter_OneInit_SafeIncrement::<Test>::decrement().unwrap();
			}

			assert_noop!(
				pallet::Counter_OneInit_SafeIncrement::<Test>::decrement(),
				ArithmeticError::Underflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_OneInit_SafeIncrement::<Test>::decrement(),
				ArithmeticError::Underflow
			);
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Counter_ZeroInit_SafeIncrement::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..255 {
				pallet::Counter_ZeroInit_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Counter_ZeroInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			for _ in 0..255 {
				pallet::Counter_ZeroInit_SafeIncrement::<Test>::decrement().unwrap();
			}

			assert_noop!(
				pallet::Counter_ZeroInit_SafeIncrement::<Test>::decrement(),
				ArithmeticError::Underflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_ZeroInit_SafeIncrement::<Test>::decrement(),
				ArithmeticError::Underflow
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Counter_DefaultInit_SafeIncrement::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..255 {
				pallet::Counter_DefaultInit_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Counter_DefaultInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			for _ in 0..255 {
				pallet::Counter_DefaultInit_SafeIncrement::<Test>::decrement().unwrap();
			}

			assert_noop!(
				pallet::Counter_DefaultInit_SafeIncrement::<Test>::decrement(),
				ArithmeticError::Underflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_DefaultInit_SafeIncrement::<Test>::decrement(),
				ArithmeticError::Underflow
			);
		})
	}
}

mod increment_to_max {
	use frame_support::assert_noop;
	use sp_runtime::{
		traits::{One, Zero},
		ArithmeticError,
	};

	use super::*;

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Counter_OneInit_ToMax::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..19 {
				pallet::Counter_OneInit_ToMax::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Counter_OneInit_ToMax::<Test>::increment(),
				pallet::Error::<Test>::OneInit_IncrementToMax_ValueTooLarge,
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_OneInit_ToMax::<Test>::increment(),
				pallet::Error::<Test>::OneInit_IncrementToMax_ValueTooLarge,
			);

			for _ in 0..20 {
				pallet::Counter_OneInit_ToMax::<Test>::decrement().unwrap();
			}

			assert_noop!(
				pallet::Counter_OneInit_ToMax::<Test>::decrement(),
				ArithmeticError::Underflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_OneInit_ToMax::<Test>::decrement(),
				ArithmeticError::Underflow
			);
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Counter_ZeroInit_ToMax::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..20 {
				pallet::Counter_ZeroInit_ToMax::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Counter_ZeroInit_ToMax::<Test>::increment(),
				pallet::Error::<Test>::ZeroInit_IncrementToMax_ValueTooLarge
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_ZeroInit_ToMax::<Test>::increment(),
				pallet::Error::<Test>::ZeroInit_IncrementToMax_ValueTooLarge
			);

			for _ in 0..20 {
				pallet::Counter_ZeroInit_ToMax::<Test>::decrement().unwrap();
			}

			assert_noop!(
				pallet::Counter_ZeroInit_ToMax::<Test>::decrement(),
				ArithmeticError::Underflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_ZeroInit_ToMax::<Test>::decrement(),
				ArithmeticError::Underflow
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Counter_DefaultInit_ToMax::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..20 {
				pallet::Counter_DefaultInit_ToMax::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Counter_DefaultInit_ToMax::<Test>::increment(),
				pallet::Error::<Test>::DefaultInit_IncrementToMax_ValueTooLarge
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_DefaultInit_ToMax::<Test>::increment(),
				pallet::Error::<Test>::DefaultInit_IncrementToMax_ValueTooLarge
			);

			for _ in 0..20 {
				pallet::Counter_DefaultInit_ToMax::<Test>::decrement().unwrap();
			}

			assert_noop!(
				pallet::Counter_DefaultInit_ToMax::<Test>::decrement(),
				ArithmeticError::Underflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Counter_DefaultInit_ToMax::<Test>::decrement(),
				ArithmeticError::Underflow
			);
		})
	}
}
