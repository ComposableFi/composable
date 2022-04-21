#![doc = include_str!("../README.md")]

// TODO: mock, test, weights

pub use pallet::*;

pub mod models;

#[frame_support::pallet]
pub mod pallet {
	use std::fmt::Debug;

	use crate::models::{Airdrop, RecipientFund, RemoteAccount};
	use codec::{Codec, FullCodec, MaxEncodedLen};
	use composable_support::{math::safe::{SafeAdd, SafeSub}, types::EthereumAddress};
	use frame_support::{
		dispatch::PostDispatchInfo,
		pallet_prelude::{MaybeSerializeDeserialize, OptionQuery, ValueQuery, *},
		traits::{
			fungible::{Inspect, Transfer},
			Time,
		},
		transactional, Blake2_128Concat, PalletId, Parameter,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::keccak_256;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32Bit, AtLeast32BitUnsigned, CheckedAdd, CheckedMul,
			CheckedSub, Convert, One, Saturating, Verify, Zero,
		},
		AccountId32, DispatchErrorWithPostInfo, MultiSignature, Perbill,
	};
	use sp_std::vec::Vec;

	/// [`AirdropId`](Config::AirdropId) of the pallet `Config`.
	pub type AirdropIdOf<T> = <T as Config>::AirdropId;
	/// [`Airdrop`](crate::models::Airdrop) of the pallet `Config`.
	pub type AirdropOf<T> = Airdrop<
		<T as frame_system::Config>::AccountId,
		<T as Config>::Balance,
		<T as Config>::Moment,
	>;
	/// [`RecipientFund`](crate::models::RecipientFund) of the pallet `Config`.
	pub type RecipientFundOf<T> = RecipientFund<<T as Config>::Balance, <T as Config>::Moment>;
	/// [`Moment`](Config::Moment) of the pallet `Config`.
	pub type MomentOf<T> = <T as Config>::Moment;
	pub type RemoteAccountOf<T> = RemoteAccount<<T as Config>::RelayChainAccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AirdropCreated {
			airdrop_id: T::AirdropId,
		},
		AirdropStarted {
			airdrop_id: T::AirdropId,
			at: T::Moment,
		},
		AirdropEnded {
			airdrop_id: T::AirdropId,
			at: T::Moment,
		},
		Claimed {
			remote_account: RemoteAccountOf<T>,
			recipient_account: T::AccountId,
			amount: T::Balance,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		// TODO: errors
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Airdrop ID
		type AirdropId: Copy
			+ Clone
			+ Eq
			+ Debug
			+ Zero
			+ One
			+ SafeAdd
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo;

		/// Representation of some amount of tokens
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ Zero;

		/// Time stamp
		type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen + FullCodec;

		/// Relay chain account ID
		type RelayChainAccountId: Parameter
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ Into<AccountId32>
			+ Ord;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Starting value for [`AirdropId`](Config::AirdropId).
	#[pallet::type_value]
	pub fn AirdropOnEmpty<T: Config>() -> T::AirdropId {
		T::AirdropId::zero()
	}

	/// The counter used to identify Airdrops.
	#[pallet::storage]
	#[pallet::getter(fn airdrop_count)]
	pub type AirdropCount<T: Config> = StorageValue<_, T::AirdropId, ValueQuery, AirdropOnEmpty<T>>;

	/// Airdrops currently stored by the pallet.
	#[pallet::storage]
	#[pallet::getter(fn airdrops)]
	pub type Airdrops<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AirdropId, AirdropOf<T>, OptionQuery>;

	/// Associations of local accounts and an [`AirdropId`](Config::AirdropId) to a remote account.
	#[pallet::storage]
	#[pallet::getter(fn associations)]
	pub type Associations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AirdropId, T::AccountId),
		RemoteAccountOf<T>,
		OptionQuery,
	>;

	/// Recipient funds of Airdrops stored by the pallet.
	#[pallet::storage]
	#[pallet::getter(fn recipient_funds)]
	pub type RecipientFunds<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AirdropId, RemoteAccountOf<T>),
		RecipientFundOf<T>,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new airdrop. This requires that the user puts down a stake in PICA.
		///
		/// Can be called by any signed origin.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_airdrop(origin: OriginFor<T>) -> DispatchResult {
			Ok(().into())
		}

		/// Add one or more recipients to the airdrop, specifying the token amount that each
		/// provided adress will receive
		///
		/// Only callable by the origin that created the airdrop.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn add_recipient(origin: OriginFor<T>) -> DispatchResult {
			Ok(().into())
		}

		/// Remove a recipient from an airdrop
		///
		/// Only callable by the origin that created the airdrop.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn remove_recipient(origin: OriginFor<T>) -> DispatchResult {
			Ok(().into())
		}

		/// Start an airdrop.
		///
		/// Only callable by the origin that created the airdrop.
		///
		/// # Errors
		/// * If the airdrop has been configured to start after a certain timestamp
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn enable(origin: OriginFor<T>) -> DispatchResult {
			Ok(().into())
		}

		/// Stop an airdrop
		///
		/// Only callable by the origin that created the airdrop.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn disable(origin: OriginFor<T>) -> DispatchResult {
			Ok(().into())
		}

		/// Claim recipient funds from an airdrop.
		///
		/// If no more funds are left to claim, the airdrop will be removed.
		///
		/// Callable by any origin.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			Ok(().into())
		}
	}
}
