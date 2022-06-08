#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

#[frame_support::pallet]
pub mod pallet {
	use composable_support::{
		abstractions::block_fold::{BlockFold, FoldStorage, FoldStrategy},
		collections::vec::bounded::BiBoundedVec,
		math::safe::{safe_multiply_by_rational, SafeAdd, SafeSub},
	};
	use composable_traits::{
		financial_nft::{FinancialNftProtocol, NftClass, NftVersion},
		staking_rewards::{
			Penalty, PenaltyOutcome, PositionState, Staking, StakingConfig, StakingNFT,
			StakingReward,
		},
		time::{DurationSeconds, Timestamp},
	};
	use frame_support::{
		pallet_prelude::*,
		storage::{bounded_btree_map::BoundedBTreeMap, bounded_btree_set::BoundedBTreeSet},
		traits::{
			fungibles::{
				Inspect as FungiblesInspect, Mutate as FungiblesMutate,
				Transfer as FungiblesTransfer,
			},
			tokens::{AssetId, Balance},
			IsType, UnixTime,
		},
		transactional, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{AccountIdConversion, Zero},
		ArithmeticError, Perbill, SaturatedConversion,
	};
	use sp_std::collections::btree_map::BTreeMap;

	pub(crate) type EpochId = u128;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type InstanceIdOf<T> = <T as FinancialNftProtocol<AccountIdOf<T>>>::InstanceId;
	pub(crate) type MaxRewardAssetsOf<T> = <T as Config>::MaxRewardAssets;
	pub(crate) type MaxStakingPresetsOf<T> = <T as Config>::MaxStakingPresets;
	pub(crate) type RewardAssetsOf<T> = BoundedBTreeSet<AssetIdOf<T>, MaxRewardAssetsOf<T>>;
	pub(crate) type DurationPresetsOf<T> =
		BoundedBTreeMap<DurationSeconds, Perbill, MaxStakingPresetsOf<T>>;
	pub(crate) type RewardsOf<T> =
		BoundedBTreeMap<AssetIdOf<T>, BalanceOf<T>, MaxRewardAssetsOf<T>>;
	pub(crate) type StakingNFTOf<T> =
		StakingNFT<AccountIdOf<T>, AssetIdOf<T>, BalanceOf<T>, EpochId, RewardsOf<T>>;
	pub(crate) type StakingConfigOf<T> =
		StakingConfig<AccountIdOf<T>, DurationPresetsOf<T>, RewardAssetsOf<T>>;
	pub(crate) type EpochDurationOf<T> = <T as Config>::EpochDuration;
	#[allow(dead_code)]
	pub(crate) type PenaltyOf<T> = Penalty<AccountIdOf<T>>;

	#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
	pub enum State {
		WaitingForEpochEnd,
		Rewarding,
		Registering,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An asset has been configured for staking.
		Configured { asset: AssetIdOf<T>, configuration: StakingConfigOf<T> },
		/// A user staked his protocol asset. Yield a NFT represeting his position.
		Staked { who: AccountIdOf<T>, stake: BalanceOf<T>, nft: InstanceIdOf<T> },
		/// A user unstaked his protocol asset.
		Unstaked {
			to: AccountIdOf<T>,
			stake: BalanceOf<T>,
			penalty: BalanceOf<T>,
			nft: InstanceIdOf<T>,
		},
		/// A new reward has been submitted, rewarding `rewarded_asset` with an `amount` of
		/// `reward_asset`.
		NewReward { rewarded_asset: AssetIdOf<T>, reward_asset: AssetIdOf<T>, amount: BalanceOf<T> },
		/// A new reward epoch started.
		NewEpoch { id: EpochId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NotConfigured,
		InvalidDurationPreset,
		TooManyRewardAssets,
		CannotClaimIfPending,
		ClaimRequireRestake,
		AlreadyTagged,
		EpochNotFound,
		PalletIsBusy,
		ImpossibleState,
	}

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ FinancialNftProtocol<AccountIdOf<Self>, ClassId = NftClass, Version = NftVersion>
	{
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The ID that uniquely identify an asset.
		type AssetId: AssetId + Ord;

		type Balance: Balance + TryFrom<u128>;

		/// The underlying currency system.
		type Assets: FungiblesInspect<
				AccountIdOf<Self>,
				AssetId = AssetIdOf<Self>,
				Balance = BalanceOf<Self>,
			> + FungiblesMutate<AccountIdOf<Self>>
			+ FungiblesTransfer<AccountIdOf<Self>>;

		/// The time provider.
		type Time: UnixTime;

		/// The governance origin, allowed to update sensitive values such as the unlock penalty.
		type GovernanceOrigin: EnsureOrigin<Self::Origin>;

		/// The pallet id, used to uniquely identify this pallet.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The maximum number of staking duration presets.
		#[pallet::constant]
		type MaxStakingPresets: Get<u32>;

		/// The maximum number of reward assets protocol asset can handle.
		#[pallet::constant]
		type MaxRewardAssets: Get<u32>;

		/// The duration of an epoch
		#[pallet::constant]
		type EpochDuration: Get<DurationSeconds>;

		#[pallet::constant]
		type ElementToProcessPerBlock: Get<u32>;
	}

