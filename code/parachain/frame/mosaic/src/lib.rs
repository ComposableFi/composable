#![doc = include_str!("../README.md")]
// TODO
// 1. TEST!
// 2. RPCs for relayer convenience.
// 3. Refactor core logic to traits.
// 4. Benchmarks and Weights!
#![cfg_attr(not(feature = "std"), no_std)]

mod decay;
mod relayer;
mod validation;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
pub mod weights;

pub use crate::weights::WeightInfo;

pub use decay::{BudgetPenaltyDecayer, Decayer};
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[allow(clippy::too_many_arguments)]
#[frame_support::pallet]
pub mod pallet {
	use crate::{
		decay::Decayer,
		relayer::{RelayerConfig, StaleRelayer},
		validation::{ValidTTL, ValidTimeLockPeriod},
		weights::WeightInfo,
	};
	use codec::FullCodec;
	use composable_support::{math::safe::SafeAdd, types::EthereumAddress, validation::Validated};
	use composable_traits::mosaic::{Claim, RelayerInterface, TransferTo};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedSub, Zero};
	use scale_info::TypeInfo;
	use sp_core::H256;
	use sp_runtime::{
		traits::{AccountIdConversion, Keccak256, Saturating},
		DispatchError,
	};
	use sp_std::{fmt::Debug, str};

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type BalanceOf<T> = <<T as Config>::Assets as Inspect<AccountIdOf<T>>>::Balance;
	pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub(crate) type AssetIdOf<T> = <<T as Config>::Assets as Inspect<AccountIdOf<T>>>::AssetId;
	pub(crate) type NetworkIdOf<T> = <T as Config>::NetworkId;
	pub(crate) type RemoteAssetIdOf<T> = <T as Config>::RemoteAssetId;
	pub(crate) type RemoteAmmIdOf<T> = <T as Config>::RemoteAmmId;
	pub(crate) type AmmMinimumAmountOutOf<T> = <T as Config>::AmmMinimumAmountOut;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type PalletId: Get<PalletId>;

		type Assets: Mutate<AccountIdOf<Self>> + Transfer<AccountIdOf<Self>>;

		/// The minimum time to live before a relayer account rotation.
		#[pallet::constant]
		type MinimumTTL: Get<BlockNumberOf<Self>>;

		/// The minimum period for which we lock outgoing/incoming funds.
		#[pallet::constant]
		type MinimumTimeLockPeriod: Get<BlockNumberOf<Self>>;

		/// The budget penalty decayer.
		type BudgetPenaltyDecayer: Decayer<BalanceOf<Self>, BlockNumberOf<Self>>
			+ Clone
			+ Encode
			+ Decode
			+ MaxEncodedLen
			+ Debug
			+ TypeInfo
			+ PartialEq;

		/// A type representing a network ID.
		type NetworkId: FullCodec + MaxEncodedLen + TypeInfo + Clone + Debug + PartialEq;

		/// A type representing a remote asset ID.
		type RemoteAssetId: FullCodec + MaxEncodedLen + TypeInfo + Clone + Debug + PartialEq;

		/// A type representing a remote AMM ID.
		type RemoteAmmId: FullCodec + MaxEncodedLen + TypeInfo + Clone + Debug + PartialEq;

		// A type representing the type of the minimum amount out after a AMM Swap.
		type AmmMinimumAmountOut: IsType<u128>
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo
			+ Clone
			+ Debug
			+ PartialEq;

		/// Origin capable of setting the relayer and AMM IDs. Intended to be RootOrHalfCouncil, as
		/// it is also used as the origin capable of stopping attackers.
		type ControlOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight implementation used for extrinsics.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Convenience identifiers emitted by the pallet for relayer bookkeeping.
	pub type Id = H256;

	/// Transaction type.
	pub enum TransactionType {
		Incoming,
		Outgoing,
	}

	#[derive(Clone, Encode, Decode, Debug, MaxEncodedLen, TypeInfo, PartialEq, Eq)]
	pub struct AmmSwapInfo<N, R, M> {
		pub destination_token_out_address: EthereumAddress,
		pub destination_amm: RemoteAmm<N, R>,
		pub minimum_amount_out: M,
	}

	#[derive(Clone, Encode, Decode, Debug, MaxEncodedLen, TypeInfo, PartialEq, Eq)]
	pub struct RemoteAmm<N, R> {
		pub network_id: N,
		pub amm_id: R,
	}

	/// The information required for an assets to be transferred between chains.
	#[derive(Clone, Debug, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq)]
	pub struct AssetInfo<BlockNumber, Balance, Decayer> {
		pub last_mint_block: BlockNumber,
		pub budget: Balance,
		pub penalty: Balance,
		pub penalty_decayer: Decayer,
	}

	/// The network information, used for rate limiting.
	#[derive(Clone, Debug, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq)]
	pub struct NetworkInfo<Balance> {
		pub enabled: bool,
		pub min_transfer_size: Balance,
		pub max_transfer_size: Balance,
	}

	/// User incoming/outgoing accounts, that hold the funds for transactions to happen.
	pub struct SubAccount<T: Config> {
		transaction_type: TransactionType,
		account_id: AccountIdOf<T>,
	}

	impl<T: Config> SubAccount<T> {
		pub fn to_id(&self) -> impl Encode {
			let prefix = match self.transaction_type {
				TransactionType::Incoming => b"incoming________",
				TransactionType::Outgoing => b"outgoing________",
			};
			[prefix.to_vec(), self.account_id.encode()]
		}
		pub fn new_outgoing(account_id: AccountIdOf<T>) -> Self {
			SubAccount { transaction_type: TransactionType::Outgoing, account_id }
		}
		pub fn new_incoming(account_id: AccountIdOf<T>) -> Self {
			SubAccount { transaction_type: TransactionType::Incoming, account_id }
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn relayer)]
	pub type Relayer<T: Config> =
		StorageValue<_, StaleRelayer<T::AccountId, T::BlockNumber>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn asset_infos)]
	pub type AssetsInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		AssetInfo<BlockNumberFor<T>, BalanceOf<T>, T::BudgetPenaltyDecayer>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn network_infos)]
	pub type NetworkInfos<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkIdOf<T>, NetworkInfo<BalanceOf<T>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn time_lock_period)]
	#[allow(clippy::disallowed_types)]
	pub type TimeLockPeriod<T: Config> =
		StorageValue<_, BlockNumberOf<T>, ValueQuery, TimeLockPeriodOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	#[allow(clippy::disallowed_types)]
	pub type Nonce<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Remote AMM IDs that exist (NetworkId, AmmId).
	/// Note that this is actually a set that does bookkeeping of valid AmmIds.
	/// Therefore, the value type is (), because it is irrelevant for our use case.
	#[pallet::storage]
	#[pallet::getter(fn amm_ids)]
	pub type RemoteAmmWhitelist<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		NetworkIdOf<T>,
		Blake2_128Concat,
		RemoteAmmIdOf<T>,
		(),
		OptionQuery,
	>;

	#[pallet::type_value]
	pub fn TimeLockPeriodOnEmpty<T: Config>() -> BlockNumberOf<T> {
		T::MinimumTimeLockPeriod::get()
	}

	/// Locked outgoing tx out of Picasso, that a relayer needs to process.
	#[pallet::storage]
	#[pallet::getter(fn outgoing_transactions)]
	pub type OutgoingTransactions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		AssetIdOf<T>,
		(BalanceOf<T>, BlockNumberFor<T>),
		OptionQuery,
	>;

	/// Locked incoming tx into Picasso that the user needs to claim.
	#[pallet::storage]
	#[pallet::getter(fn incoming_transactions)]
	pub type IncomingTransactions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		AssetIdOf<T>,
		(BalanceOf<T>, BlockNumberFor<T>),
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn local_to_remote_asset)]
	pub type LocalToRemoteAsset<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		NetworkIdOf<T>,
		RemoteAssetIdOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn remote_to_local_asset)]
	pub type RemoteToLocalAsset<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		RemoteAssetIdOf<T>,
		Blake2_128Concat,
		NetworkIdOf<T>,
		AssetIdOf<T>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The account of the relayer has been set.
		RelayerSet { relayer: AccountIdOf<T> },
		/// The relayer has been rotated to `account_id`.
		RelayerRotated { ttl: BlockNumberOf<T>, account_id: AccountIdOf<T> },
		BudgetUpdated {
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
			decay: T::BudgetPenaltyDecayer,
		},
		/// The `NetworkInfos` `network_info` was updated for `network_id`.
		NetworksUpdated { network_id: NetworkIdOf<T>, network_info: NetworkInfo<BalanceOf<T>> },
		/// An outgoing tx is created, and locked in the outgoing tx pool.
		TransferOut {
			id: Id,
			to: EthereumAddress,
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			amount: BalanceOf<T>,
			swap_to_native: bool,
			source_user_account: AccountIdOf<T>,
			amm_swap_info:
				Option<AmmSwapInfo<NetworkIdOf<T>, RemoteAmmIdOf<T>, AmmMinimumAmountOutOf<T>>>,
			minimum_amount_out: BalanceOf<T>,
		},
		/// User claimed outgoing tx that was not (yet) picked up by the relayer
		StaleTxClaimed {
			to: AccountIdOf<T>,
			by: AccountIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		/// An incoming tx is created and waiting for the user to claim.
		TransferInto {
			id: Id,
			to: AccountIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		/// When we have finality issues occur on the Ethereum chain,
		/// we burn the locked `IncomingTransaction` for which we know that it is invalid.
		TransferIntoRescined {
			account: AccountIdOf<T>,
			amount: BalanceOf<T>,
			asset_id: AssetIdOf<T>,
		},
		/// The relayer partially accepted the user's `OutgoingTransaction`.
		PartialTransferAccepted {
			from: AccountIdOf<T>,
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		/// The relayer accepted the user's `OutgoingTransaction`.
		TransferAccepted {
			from: AccountIdOf<T>,
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		/// The user claims his `IncomingTransaction` and unlocks the locked amount.
		TransferClaimed {
			by: AccountIdOf<T>,
			to: AccountIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		/// An asset mapping has been created.
		AssetMappingCreated {
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
		},
		/// An existing asset mapping has been updated.
		AssetMappingUpdated {
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
		},
		/// An existing asset mapping has been deleted.
		AssetMappingDeleted {
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		RelayerNotSet,
		BadTTL,
		BadTimelockPeriod,
		UnsupportedAsset,
		NetworkDisabled,
		UnsupportedNetwork,
		Overflow,
		NoStaleTransactions,
		InsufficientBudget,
		ExceedsMaxTransferSize,
		BelowMinTransferSize,
		NoClaimableTx,
		TxStillLocked,
		NoOutgoingTx,
		AmountMismatch,
		AssetNotMapped,
		RemoteAmmIdNotFound,
		RemoteAmmIdAlreadyExists,
		DestinationAmmIdNotWhitelisted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the current Relayer configuration.
		///
		/// This is enacted immediately and invalidates inflight/ incoming transactions from the
		/// previous Relayer. However, existing budgets remain in place.
		///
		/// This can only be called by the [`ControlOrigin`].
		///
		/// [`ControlOrigin`]: https://dali.devnets.composablefinance.ninja/doc/pallet_mosaic/pallet/trait.Config.html#associatedtype.ControlOrigin
		#[pallet::weight(T::WeightInfo::set_relayer())]
		#[transactional]
		pub fn set_relayer(
			origin: OriginFor<T>,
			relayer: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::ControlOrigin::ensure_origin(origin)?;

			<Pallet<T> as RelayerInterface>::set_relayer(relayer.clone());

			Self::deposit_event(Event::RelayerSet { relayer });
			Ok(().into())
		}

		/// Rotates the Relayer Account
		///
		/// # Restrictions
		///  - Only callable by the current Relayer.
		///  - The Time To Live (TTL) must be greater than the [`MinimumTTL`](Config::MinimumTTL)
		#[pallet::weight(T::WeightInfo::rotate_relayer())]
		#[transactional]
		pub fn rotate_relayer(
			origin: OriginFor<T>,
			new: T::AccountId,
			validated_ttl: Validated<T::BlockNumber, ValidTTL<T::MinimumTTL>>,
		) -> DispatchResultWithPostInfo {
			let ttl = validated_ttl.value();
			let (relayer, current_block) = Self::ensure_relayer(origin)?;

			<Pallet<T> as RelayerInterface>::rotate_relayer(
				relayer,
				new.clone(),
				ttl,
				current_block,
			);

			Self::deposit_event(Event::RelayerRotated { account_id: new, ttl });
			Ok(().into())
		}

		/// Sets supported networks and maximum transaction sizes accepted by the Relayer.
		///
		/// Only callable by the current Relayer
		#[pallet::weight(T::WeightInfo::set_network())]
		#[transactional]
		pub fn set_network(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			network_info: NetworkInfo<BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_relayer(origin)?;

			<Pallet<T> as RelayerInterface>::set_network(network_id.clone(), network_info.clone());

			Self::deposit_event(Event::NetworksUpdated { network_id, network_info });
			Ok(().into())
		}

		/// Sets the relayer budget for _incoming_ transactions for specific assets. Does not reset
		/// the current `penalty`.
		///
		/// # Restrictions
		/// - This can only be called by the [`ControlOrigin`](Config::ControlOrigin)
		#[pallet::weight(T::WeightInfo::set_budget())]
		#[transactional]
		pub fn set_budget(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
			decay: T::BudgetPenaltyDecayer,
		) -> DispatchResultWithPostInfo {
			// Can also be token governance associated I reckon, as Angular holders should be able
			// to grant mosaic permission to mint. We'll save that for phase 3.
			T::ControlOrigin::ensure_origin(origin)?;

			<Pallet<T> as RelayerInterface>::set_budget(asset_id, amount, decay.clone());

			Self::deposit_event(Event::BudgetUpdated { asset_id, amount, decay });
			Ok(().into())
		}

		/// Creates an outgoing transaction request, locking the funds locally until picked up by
		/// the Relayer.
		///
		/// # Restrictions
		/// - Network must be supported.
		/// - AssetId must be supported.
		/// - Amount must be lower than the networks `max_transfer_size`.
		/// - Origin must have sufficient funds.
		/// - Transfers near Balance::max may result in overflows, which are caught and returned as
		///   an error.
		#[pallet::weight(T::WeightInfo::transfer_to())]
		#[transactional]
		// allowing too many arguments to keep the api simple for the Relayer team
		#[allow(clippy::too_many_arguments)]
		pub fn transfer_to(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			asset_id: AssetIdOf<T>,
			address: EthereumAddress,
			amount: BalanceOf<T>,
			minimum_amount_out: BalanceOf<T>,
			swap_to_native: bool,
			source_user_account: AccountIdOf<T>,
			amm_swap_info: Option<
				AmmSwapInfo<NetworkIdOf<T>, RemoteAmmIdOf<T>, AmmMinimumAmountOutOf<T>>,
			>,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(AssetsInfo::<T>::contains_key(asset_id), Error::<T>::UnsupportedAsset);
			let remote_asset_id = Self::get_remote_mapping(asset_id, network_id.clone())?;
			let network_info =
				NetworkInfos::<T>::get(network_id.clone()).ok_or(Error::<T>::UnsupportedNetwork)?;
			ensure!(network_info.enabled, Error::<T>::NetworkDisabled);
			ensure!(network_info.max_transfer_size >= amount, Error::<T>::ExceedsMaxTransferSize);
			ensure!(network_info.min_transfer_size <= amount, Error::<T>::BelowMinTransferSize);

			let now = <frame_system::Pallet<T>>::block_number();

			<Pallet<T> as TransferTo>::transfer_to(
				caller.clone(),
				asset_id,
				amount,
				keep_alive,
				now,
			)?;

			// Ensure that users can only swap using a whitelisted destination amm id
			if let Some(swap_info) = &amm_swap_info {
				ensure!(
					RemoteAmmWhitelist::<T>::contains_key(
						&swap_info.destination_amm.network_id,
						&swap_info.destination_amm.amm_id
					),
					Error::<T>::DestinationAmmIdNotWhitelisted
				);
			}

			let id = generate_id::<T>(&caller, &network_id, &asset_id, &address, &amount, &now);
			Self::deposit_event(Event::<T>::TransferOut {
				id,
				to: address,
				amount,
				asset_id,
				network_id,
				remote_asset_id,
				swap_to_native,
				source_user_account,
				amm_swap_info,
				minimum_amount_out,
			});

			Ok(().into())
		}

		/// This is called by the Relayer to confirm that it will relay a transaction.
		///
		/// Once this is called, the sender will be unable to reclaim their tokens.
		///
		/// If all the funds are not removed, the reclaim period will not be reset. If the
		/// reclaim period is not reset, the Relayer will still attempt to pick up the
		/// remainder of the transaction.
		///
		/// # Restrictions
		/// - Only callable by the current Relayer
		/// - Outgoing transaction must exist for the user
		/// - Amount must be equal or lower than what the user has locked
		///
		/// # Note
		/// - Reclaim period is not reset if not all the funds are moved; meaning that the clock
		///   remains ticking for the relayer to pick up the rest of the transaction.
		#[pallet::weight(T::WeightInfo::accept_transfer())]
		#[transactional]
		pub fn accept_transfer(
			origin: OriginFor<T>,
			from: AccountIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_relayer(origin)?;
			let asset_id = Self::get_local_mapping(remote_asset_id.clone(), network_id.clone())?;

			<Pallet<T> as RelayerInterface>::accept_transfer(
				asset_id,
				from,
				network_id,
				remote_asset_id,
				amount,
			)?;

			Ok(().into())
		}

		/// Claims user funds from the `OutgoingTransactions`, in case that the Relayer has not
		/// picked them up.
		#[pallet::weight(T::WeightInfo::claim_stale_to())]
		#[transactional]
		pub fn claim_stale_to(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			to: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			let now = <frame_system::Pallet<T>>::block_number();

			<Pallet<T> as TransferTo>::claim_stale_to(caller, asset_id, to, now)?;

			Ok(().into())
		}

		/// Mints new tokens into the pallet's wallet, ready for the user to be picked up after
		/// `lock_time` blocks have expired.
		///
		/// Only callable by the current Relayer
		#[pallet::weight(T::WeightInfo::timelocked_mint())]
		#[transactional]
		pub fn timelocked_mint(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			to: AccountIdOf<T>,
			amount: BalanceOf<T>,
			lock_time: BlockNumberOf<T>,
			id: Id,
		) -> DispatchResultWithPostInfo {
			let (_caller, current_block) = Self::ensure_relayer(origin)?;
			let asset_id = Self::get_local_mapping(remote_asset_id.clone(), network_id.clone())?;

			<Pallet<T> as RelayerInterface>::timelocked_mint(
				asset_id,
				current_block,
				to.clone(),
				amount,
				lock_time,
			)?;

			Self::deposit_event(Event::<T>::TransferInto {
				id,
				to,
				network_id,
				remote_asset_id,
				asset_id,
				amount,
			});

			Ok(().into())
		}

		/// Sets the time lock, in blocks, on new transfers
		///
		/// This can only be called by the [`ControlOrigin`](Config::ControlOrigin)
		#[pallet::weight(T::WeightInfo::set_timelock_duration())]
		#[transactional]
		pub fn set_timelock_duration(
			origin: OriginFor<T>,
			period: Validated<BlockNumberOf<T>, ValidTimeLockPeriod<T::MinimumTimeLockPeriod>>,
		) -> DispatchResultWithPostInfo {
			let validated_period = period.value();
			T::ControlOrigin::ensure_origin(origin)?;

			<Pallet<T> as RelayerInterface>::set_timelock_duration(validated_period);

			Ok(().into())
		}

		/// Burns funds waiting in incoming_transactions that are still unclaimed.
		///
		/// May be used by the Relayer in case of finality issues on the other side of the bridge.
		#[pallet::weight(T::WeightInfo::rescind_timelocked_mint())]
		#[transactional]
		pub fn rescind_timelocked_mint(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: RemoteAssetIdOf<T>,
			account: AccountIdOf<T>,
			untrusted_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_relayer(origin)?;
			let asset_id = Self::get_local_mapping(remote_asset_id.clone(), network_id.clone())?;

			<Pallet<T> as RelayerInterface>::rescind_timelocked_mint(
				asset_id,
				account.clone(),
				untrusted_amount,
			)?;

			Self::deposit_event(Event::<T>::TransferIntoRescined {
				account,
				amount: untrusted_amount,
				asset_id,
			});

			Ok(().into())
		}

		/// Collects funds deposited by the Relayer into the owner's account
		#[pallet::weight(T::WeightInfo::claim_to())]
		#[transactional]
		pub fn claim_to(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			to: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let now = <frame_system::Pallet<T>>::block_number();

			<Pallet<T> as Claim>::claim_to(caller, asset_id, to, now)?;

			Ok(().into())
		}

		/// Update a network asset mapping.
		///
		/// This can only be called by the [`ControlOrigin`](Config::ControlOrigin)
		///
		/// Possibly emits one of:
		/// - `AssetMappingCreated`
		/// - `AssetMappingDeleted`
		/// - `AssetMappingUpdated`
		#[pallet::weight(T::WeightInfo::update_asset_mapping())]
		#[transactional]
		pub fn update_asset_mapping(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
			remote_asset_id: Option<RemoteAssetIdOf<T>>,
		) -> DispatchResultWithPostInfo {
			T::ControlOrigin::ensure_origin(origin)?;
			let _ =
				NetworkInfos::<T>::get(network_id.clone()).ok_or(Error::<T>::UnsupportedNetwork)?;
			let entry = LocalToRemoteAsset::<T>::try_get(asset_id, network_id.clone()).ok();
			match (entry, remote_asset_id) {
				// remove an non-existent entry.
				(None, None) => {},
				// insert a new entry.
				(None, Some(remote_asset_id)) => {
					LocalToRemoteAsset::<T>::insert(
						asset_id,
						network_id.clone(),
						remote_asset_id.clone(),
					);
					RemoteToLocalAsset::<T>::insert(
						remote_asset_id.clone(),
						network_id.clone(),
						asset_id,
					);
					Self::deposit_event(Event::<T>::AssetMappingCreated {
						asset_id,
						network_id,
						remote_asset_id,
					});
				},
				// remove an existing entry.
				(Some(remote_asset_id), None) => {
					LocalToRemoteAsset::<T>::remove(asset_id, network_id.clone());
					RemoteToLocalAsset::<T>::remove(remote_asset_id.clone(), network_id.clone());
					Self::deposit_event(Event::<T>::AssetMappingDeleted {
						asset_id,
						network_id,
						remote_asset_id,
					});
				},
				// update an existing entry
				(Some(old_remote_asset_id), Some(new_remote_asset_id)) => {
					LocalToRemoteAsset::<T>::insert(
						asset_id,
						network_id.clone(),
						new_remote_asset_id.clone(),
					);
					RemoteToLocalAsset::<T>::remove(old_remote_asset_id, network_id.clone());
					RemoteToLocalAsset::<T>::insert(
						new_remote_asset_id.clone(),
						network_id.clone(),
						asset_id,
					);
					Self::deposit_event(Event::<T>::AssetMappingUpdated {
						asset_id,
						network_id,
						remote_asset_id: new_remote_asset_id,
					});
				},
			}
			Ok(().into())
		}

		/// Adds a remote AMM for a specific Network
		#[pallet::weight(T::WeightInfo::add_remote_amm_id())]
		#[transactional]
		pub fn add_remote_amm_id(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			amm_id: RemoteAmmIdOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ControlOrigin::ensure_origin(origin)?;

			ensure!(
				!RemoteAmmWhitelist::<T>::contains_key(&network_id, &amm_id),
				Error::<T>::RemoteAmmIdAlreadyExists,
			);

			RemoteAmmWhitelist::<T>::insert(network_id, amm_id, ());

			Ok(().into())
		}

		/// Removes a remote AMM for a specific Network
		#[pallet::weight(T::WeightInfo::remove_remote_amm_id())]
		#[transactional]
		pub fn remove_remote_amm_id(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			amm_id: RemoteAmmIdOf<T>,
		) -> DispatchResultWithPostInfo {
			T::ControlOrigin::ensure_origin(origin)?;

			ensure!(
				RemoteAmmWhitelist::<T>::contains_key(&network_id, &amm_id),
				Error::<T>::RemoteAmmIdNotFound
			);

			RemoteAmmWhitelist::<T>::remove(network_id, amm_id);

			Ok(().into())
		}
	}

	#[pallet::extra_constants]
	impl<T: Config> Pallet<T> {
		pub fn timelock_period() -> BlockNumberOf<T> {
			TimeLockPeriod::<T>::get()
		}
	}

	impl<T: Config> Pallet<T> {
		/// AccountId of the pallet, used to store all funds before actually moving them.
		pub(crate) fn sub_account_id(sub_account: SubAccount<T>) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account_truncating(sub_account.to_id())
		}

		/// Queries storage, returning the account_id of the current relayer.
		#[allow(dead_code)]
		pub(crate) fn relayer_account_id() -> Result<AccountIdOf<T>, DispatchError> {
			let current_block = <frame_system::Pallet<T>>::block_number();
			Ok(Relayer::<T>::get()
				.ok_or(Error::<T>::RelayerNotSet)?
				.update(current_block)
				.account_id()
				.clone())
		}

		pub(crate) fn ensure_relayer(
			origin: OriginFor<T>,
		) -> Result<
			(RelayerConfig<AccountIdOf<T>, BlockNumberOf<T>>, BlockNumberOf<T>),
			DispatchError,
		> {
			let acc = ensure_signed(origin).map_err(|_| DispatchError::BadOrigin)?;
			let current_block = <frame_system::Pallet<T>>::block_number();
			let relayer =
				Relayer::<T>::get().ok_or(Error::<T>::RelayerNotSet)?.update(current_block);
			ensure!(relayer.is_relayer(&acc), DispatchError::BadOrigin);
			Ok((relayer, current_block))
		}

		pub(crate) fn get_local_mapping(
			remote_asset_id: RemoteAssetIdOf<T>,
			network_id: NetworkIdOf<T>,
		) -> Result<AssetIdOf<T>, Error<T>> {
			RemoteToLocalAsset::<T>::try_get(remote_asset_id, network_id)
				.map_err(|_| Error::<T>::AssetNotMapped)
		}

		pub(crate) fn get_remote_mapping(
			asset_id: AssetIdOf<T>,
			network_id: NetworkIdOf<T>,
		) -> Result<RemoteAssetIdOf<T>, Error<T>> {
			LocalToRemoteAsset::<T>::try_get(asset_id, network_id)
				.map_err(|_| Error::<T>::AssetNotMapped)
		}
	}

	impl<T: Config> RelayerInterface for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type BlockNumber = T::BlockNumber;
		type BudgetPenaltyDecayer = T::BudgetPenaltyDecayer;
		type NetworkId = NetworkIdOf<T>;
		type NetworkInfo = NetworkInfo<BalanceOf<T>>;
		type RelayerConfig = RelayerConfig<T::AccountId, T::BlockNumber>;
		type RemoteAssetId = RemoteAssetIdOf<T>;

		fn accept_transfer(
			asset_id: Self::AssetId,
			from: Self::AccountId,
			network_id: Self::NetworkId,
			remote_asset_id: Self::RemoteAssetId,
			amount: Self::Balance,
		) -> DispatchResultWithPostInfo {
			OutgoingTransactions::<T>::try_mutate_exists::<_, _, _, DispatchError, _>(
				from.clone(),
				asset_id,
				|maybe_tx| match *maybe_tx {
					Some((balance, lock_period)) => {
						ensure!(amount <= balance, Error::<T>::AmountMismatch);
						T::Assets::burn_from(
							asset_id,
							&Self::sub_account_id(SubAccount::new_outgoing(from.clone())),
							amount,
						)?;

						// No remaining funds need to be transferred for this asset, so we can
						// delete the storage item.
						if amount == balance {
							*maybe_tx = None;
							Self::deposit_event(Event::<T>::TransferAccepted {
								from,
								network_id,
								remote_asset_id,
								asset_id,
								amount,
							});
						} else {
							let new_balance =
								balance.checked_sub(&amount).ok_or(Error::<T>::AmountMismatch)?;
							*maybe_tx = Some((new_balance, lock_period));
							Self::deposit_event(Event::<T>::PartialTransferAccepted {
								from,
								network_id,
								remote_asset_id,
								asset_id,
								amount,
							});
						}

						Ok(())
					},
					None => Err(Error::<T>::NoOutgoingTx.into()),
				},
			)?;

			Ok(().into())
		}

		fn rotate_relayer(
			relayer: Self::RelayerConfig,
			new: Self::AccountId,
			ttl: Self::BlockNumber,
			current_block: Self::BlockNumber,
		) {
			let ttl = current_block.saturating_add(ttl);
			let relayer = relayer.rotate(new, ttl);
			Relayer::<T>::set(Some(relayer.into()));
		}

		fn rescind_timelocked_mint(
			asset_id: Self::AssetId,
			account: Self::AccountId,
			untrusted_amount: Self::Balance,
		) -> DispatchResultWithPostInfo {
			IncomingTransactions::<T>::try_mutate_exists::<_, _, _, DispatchError, _>(
				account.clone(),
				asset_id,
				|prev| {
					let (balance, _) = prev.as_mut().ok_or(Error::<T>::NoClaimableTx)?;
					// Wipe the entire incoming transaction.
					if *balance == untrusted_amount {
						*prev = None;
					} else {
						*balance = balance.saturating_sub(untrusted_amount);
					}
					T::Assets::burn_from(
						asset_id,
						&Self::sub_account_id(SubAccount::new_incoming(account.clone())),
						untrusted_amount,
					)?;
					Ok(())
				},
			)?;
			Ok(().into())
		}

		fn set_budget(
			asset_id: Self::AssetId,
			amount: Self::Balance,
			decay: Self::BudgetPenaltyDecayer,
		) {
			let current_block = <frame_system::Pallet<T>>::block_number();

			AssetsInfo::<T>::mutate(asset_id, |item| {
				let new = item
					.take()
					.map(|mut asset_info| {
						asset_info.budget = amount;
						asset_info.penalty_decayer = decay.clone();
						asset_info
					})
					.unwrap_or_else(|| AssetInfo {
						last_mint_block: current_block,
						budget: amount,
						penalty: Zero::zero(),
						penalty_decayer: decay.clone(),
					});
				*item = Some(new);
			});
		}

		fn set_network(network_id: Self::NetworkId, network_info: Self::NetworkInfo) {
			NetworkInfos::<T>::insert(network_id, network_info);
		}

		fn set_relayer(relayer: Self::AccountId) {
			Relayer::<T>::set(Some(StaleRelayer::new(relayer)));
		}

		fn set_timelock_duration(period: Self::BlockNumber) {
			TimeLockPeriod::<T>::set(period);
		}

		fn timelocked_mint(
			asset_id: Self::AssetId,
			current_block: Self::BlockNumber,
			to: Self::AccountId,
			amount: Self::Balance,
			lock_time: Self::BlockNumber,
		) -> DispatchResultWithPostInfo {
			AssetsInfo::<T>::try_mutate_exists::<_, _, DispatchError, _>(asset_id, |info| {
				let AssetInfo { last_mint_block, penalty, budget, penalty_decayer } =
					info.take().ok_or(Error::<T>::UnsupportedAsset)?;

				let new_penalty = penalty_decayer
					.checked_decay(penalty, current_block, last_mint_block)
					.unwrap_or_else(Zero::zero);

				let penalised_budget = budget.saturating_sub(new_penalty);

				// Check if the relayer has a sufficient budget to mint the requested amount.
				ensure!(amount <= penalised_budget, Error::<T>::InsufficientBudget);

				T::Assets::mint_into(
					asset_id,
					&Self::sub_account_id(SubAccount::new_incoming(to.clone())),
					amount,
				)?;

				let lock_at = current_block.saturating_add(lock_time);

				IncomingTransactions::<T>::mutate(to.clone(), asset_id, |prev| match prev {
					Some((balance, _)) =>
						*prev = Some(((*balance).saturating_add(amount), lock_at)),
					_ => *prev = Some((amount, lock_at)),
				});

				*info = Some(AssetInfo {
					last_mint_block: current_block,
					budget,
					penalty: new_penalty.saturating_add(amount),
					penalty_decayer,
				});

				Ok(())
			})?;

			Ok(().into())
		}
	}

	impl<T: Config> TransferTo for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type BlockNumber = BlockNumberOf<T>;

		fn claim_stale_to(
			caller: Self::AccountId,
			asset_id: Self::AssetId,
			to: Self::AccountId,
			now: Self::BlockNumber,
		) -> DispatchResultWithPostInfo {
			OutgoingTransactions::<T>::try_mutate_exists(
				caller.clone(),
				asset_id,
				|prev| -> Result<(), DispatchError> {
					let amount = match *prev {
						Some((balance, lock_time)) => {
							let still_locked = lock_time >= now;
							if still_locked {
								Err(Error::<T>::TxStillLocked)
							} else {
								T::Assets::transfer(
									asset_id,
									&Self::sub_account_id(SubAccount::new_outgoing(caller.clone())),
									&to,
									balance,
									false,
								)?;
								Ok(balance)
							}
						},
						_ => Err(Error::<T>::NoStaleTransactions),
					}?;

					*prev = None;
					Self::deposit_event(Event::<T>::StaleTxClaimed {
						to,
						asset_id,
						by: caller,
						amount,
					});
					Ok(())
				},
			)?;

			Ok(().into())
		}

		fn transfer_to(
			caller: Self::AccountId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
			now: Self::BlockNumber,
		) -> DispatchResultWithPostInfo {
			T::Assets::transfer(
				asset_id,
				&caller,
				&Self::sub_account_id(SubAccount::new_outgoing(caller.clone())),
				amount,
				keep_alive,
			)?;

			let lock_until = now.safe_add(&TimeLockPeriod::<T>::get())?;

			OutgoingTransactions::<T>::try_mutate(
				caller,
				asset_id,
				|tx| -> Result<(), DispatchError> {
					match tx.as_mut() {
						// If we already have an outgoing tx, we update the lock_time and add the
						// amount.
						Some((already_locked, _)) => {
							let amount = amount.safe_add(already_locked)?;
							*tx = Some((amount, lock_until))
						},
						None => *tx = Some((amount, lock_until)),
					}
					Ok(())
				},
			)?;

			Ok(().into())
		}
	}

	impl<T: Config> Claim for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type BlockNumber = BlockNumberOf<T>;

		fn claim_to(
			caller: Self::AccountId,
			asset_id: Self::AssetId,
			to: Self::AccountId,
			now: Self::BlockNumber,
		) -> DispatchResultWithPostInfo {
			IncomingTransactions::<T>::try_mutate_exists::<_, _, _, DispatchError, _>(
				caller.clone(),
				asset_id,
				|deposit| {
					let (amount, unlock_after) = deposit.ok_or(Error::<T>::NoClaimableTx)?;
					ensure!(unlock_after < now, Error::<T>::TxStillLocked);
					T::Assets::transfer(
						asset_id,
						&Self::sub_account_id(SubAccount::new_incoming(caller.clone())),
						&to,
						amount,
						false,
					)?;
					// Delete the deposit.
					deposit.take();
					Self::deposit_event(Event::<T>::TransferClaimed {
						by: caller,
						to,
						asset_id,
						amount,
					});
					Ok(())
				},
			)?;

			Ok(().into())
		}
	}

	/// Uses Keccak256 to generate an identifier for
	pub(crate) fn generate_id<T: Config>(
		to: &AccountIdOf<T>,
		network_id: &NetworkIdOf<T>,
		asset_id: &AssetIdOf<T>,
		address: &EthereumAddress,
		amount: &BalanceOf<T>,
		block_number: &BlockNumberOf<T>,
	) -> Id {
		use sp_runtime::traits::Hash;

		let nonce = Nonce::<T>::mutate(|nonce| {
			// TODO: Use WrappingNext here
			*nonce = nonce.wrapping_add(1);
			*nonce
		});

		Keccak256::hash_of(&(to, network_id, asset_id, address, amount, &block_number, nonce))
	}
}
