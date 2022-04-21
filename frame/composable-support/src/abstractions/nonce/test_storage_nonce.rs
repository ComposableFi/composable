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
	use frame_support::{
		pallet_prelude::*,
		traits::{Get, IsType},
	};
	use sp_runtime::traits::{One, Zero};
	use sp_std::ops::Add;

	use crate::{
		abstractions::{
			nonce::*,
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
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type ZeroStart_WrappingIncrement: Copy + Zero + WrappingNext + TypeInfo + Member + FullCodec;
		type OneStart_WrappingIncrement: Copy + One + WrappingNext + TypeInfo + Member + FullCodec;
		type DefaultStart_WrappingIncrement: Copy
			+ Default
			+ WrappingNext
			+ TypeInfo
			+ Member
			+ FullCodec;

		type ZeroStart_SafeIncrement: Copy + Zero + SafeAdd + One + TypeInfo + Member + FullCodec;
		type OneStart_SafeIncrement: Copy + One + SafeAdd + One + TypeInfo + Member + FullCodec;
		type DefaultStart_SafeIncrement: Copy
			+ Default
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec;

		type ZeroStart_IncrementToMax: Copy
			+ Zero
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec
			+ PartialOrd
			+ Add<Output = Self::ZeroStart_IncrementToMax>
			+ 'static;
		type OneStart_IncrementToMax: Copy
			+ One
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec
			+ PartialOrd
			+ Add<Output = Self::OneStart_IncrementToMax>
			+ 'static;
		type DefaultStart_IncrementToMax: Copy
			+ Default
			+ SafeAdd
			+ One
			+ TypeInfo
			+ Member
			+ FullCodec
			+ PartialOrd
			+ Add<Output = Self::DefaultStart_IncrementToMax>
			+ 'static;

		#[pallet::constant]
		type ZeroStart_IncrementToMax_MaximumValue: Get<Self::ZeroStart_IncrementToMax>;
		#[pallet::constant]
		type OneStart_IncrementToMax_MaximumValue: Get<Self::OneStart_IncrementToMax>;
		#[pallet::constant]
		type DefaultStart_IncrementToMax_MaximumValue: Get<Self::DefaultStart_IncrementToMax>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		ZeroStart_IncrementToMax_ValueTooLarge,
		OneStart_IncrementToMax_ValueTooLarge,
		DefaultStart_IncrementToMax_ValueTooLarge,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// WrappingIncrement

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_ZeroStart_WrappingIncrement<T: Config> = StorageValue<
		_,
		T::ZeroStart_WrappingIncrement,
		ValueQuery,
		Nonce<ZeroInit, WrappingIncrement>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_OneStart_WrappingIncrement<T: Config> = StorageValue<
		_,
		T::OneStart_WrappingIncrement,
		ValueQuery,
		Nonce<OneInit, WrappingIncrement>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_DefaultStart_WrappingIncrement<T: Config> = StorageValue<
		_,
		T::DefaultStart_WrappingIncrement,
		ValueQuery,
		Nonce<DefaultInit, WrappingIncrement>,
	>;

	// SafeIncrement

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_ZeroStart_SafeIncrement<T: Config> =
		StorageValue<_, T::ZeroStart_SafeIncrement, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_OneStart_SafeIncrement<T: Config> =
		StorageValue<_, T::OneStart_SafeIncrement, ValueQuery, Nonce<OneInit, SafeIncrement>>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_DefaultStart_SafeIncrement<T: Config> = StorageValue<
		_,
		T::DefaultStart_SafeIncrement,
		ValueQuery,
		Nonce<DefaultInit, SafeIncrement>,
	>;

	// IncrementToMax

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_ZeroStart_IncrementToMax<T: Config> = StorageValue<
		_,
		T::ZeroStart_IncrementToMax,
		ValueQuery,
		Nonce<
			ZeroInit,
			IncrementToMax<
				T::ZeroStart_IncrementToMax_MaximumValue,
				ZeroStart_IncrementToMax_Error,
				Error<T>,
			>,
		>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_OneStart_IncrementToMax<T: Config> = StorageValue<
		_,
		T::OneStart_IncrementToMax,
		ValueQuery,
		Nonce<
			OneInit,
			IncrementToMax<
				T::OneStart_IncrementToMax_MaximumValue,
				OneStart_IncrementToMax_Error,
				Error<T>,
			>,
		>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_type)] // nonce
	pub type Nonce_DefaultStart_IncrementToMax<T: Config> = StorageValue<
		_,
		T::DefaultStart_IncrementToMax,
		ValueQuery,
		Nonce<
			DefaultInit,
			IncrementToMax<
				T::DefaultStart_IncrementToMax_MaximumValue,
				DefaultStart_IncrementToMax_Error,
				Error<T>,
			>,
		>,
	>;

	error_to_pallet_error!(
		ZeroStart_IncrementToMax_Error -> ZeroStart_IncrementToMax_ValueTooLarge;
		OneStart_IncrementToMax_Error -> OneStart_IncrementToMax_ValueTooLarge;
		DefaultStart_IncrementToMax_Error -> DefaultStart_IncrementToMax_ValueTooLarge;
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
		TestPallet: pallet::{Pallet, Storage, Event<T>},
	}
);

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
	type Event = Event;

	type ZeroStart_WrappingIncrement = u8;
	type OneStart_WrappingIncrement = u8;
	type DefaultStart_WrappingIncrement = u8;

	type ZeroStart_SafeIncrement = u8;
	type OneStart_SafeIncrement = u8;
	type DefaultStart_SafeIncrement = u8;

	type ZeroStart_IncrementToMax = u8;
	type OneStart_IncrementToMax = u8;
	type DefaultStart_IncrementToMax = u8;

	type ZeroStart_IncrementToMax_MaximumValue = ConstU8<20>;
	type OneStart_IncrementToMax_MaximumValue = ConstU8<20>;
	type DefaultStart_IncrementToMax_MaximumValue = ConstU8<20>;
}

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