	#[pallet::type_value]
	pub fn StateOnEmpty<T: Config>() -> State {
		State::WaitingForEpochEnd
	}

	#[pallet::storage]
	#[pallet::getter(fn current_state)]
	pub type CurrentState<T: Config> = StorageValue<_, State, ValueQuery, StateOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn fold_over_stakers)]
	pub type FoldState<T: Config> = StorageValue<_, BlockFold<(), InstanceIdOf<T>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn current_epoch_start)]
	pub type EpochStart<T: Config> = StorageValue<_, Timestamp, OptionQuery>;

	#[pallet::type_value]
	pub fn EpochOnEmpty<T: Config>() -> u128 {
		u128::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn current_epoch)]
	pub type CurrentEpoch<T: Config> = StorageValue<_, u128, ValueQuery, EpochOnEmpty<T>>;

	#[pallet::type_value]
	pub fn EpochRewardOnEmpty<T: Config>() -> BalanceOf<T> {
		BalanceOf::<T>::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn epoch_rewards)]
	pub type EpochRewards<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		(EpochId, AssetIdOf<T>),
		Twox64Concat,
		AssetIdOf<T>,
		BalanceOf<T>,
		ValueQuery,
		EpochRewardOnEmpty<T>,
	>;

	#[pallet::type_value]
	pub fn SharesOnEmpty<T: Config>() -> u128 {
		u128::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn total_shares)]
	pub type TotalShares<T: Config> = StorageMap<
		_,
		Twox64Concat,
		(AssetIdOf<T>, AssetIdOf<T>),
		u128,
		ValueQuery,
		SharesOnEmpty<T>,
	>;

	#[pallet::type_value]
	pub fn RewardStateOnEmpty<T: Config>() -> (EpochId, Timestamp) {
		(0, 0)
	}

	#[pallet::storage]
	#[pallet::getter(fn reward_state)]
	pub type EndEpochSnapshot<T: Config> =
		StorageValue<_, (EpochId, Timestamp), ValueQuery, RewardStateOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn stakers)]
	pub type Stakers<T: Config> = StorageMap<_, Twox64Concat, InstanceIdOf<T>, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pending_stakers)]
	pub type PendingStakers<T: Config> =
		StorageMap<_, Twox64Concat, InstanceIdOf<T>, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn staking_configurations)]
	pub type StakingConfigurations<T: Config> =
		StorageMap<_, Twox64Concat, AssetIdOf<T>, StakingConfigOf<T>, OptionQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Enable a protocol staking configuration.
		///
		/// Minimal staking duration must be larger or equal to epoch.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic, must be `T::GovernanceOrigin`.
		/// * `staking_configuration` the staking configuration for the given protocol `asset`.
		#[pallet::weight(10_000)]
		pub fn configure(
			origin: OriginFor<T>,
			asset: AssetIdOf<T>,
			configuration: StakingConfigOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = T::GovernanceOrigin::ensure_origin(origin)?;
			StakingConfigurations::<T>::insert(asset, configuration.clone());
			Self::deposit_event(Event::<T>::Configured { asset, configuration });
			Ok(().into())
		}

		/// Stake an amount of protocol asset tokens. Generating an NFT for the staked position.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Will be the owner of the  fNFT
		///   targeted by `instance_id`.
		/// * `amount` the amount of tokens to stake.
		/// * `duration` the duration for which the tokens will be staked.
		/// * `keep_alive` whether to keep the caller account alive or not.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn stake(
			origin: OriginFor<T>,
			asset: AssetIdOf<T>,
			amount: BalanceOf<T>,
			duration: Timestamp,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			<Self as Staking>::stake(&asset, &from, amount, duration, keep_alive)?;
			Ok(().into())
		}

		/// Unstake an amount of protocol asset tokens.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
		///   by `instance_id`.
		/// * `instance_id` the ID of the NFT that represent our staked position.
		/// * `to` the account in which the rewards will be transferred before unstaking.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn unstake(
			origin: OriginFor<T>,
			instance_id: InstanceIdOf<T>,
			to: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			T::ensure_protocol_nft_owner::<StakingNFTOf<T>>(&owner, &instance_id)?;
			<Self as Staking>::unstake(&instance_id, &to)?;
			Ok(().into())
		}

		/// Claim the current available rewards.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Can be anyone.
		/// * `instance_id` the ID of the NFT that represent our staked position.
		/// * `to` the account in which the rewards will be transferred.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim(
			origin: OriginFor<T>,
			instance_id: InstanceIdOf<T>,
			to: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// Only the owner is able to select an arbitrary `to` account.
			let nft_owner = T::get_protocol_nft_owner::<StakingNFTOf<T>>(&instance_id)?;
			let to = if nft_owner == who { to } else { nft_owner };
			<Self as Staking>::claim(&instance_id, &to)?;
			Ok(().into())
		}

		/// Splits fNFT position into several chunks with various amounts, but with same exposure.
		/// fNFT splitted earns reward in current epoch proportional to split.
		/// Can split only at  `State::WaitingForEpochEnd` state.
		///
		/// `origin` - owner of fNFT
		/// `amounts` - amount of in each fNFT, sum must equal to current stake.
		///
		///  raises event of NFT `SplitCreation`
		#[pallet::weight(10_000)]
		pub fn split(
			_origin: OriginFor<T>,
			_asset: InstanceIdOf<T>,
			_amounts: BiBoundedVec<T::Balance, 2, 16>,
		) -> DispatchResult {
			Err(DispatchError::Other("no implemented. TODO: call split on fnft provider"))
		}

		/// Extends fNFT position stake. Applied only to next epoch.
		#[pallet::weight(10_000)]
		pub fn extend_stake(
			_origin: OriginFor<T>,
			_instance_id: InstanceIdOf<T>,
			_balance: T::Balance,
		) -> DispatchResult {
			Err(DispatchError::Other("no implemented. TODO: insert update for next fold"))
		}

		/// Extends stake duration.
		/// `duration` - if none, then extend current duration from start. If more than current
		/// duration, takes some time from new duration.
		///
		/// Fails if `duration` extensions does not fits allowed.
		#[pallet::weight(10_000)]
		pub fn extend_duration(
			_origin: OriginFor<T>,
			_instance_id: InstanceIdOf<T>,
			_duration: Option<DurationSeconds>,
		) -> DispatchResult {
			Err(DispatchError::Other("no implemented. TODO: insert update for next fold").into())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_: T::BlockNumber) -> Weight {
			// TODO(hussein-aitlahcen): abstract per-block fold of chunk into a macro:
			match Self::current_state() {
				State::WaitingForEpochEnd => {
					// NOTE: we start new epoch here, it will work well if an only if epoch time is
					// longer than total fold time - which is most likely yes
					Self::update_epoch();
				},
				State::Rewarding => {
					let (reward_epoch, reward_epoch_start) = EndEpochSnapshot::<T>::get();
					let result = <(FoldState<T>, Stakers<T>)>::step(
						FoldStrategy::new_chunk(T::ElementToProcessPerBlock::get()),
						(),
						|_, nft_id, _| {
							let try_reward = T::try_mutate_protocol_nft(
								&nft_id,
								|nft: &mut StakingNFTOf<T>| -> DispatchResult {
									match nft.state(&reward_epoch, reward_epoch_start) {
										PositionState::Pending => {},
										PositionState::Expired => {
											// TODO: https://app.clickup.com/t/2xw5fca
										},
										PositionState::LockedRewarding => {
											// TODO: return here increased share if one of assets is
											// same as staked
											let shares = nft.shares();
											for (reward_asset, pending_reward) in
												nft.pending_rewards.clone().into_iter()
											{
												let total_shares =
													Self::total_shares((nft.asset, reward_asset));
												let reward = EpochRewards::<T>::get(
													(reward_epoch, nft.asset),
													reward_asset,
												)
												.saturated_into();
												let reward_shares = safe_multiply_by_rational(
													shares,
													reward,
													total_shares,
												)?;
												// TODO: if adding asset which is staked, increase
												// total
												nft.pending_rewards
													.try_insert(
														reward_asset,
														pending_reward.safe_add(
															&reward_shares.try_into().map_err(
																|_| ArithmeticError::Overflow,
															)?,
														)?,
													)
													.map_err(|_| ArithmeticError::Overflow)?;
											}
										},
									}
									Ok(())
								},
							);
							if let Err(e) = try_reward {
								log::warn!("Failed to reward NFT: {:?}, message: {:?}", nft_id, e);
							}
						},
					);
					if let BlockFold::Done { .. } = result {
						CurrentState::<T>::set(State::Registering);
					}
				},
				State::Registering => {
					// TODO: extend adding extensions to exisitng nfts
					let result = <(FoldState<T>, PendingStakers<T>)>::step(
						FoldStrategy::Chunk {
							number_of_elements: T::ElementToProcessPerBlock::get(),
						},
						(),
						|_, nft_id, _| {
							let nft = T::get_protocol_nft::<StakingNFTOf<T>>(&nft_id)
								.expect("impossible; qed");
							for reward_asset in nft.pending_rewards.keys() {
								TotalShares::<T>::mutate(
									(nft.asset, reward_asset),
									|total_shares| {
										*total_shares = total_shares
											.checked_add(nft.shares())
											.expect("impossible; qed;");
									},
								);
							}
							Stakers::<T>::insert(nft_id, ());
						},
					);
					if let BlockFold::Done { .. } = result {
						PendingStakers::<T>::remove_all(None);
						CurrentState::<T>::set(State::WaitingForEpochEnd);
					}
				},
			}
			0
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn ensure_valid_interaction_state() -> DispatchResult {
			match Self::current_state() {
				State::WaitingForEpochEnd => Ok(()),
				State::Rewarding | State::Registering => Err(Error::<T>::PalletIsBusy.into()),
			}
		}

		pub(crate) fn update_epoch() {
			let now = T::Time::now().as_secs();
			EpochStart::<T>::mutate(|entry| match entry {
				Some(epoch_start) => {
					let delta = now.checked_sub(*epoch_start).expect("back to the future; qed;");
					if delta > EpochDurationOf::<T>::get() {
						*epoch_start = now;
						Self::on_new_epoch();
					}
				},
				None => {
					*entry = Some(now);
					// NOTE(hussein-aitlahcen): on pallet initialization, new epoch is directly
					// started.
					Self::on_new_epoch();
				},
			});
		}

		pub(crate) fn on_new_epoch() {
			// Store current epoch snapshot.
			EndEpochSnapshot::<T>::set((Self::current_epoch(), Self::epoch_start()));
			// Increment epoch.
			CurrentEpoch::<T>::mutate(|x| *x = *x + 1);
			// Set rewarding state, i.e. rewarding previous epoch.
			CurrentState::<T>::set(State::Rewarding);
			// Notify.
			Self::deposit_event(Event::<T>::NewEpoch { id: Self::current_epoch() });
		}

		pub(crate) fn epoch_start() -> Timestamp {
			EpochStart::<T>::get().unwrap_or(0)
		}

		pub(crate) fn epoch_next() -> Result<EpochId, DispatchError> {
			Self::current_epoch().safe_add(&1).map_err(Into::into)
		}

		pub(crate) fn now() -> Timestamp {
			T::Time::now().as_secs()
		}

		/// The staking protocol account. Derived from the staking pallet id.
		pub(crate) fn account_id(asset: &AssetIdOf<T>) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(asset)
		}

		pub(crate) fn get_config(
			asset: &AssetIdOf<T>,
		) -> Result<StakingConfigOf<T>, DispatchError> {
			StakingConfigurations::<T>::get(asset).ok_or(Error::<T>::NotConfigured.into())
		}

		pub(crate) fn collect_rewards(
			nft: &mut StakingNFTOf<T>,
			to: &AccountIdOf<T>,
		) -> DispatchResult {
			let protocol_account = Self::account_id(&nft.asset);
			for (reward_asset, reward) in nft.pending_rewards.clone() {
				T::Assets::transfer(reward_asset, &protocol_account, to, reward, false)?;
				nft.pending_rewards.remove(&reward_asset);
			}
			Ok(())
		}
	}

	impl<T: Config> StakingReward for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;

		fn transfer_reward(
			asset: &Self::AssetId,
			reward_asset: &Self::AssetId,
			from: &Self::AccountId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			// Transfer the reward locally.
			let protocol_account = Self::account_id(asset);
			T::Assets::transfer(*reward_asset, from, &protocol_account, amount, keep_alive)?;
			EpochRewards::<T>::try_mutate(
				(Self::current_epoch(), *asset),
				reward_asset,
				|collected_amount| -> DispatchResult {
					*collected_amount = collected_amount.safe_add(&amount)?;
					Ok(())
				},
			)?;
			Self::deposit_event(Event::<T>::NewReward {
				rewarded_asset: *asset,
				reward_asset: *reward_asset,
				amount,
			});
			Ok(())
		}
	}

	impl<T: Config> Staking for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type InstanceId = T::InstanceId;

		fn stake(
			asset: &Self::AssetId,
			from: &Self::AccountId,
			amount: Self::Balance,
			duration: DurationSeconds,
			keep_alive: bool,
		) -> Result<Self::InstanceId, DispatchError> {
			Self::ensure_valid_interaction_state()?;
			let config = Self::get_config(asset)?;
			let reward_multiplier = *config
				.duration_presets
				.get(&duration)
				.ok_or(Error::<T>::InvalidDurationPreset)?;
			// Acquire protocol asset from user.
			let protocol_account = Self::account_id(asset);
			T::Assets::transfer(*asset, from, &protocol_account, amount, keep_alive)?;
			// Actually create the NFT representing the user position.
			let now = Self::now();
			let next_epoch = Self::epoch_next()?;
			// Initialize pending rewards to 0.
			let pending_rewards = config
				.reward_assets
				.into_iter()
				.map(|x| (x, BalanceOf::<T>::zero()))
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.map_err(|_| Error::<T>::ImpossibleState)?;
			let nft: StakingNFTOf<T> = StakingNFT {
				asset: *asset,
				stake: amount,
				reward_epoch_start: next_epoch,
				pending_rewards,
				lock_date: now,
				lock_duration: duration,
				early_unstake_penalty: config.early_unstake_penalty,
				reward_multiplier,
			};
			let instance_id = T::mint_protocol_nft(from, &nft)?;
			PendingStakers::<T>::insert(instance_id, ());
			// Trigger event
			Self::deposit_event(Event::<T>::Staked {
				who: from.clone(),
				stake: amount,
				nft: instance_id,
			});
			Ok(instance_id)
		}

		fn unstake(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult {
			Self::ensure_valid_interaction_state()?;
			<Self as Staking>::claim(instance_id, to)?;
			let nft = T::get_protocol_nft::<StakingNFTOf<T>>(instance_id)?;
			let protocol_account = Self::account_id(&nft.asset);
			let current_epoch = Self::current_epoch();
			let current_epoch_start = Self::epoch_start();
			let penalty_outcome = match nft.state(&current_epoch, current_epoch_start) {
				PositionState::Pending => {
					// When the position is not being rewarded yet, remove from the pending
					// amount.
					PendingStakers::<T>::remove(instance_id);
					Ok(PenaltyOutcome::NotApplied { amount: nft.stake })
				},
				PositionState::LockedRewarding => {
					// Decrement total shares.
					for reward_asset in nft.pending_rewards.keys() {
						TotalShares::<T>::try_mutate(
							(nft.asset, reward_asset),
							|total_shares| -> DispatchResult {
								*total_shares = total_shares.safe_sub(&nft.shares())?;
								Ok(())
							},
						)?;
					}
					nft.early_unstake_penalty.penalize::<BalanceOf<T>>(nft.stake)
				},
				PositionState::Expired => {
					// Decrement total shares.
					for reward_asset in nft.pending_rewards.keys() {
						TotalShares::<T>::try_mutate(
							(nft.asset, reward_asset),
							|total_shares| -> DispatchResult {
								*total_shares = total_shares.safe_sub(&nft.shares())?;
								Ok(())
							},
						)?;
					}
					Ok(PenaltyOutcome::NotApplied { amount: nft.stake })
				},
			}?;
			match penalty_outcome.clone() {
				PenaltyOutcome::Applied {
					amount_remaining,
					amount_penalty,
					penalty_beneficiary,
				} => {
					T::Assets::transfer(
						nft.asset,
						&protocol_account,
						&to,
						amount_remaining,
						false,
					)?;
					T::Assets::transfer(
						nft.asset,
						&protocol_account,
						&penalty_beneficiary,
						amount_penalty,
						false,
					)?;
				},
				PenaltyOutcome::NotApplied { amount } => {
					T::Assets::transfer(nft.asset, &protocol_account, &to, amount, false)?;
				},
			}
			// Actually burn the NFT from the storage.
			T::burn_protocol_nft::<StakingNFTOf<T>>(instance_id)?;
			// Trigger event
			Self::deposit_event(Event::<T>::Unstaked {
				to: to.clone(),
				stake: nft.stake,
				penalty: penalty_outcome.penalty_amount().unwrap_or(Zero::zero()),
				nft: *instance_id,
			});
			Ok(())
		}

		fn claim(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult {
			T::try_mutate_protocol_nft(instance_id, |nft: &mut StakingNFTOf<T>| -> DispatchResult {
				Self::collect_rewards(nft, to)?;
				Ok(())
			})
		}
	}
}
