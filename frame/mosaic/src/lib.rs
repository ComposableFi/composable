// TODO
// 1. TEST!
// 2. RPCs for relayer convenience.
// 3. Refactor core logic to traits.
// 4. Benchmarks and Weights!

#![cfg_attr(not(feature = "std"), no_std)]

mod decay;
mod relayer;

pub use decay::{BudgetDecay, Decayable};
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

	use crate::{
		decay::Decayable,
		relayer::{RelayerConfig, StaleRelayer},
	};
	use codec::FullCodec;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedAdd, Zero};
	use scale_info::TypeInfo;
	use sp_runtime::traits::{AccountIdConversion, Saturating};
	use sp_std::{fmt::Debug, str};

	type AccoundIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Assets as Inspect<AccoundIdOf<T>>>::Balance;
	type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	type AssetIdOf<T> = <<T as Config>::Assets as Inspect<AccoundIdOf<T>>>::AssetId;
	type NetworkIdOf<T> = <T as Config>::NetworkId;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type PalletId: Get<PalletId>;
		type Assets: Mutate<AccoundIdOf<Self>> + Transfer<AccoundIdOf<Self>>;

		type MinimumTTL: Get<BlockNumberOf<Self>>;
		type MinimumTimeLockPeriod: Get<BlockNumberOf<Self>>;

		type BudgetDecay: Decayable<BalanceOf<Self>, BlockNumberOf<Self>>
			+ Clone
			+ Encode
			+ Decode
			+ Debug
			+ TypeInfo
			+ PartialEq;

		type NetworkId: FullCodec + TypeInfo + Clone + Debug + PartialEq;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Relayer<T: Config> =
		StorageValue<_, StaleRelayer<T::AccountId, T::BlockNumber>, ValueQuery>;

	#[derive(Clone, Debug, Encode, Decode, TypeInfo, PartialEq)]
	pub struct AssetInfo<BlockNumber, Balance, Decayable> {
		last_deposit: BlockNumber,
		budget: Balance,
		penalty: Balance,
		decay: Decayable,
	}

	#[pallet::storage]
	pub type AssetsInfo<T: Config> = StorageMap<
		_,
		Twox64Concat,
		AssetIdOf<T>,
		AssetInfo<BlockNumberFor<T>, BalanceOf<T>, T::BudgetDecay>,
		OptionQuery,
	>;

	#[derive(Clone, Debug, Encode, Decode, TypeInfo, PartialEq)]
	pub struct NetworkInfo<Balance> {
		pub enabled: bool,
		pub max_transfer_size: Balance,
	}

	#[pallet::storage]
	pub type NetworkInfos<T: Config> =
		StorageMap<_, Twox64Concat, NetworkIdOf<T>, NetworkInfo<BalanceOf<T>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn time_lock_period)]
	pub type TimeLockPeriod<T: Config> =
		StorageValue<_, BlockNumberOf<T>, ValueQuery, TimeLockPeriodOnEmpty<T>>;

	#[pallet::type_value]
	pub fn TimeLockPeriodOnEmpty<T: Config>() -> BlockNumberOf<T> {
		T::MinimumTimeLockPeriod::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn outgoing_transactions)]
	pub type OutgoingTransactions<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		AccoundIdOf<T>,
		Twox64Concat,
		AssetIdOf<T>,
		(BalanceOf<T>, BlockNumberFor<T>),
		OptionQuery,
	>;

	#[pallet::storage]
	pub type IncomingTransactions<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		AccoundIdOf<T>,
		Twox64Concat,
		AssetIdOf<T>,
		(BalanceOf<T>, BlockNumberFor<T>),
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RelayerSet {
			relayer: AccoundIdOf<T>,
		},
		TransferOut {
			id: Id,
			to: EthereumAddress,
			amount: BalanceOf<T>,
			network_id: NetworkIdOf<T>,
		},
		StaleTxClaimed {
			to: AccoundIdOf<T>,
			by: AccoundIdOf<T>,
			amount: BalanceOf<T>,
		},
		TransferInto {
			to: AccoundIdOf<T>,
			amount: BalanceOf<T>,
			asset_id: AssetIdOf<T>,
			id: Id,
		},
		TransferAccepted {
			from: AccoundIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		TransferClaimed {
			by: AccoundIdOf<T>,
			to: AccoundIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		BadOrigin,
		BadTTL,
		UnsupportedAsset,
		NetworkDisabled,
		UnsupportedNetwork,
		Overflow,
		NoStaleTransactions,
		InsufficientBudget,
		ExceedsMaxTransferSize,
		NoClaimableTx,
		NoOutgoingTx,
		AmountMismatch,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the current relayer configuration. This is enacted immediately and invalidates
		/// inflight, incoming transactions from the previous relayer. Budgets remain in place
		/// however.
		#[pallet::weight(10_000)]
		pub fn set_relayer(
			origin: OriginFor<T>,
			relayer: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			Relayer::<T>::set(StaleRelayer::new(relayer.clone()));
			Self::deposit_event(Event::RelayerSet { relayer });
			Ok(().into())
		}

		/// Rotates the Relayer Account
		///
		/// # Restrictions
		///  - Only callable by the current relayer.
		///  - TTL must be sufficiently long.
		#[pallet::weight(10_000)]
		pub fn rotate_relayer(
			origin: OriginFor<T>,
			new: T::AccountId,
			ttl: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			ensure!(ttl > T::MinimumTTL::get(), Error::<T>::BadTTL);
			let (relayer, current_block) = ensure_relayer::<T>(origin)?;
			let ttl = current_block.saturating_add(ttl);
			let relayer = relayer.rotate(new, ttl);
			Relayer::<T>::set(relayer.into());
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn set_network(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			network_info: NetworkInfo<BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			ensure_relayer::<T>(origin)?;
			NetworkInfos::<T>::insert(network_id, network_info);
			Ok(().into())
		}

		/// Sets the relayer budget for _incoming_ transactions for specific assets.
		///
		/// # Restrictions
		/// - Only callable by root
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn set_budget(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
			decay: T::BudgetDecay,
		) -> DispatchResultWithPostInfo {
			// Can also be token governance associated I reckon, as Angular holders should be able
			// to grant mosaic permission to mint. We'll save that for phase 3.
			ensure_root(origin)?;
			let current_block = <frame_system::Pallet<T>>::block_number();

			AssetsInfo::<T>::mutate(asset_id, |item| {
				let new = item
					.take()
					.map(|mut asset_info| {
						asset_info.budget = amount;
						asset_info.decay = decay.clone();
						asset_info
					})
					.unwrap_or_else(|| AssetInfo {
						last_deposit: current_block,
						budget: amount,
						penalty: Zero::zero(),
						decay,
					});
				*item = Some(new);
			});

			Ok(().into())
		}

		/// Creates an outgoing transaction request, locking the funds locally until picked up by
		/// the relayer.
		///
		/// # Restrictions
		/// - Network must be supported.
		/// - AssetId must be supported.
		/// - Amount must be lower than the networks `max_transfer_size`.
		/// - Origin must have sufficient funds.
		/// - Transfers near Balance::max may result in overflows, which are caught and returned as
		///   an error.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer_to(
			origin: OriginFor<T>,
			network_id: NetworkIdOf<T>,
			asset_id: AssetIdOf<T>,
			address: EthereumAddress,
			amount: BalanceOf<T>,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(AssetsInfo::<T>::contains_key(asset_id), Error::<T>::UnsupportedAsset);
			let network_info =
				NetworkInfos::<T>::get(network_id.clone()).ok_or(Error::<T>::UnsupportedNetwork)?;
			ensure!(network_info.enabled, Error::<T>::NetworkDisabled);
			ensure!(network_info.max_transfer_size > amount, Error::<T>::ExceedsMaxTransferSize);

			T::Assets::transfer(asset_id, &caller, &Self::account_id(), amount, keep_alive)?;

			let lock_until = <frame_system::Pallet<T>>::block_number()
				.checked_add(&TimeLockPeriod::<T>::get())
				.ok_or(Error::<T>::Overflow)?;

			OutgoingTransactions::<T>::try_mutate(
				caller.clone(),
				asset_id,
				|prev| -> Result<(), Error<T>> {
					match prev.as_mut() {
						// If we already have an outgoing tx, we update the lock_time and add the
						// amount.
						Some((already_locked, _)) => {
							let amount =
								amount.checked_add(already_locked).ok_or(Error::<T>::Overflow)?;
							*prev = Some((amount, lock_until))
						},
						None => *prev = Some((amount, lock_until)),
					}
					Ok(())
				},
			)?;

			let id = generate_id::<T>(&caller, &network_id, &asset_id, &address, &amount);
			Self::deposit_event(Event::<T>::TransferOut { to: address, amount, network_id, id });

			Ok(().into())
		}

		/// Called by the relayer to confirm that it will relay a transaction, disabling the user
		/// from reclaiming their tokens.
		///
		/// # Restrictions
		/// - Origin must be relayer
		/// - Outgoing transaction must exist for the user
		/// - Amount must be equal or lower than what the user has locked
		///
		/// # Note
		/// - Reclaim period is not reset if not all the funds are moved; menaing that the clock
		///   remains ticking for the relayer to pick up the rest of the transaction.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn accept_transfer(
			origin: OriginFor<T>,
			from: AccoundIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_relayer::<T>(origin)?;
			OutgoingTransactions::<T>::try_mutate_exists::<_, _, _, DispatchError, _>(
				from.clone(),
				asset_id,
				|maybe_tx| match maybe_tx.as_mut() {
					Some(tx) => {
						ensure!(amount <= tx.0, Error::<T>::AmountMismatch);
						T::Assets::burn_from(asset_id, &Self::account_id(), tx.0)?;

						// No remaing funds need to be transferred for this asset, so we can delete
						// the storage item.
						if amount == tx.0 {
							*maybe_tx = None
						}

						Self::deposit_event(Event::<T>::TransferAccepted {
							from,
							asset_id,
							amount,
						});
						Ok(())
					},
					None => Err(Error::<T>::NoOutgoingTx.into()),
				},
			)?;
			Ok(().into())
		}

		/// Claims user funds from the `OutgoingTransactions`, in case that the relayer has not
		/// picked them up.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim_stale_to(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			to: AccoundIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			let now = <frame_system::Pallet<T>>::block_number();

			OutgoingTransactions::<T>::try_mutate_exists(
				caller.clone(),
				asset_id,
				|prev| -> Result<(), DispatchError> {
					let amount = match prev {
						Some(tx) if tx.1 < now => {
							T::Assets::transfer(asset_id, &Self::account_id(), &to, tx.0, true)?;
							tx.0
						},
						_ => return Err(Error::<T>::NoStaleTransactions.into()),
					};

					*prev = None;
					Self::deposit_event(Event::<T>::StaleTxClaimed { to, by: caller, amount });
					Ok(())
				},
			)?;
			Ok(().into())
		}

		/// Mints new tokens into the pallet's wallet, ready for the user to be picked up after
		/// `lock_time` blocks have expired.
		#[pallet::weight(10_000)]
		pub fn timelocked_mint(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			to: AccoundIdOf<T>,
			amount: BalanceOf<T>,
			lock_time: BlockNumberOf<T>,
			id: Id,
		) -> DispatchResultWithPostInfo {
			let (_caller, current_block) = ensure_relayer::<T>(origin)?;

			let AssetInfo { last_deposit, penalty, budget, decay } =
				AssetsInfo::<T>::get(asset_id).ok_or(Error::<T>::UnsupportedAsset)?;
			let penalty = decay
				.checked_decay(penalty, last_deposit, current_block)
				.unwrap_or_else(Zero::zero);
			let budget = budget.saturating_sub(penalty);
			ensure!(budget > amount, Error::<T>::InsufficientBudget);

			T::Assets::mint_into(asset_id, &Self::account_id(), amount)?;
			let lock_at = lock_time.saturating_add(current_block);

			IncomingTransactions::<T>::mutate(to.clone(), asset_id, |prev| match prev.as_mut() {
				Some(tx) => *tx = (tx.0.saturating_add(amount), lock_at),
				_ => *prev = Some((amount, lock_at)),
			});

			Self::deposit_event(Event::<T>::TransferInto { to, asset_id, amount, id });
			Ok(().into())
		}

		// TODO Add remove incoming transaction

		/// Collects funds deposited by the relayer into the
		#[pallet::weight(10_000)]
		pub fn claim_to(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			to: AccoundIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let now = <frame_system::Pallet<T>>::block_number();

			IncomingTransactions::<T>::try_mutate_exists::<_, _, _, DispatchError, _>(
				to.clone(),
				asset_id,
				|deposit| {
					let tx = deposit.ok_or(Error::<T>::NoClaimableTx)?;
					ensure!(tx.1 > now, Error::<T>::NoClaimableTx);
					T::Assets::transfer(asset_id, &Self::account_id(), &to, tx.0, true)?;
					// Delete the deposit.
					deposit.take();
					Self::deposit_event(Event::<T>::TransferClaimed {
						by: caller,
						to,
						asset_id,
						amount: tx.0,
					});
					Ok(())
				},
			)?;
			Ok(().into())
		}
	}

	fn ensure_relayer<T: Config>(
		origin: OriginFor<T>,
	) -> Result<(RelayerConfig<AccoundIdOf<T>, BlockNumberOf<T>>, BlockNumberOf<T>), Error<T>> {
		let acc = ensure_signed(origin).map_err(|_| Error::<T>::BadOrigin)?;
		let current_block = <frame_system::Pallet<T>>::block_number();
		let relayer = Relayer::<T>::get().update(current_block);
		ensure!(relayer.is_relayer(&acc), Error::<T>::BadOrigin);
		Ok((relayer, current_block))
	}

	#[pallet::extra_constants]
	impl<T: Config> Pallet<T> {
		/// AccountId of the pallet, used to store all funds before actually moving them.
		fn account_id() -> AccoundIdOf<T> {
			T::PalletId::get().into_account()
		}
	}

	impl<T: Config> Pallet<T> {
		/// Queries storage, returning the account_id of the current relayer.
		pub fn relayer_account_id() -> Option<AccoundIdOf<T>> {
			let current_block = <frame_system::Pallet<T>>::block_number();
			Relayer::<T>::get().update(current_block).account_id().map(|acc| acc.clone())
		}
	}

	/// Convenience identifiers emitted by the pallet for relayer bookkeeping.
	pub type Id = [u8; 32];

	/// Raw ethereum addresses.
	pub type EthereumAddress = [u8; 20];

	fn generate_id<T: Config>(
		_to: &AccoundIdOf<T>,
		_network_id: &NetworkIdOf<T>,
		_asset_id: &AssetIdOf<T>,
		_address: &EthereumAddress,
		_amount: &BalanceOf<T>,
	) -> Id {
		todo!(
			"generate an id based of the hash of the input parameters, and an incrementing nonce."
		)
	}
}