mod safe_increment {
	use frame_support::assert_noop;
	use sp_runtime::ArithmeticError;

	use super::*;

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_OneStart_SafeIncrement::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..254 {
				pallet::Nonce_OneStart_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_OneStart_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_OneStart_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_ZeroStart_SafeIncrement::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..255 {
				pallet::Nonce_ZeroStart_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_ZeroStart_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_ZeroStart_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_DefaultStart_SafeIncrement::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..255 {
				pallet::Nonce_DefaultStart_SafeIncrement::<Test>::increment().unwrap();
			}

			assert_noop!(
				pallet::Nonce_DefaultStart_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);

			// once more for good measure
			assert_noop!(
				pallet::Nonce_DefaultStart_SafeIncrement::<Test>::increment(),
				ArithmeticError::Overflow
			);
		})
	}
}

mod wrapping_increment {
	use super::*;

	#[test]
	fn one_start() {
		ExtBuilder::default().build().execute_with(|| {
			// probbaly an uncommon usecase

			let initial_value = pallet::Nonce_OneStart_WrappingIncrement::<Test>::get();
			assert!(initial_value.is_one(), "initial value should be one");

			for _ in 0..254 {
				pallet::Nonce_OneStart_WrappingIncrement::<Test>::increment();
			}

			// wrapping when starting at 1 has somewhat strange behaviour when the type has a
			// zero value
			assert!(pallet::Nonce_OneStart_WrappingIncrement::<Test>::increment().is_zero());

			// one more than previous loop as it wrapped to zero
			for _ in 0..255 {
				pallet::Nonce_OneStart_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(pallet::Nonce_OneStart_WrappingIncrement::<Test>::increment(), u8::zero());
		})
	}

	#[test]
	fn zero_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_ZeroStart_WrappingIncrement::<Test>::get();
			assert!(initial_value.is_zero(), "initial value should be zero");

			for _ in 0..255 {
				pallet::Nonce_ZeroStart_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_ZeroStart_WrappingIncrement::<Test>::increment(),
				initial_value
			);

			for _ in 0..255 {
				pallet::Nonce_ZeroStart_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_ZeroStart_WrappingIncrement::<Test>::increment(),
				initial_value
			);
		})
	}

	#[test]
	fn default_start() {
		ExtBuilder::default().build().execute_with(|| {
			let initial_value = pallet::Nonce_DefaultStart_WrappingIncrement::<Test>::get();
			assert_eq!(initial_value, u8::default(), "initial value should be the default");

			// default is the same as zero in the case of u8
			for _ in 0..255 {
				pallet::Nonce_DefaultStart_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_DefaultStart_WrappingIncrement::<Test>::increment(),
				initial_value
			);

			for _ in 0..255 {
				pallet::Nonce_DefaultStart_WrappingIncrement::<Test>::increment();
			}

			assert_eq!(
				pallet::Nonce_DefaultStart_WrappingIncrement::<Test>::increment(),
				initial_value
			);
		})
	}
}
