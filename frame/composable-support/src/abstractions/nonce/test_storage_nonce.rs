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
			nonce::{Nonce, StorageValue},
			utils::{
				increment::{IncrementToMax, SafeIncrement, WrappingIncrement},
				start_at::{DefaultInit, OneInit, ZeroInit},
			},
		},
		error_to_pallet_error,
		math::{safe::SafeAdd, wrapping_next::WrappingNext},
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type ZeroInit_WrappingIncrement: Copy + Zero + WrappingNext + TypeInfo + Member + FullCodec;
		type OneInit_WrappingIncrement: Copy + One + WrappingNext + TypeInfo + Member + FullCodec;
		type DefaultInit_WrappingIncrement: Copy
			+ Default
			+ WrappingNext
			+ TypeInfo
			+ Member
			+ FullCodec;

		type ZeroInit_SafeIncrement: Copy + Zero + SafeAdd + One + TypeInfo + Member + FullCodec;
		type OneInit_SafeIncrement: Copy + One + SafeAdd + One + TypeInfo + Member + FullCodec;
		type DefaultInit_SafeIncrement: Copy
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

	// WrappingIncrement

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_ZeroInit_WrappingIncrement<T: Config> = StorageValue<
		_,
		T::ZeroInit_WrappingIncrement,
		ValueQuery,
		Nonce<ZeroInit, WrappingIncrement>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_OneInit_WrappingIncrement<T: Config> = StorageValue<
		_,
		T::OneInit_WrappingIncrement,
		ValueQuery,
		Nonce<OneInit, WrappingIncrement>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_DefaultInit_WrappingIncrement<T: Config> = StorageValue<
		_,
		T::DefaultInit_WrappingIncrement,
		ValueQuery,
		Nonce<DefaultInit, WrappingIncrement>,
	>;

	// SafeIncrement

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_ZeroInit_SafeIncrement<T: Config> =
		StorageValue<_, T::ZeroInit_SafeIncrement, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_OneInit_SafeIncrement<T: Config> =
		StorageValue<_, T::OneInit_SafeIncrement, ValueQuery, Nonce<OneInit, SafeIncrement>>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_DefaultInit_SafeIncrement<T: Config> = StorageValue<
		_,
		T::DefaultInit_SafeIncrement,
		ValueQuery,
		Nonce<DefaultInit, SafeIncrement>,
	>;

	// IncrementToMax

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_ZeroInit_IncrementToMax<T: Config> = StorageValue<
		_,
		T::ZeroInit_IncrementToMax,
		ValueQuery,
		Nonce<
			ZeroInit,
			IncrementToMax<
				T::ZeroInit_IncrementToMax_MaximumValue,
				ZeroInit_IncrementToMax_ValueTooLarge,
				Error<T>,
			>,
		>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_OneInit_IncrementToMax<T: Config> = StorageValue<
		_,
		T::OneInit_IncrementToMax,
		ValueQuery,
		Nonce<
			OneInit,
			IncrementToMax<
				T::OneInit_IncrementToMax_MaximumValue,
				OneInit_IncrementToMax_ValueTooLarge,
				Error<T>,
			>,
		>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // nonce
	pub type Nonce_DefaultInit_IncrementToMax<T: Config> = StorageValue<
		_,
		T::DefaultInit_IncrementToMax,
		ValueQuery,
		Nonce<
			DefaultInit,
			IncrementToMax<
				T::DefaultInit_IncrementToMax_MaximumValue,
				DefaultInit_IncrementToMax_ValueTooLarge,
				Error<T>,
			>,
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
	type ZeroInit_WrappingIncrement = u8;
	type OneInit_WrappingIncrement = u8;
	type DefaultInit_WrappingIncrement = u8;

	type ZeroInit_SafeIncrement = u8;
	type OneInit_SafeIncrement = u8;
	type DefaultInit_SafeIncrement = u8;

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

mod safe_increment {
	use frame_support::assert_noop;
	use sp_runtime::{
		traits::{One, Zero},
		ArithmeticError,
	};

	use super::*;

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_OneInit_SafeIncrement::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..254 {
				pallet::Nonce_OneInit_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_OneInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_OneInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_ZeroInit_SafeIncrement::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..255 {
				pallet::Nonce_ZeroInit_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_ZeroInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_ZeroInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_DefaultInit_SafeIncrement::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..255 {
				pallet::Nonce_DefaultInit_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_DefaultInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_DefaultInit_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);
		})
	}
}

mod wrapping_increment {
	use super::*;
	use sp_runtime::traits::{One, Zero};

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			// probbaly an uncommon usecase

			let initial_value = pallet::Nonce_OneInit_WrappingIncrement::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..254 {
				pallet::Nonce_OneInit_WrappingIncrement::<Test>::increment();
			}

			// wrapping when starting at 1 has somewhat strange behaviour when the type has a
			// zero value
			assert!(pallet::Nonce_OneInit_WrappingIncrement::<Test>::increment().is_zero());

			// one more than previous loop as it wrapped to zero
			for _ in 0..255 {
				pallet::Nonce_OneInit_WrappingIncrement::<Test>::increment();
			}

			assert!(pallet::Nonce_OneInit_WrappingIncrement::<Test>::increment().is_zero());
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_ZeroInit_WrappingIncrement::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..255 {
				pallet::Nonce_ZeroInit_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_ZeroInit_WrappingIncrement::<Test>::increment(),
				initial_value
			);

			for _ in 0..255 {
				pallet::Nonce_ZeroInit_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_ZeroInit_WrappingIncrement::<Test>::increment(),
				initial_value
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_DefaultInit_WrappingIncrement::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..255 {
				pallet::Nonce_DefaultInit_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_DefaultInit_WrappingIncrement::<Test>::increment(),
				initial_value
			);

			for _ in 0..255 {
				pallet::Nonce_DefaultInit_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_DefaultInit_WrappingIncrement::<Test>::increment(),
				initial_value
			);
		})
	}
}

mod increment_to_max {
	use frame_support::assert_noop;
	use sp_runtime::traits::{One, Zero};

	use super::*;

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_OneInit_IncrementToMax::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..19 {
				pallet::Nonce_OneInit_IncrementToMax::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_OneInit_IncrementToMax::<Test>::increment(),
				pallet::Error::<Test>::OneInit_IncrementToMax_ValueTooLarge,
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_OneInit_IncrementToMax::<Test>::increment(),
				pallet::Error::<Test>::OneInit_IncrementToMax_ValueTooLarge,
			);
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_ZeroInit_IncrementToMax::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..20 {
				pallet::Nonce_ZeroInit_IncrementToMax::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_ZeroInit_IncrementToMax::<Test>::increment(),
				pallet::Error::<Test>::ZeroInit_IncrementToMax_ValueTooLarge
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_ZeroInit_IncrementToMax::<Test>::increment(),
				pallet::Error::<Test>::ZeroInit_IncrementToMax_ValueTooLarge
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_DefaultInit_IncrementToMax::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..20 {
				pallet::Nonce_DefaultInit_IncrementToMax::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_DefaultInit_IncrementToMax::<Test>::increment(),
				pallet::Error::<Test>::DefaultInit_IncrementToMax_ValueTooLarge
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_DefaultInit_IncrementToMax::<Test>::increment(),
				pallet::Error::<Test>::DefaultInit_IncrementToMax_ValueTooLarge
			);
		})
	}
}
