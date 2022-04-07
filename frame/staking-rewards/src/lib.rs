#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
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
	use composable_traits::{
		financial_nft::{FinancialNFTProtocol, NFTClass, NFTVersion},
		math::{safe_multiply_by_rational, SafeAdd, SafeSub},
		staking_rewards::{
			ClaimStrategy, PositionState, Staking, StakingConfig, StakingNFT, StakingReward,
			StakingTag,
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
	use sp_core::U512;
	use sp_runtime::{
		traits::{AccountIdConversion, Zero},
		ArithmeticError, Perbill, SaturatedConversion,
	};
	use sp_std::collections::btree_map::BTreeMap;

	pub(crate) type CollectedReward = U512;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type InstanceIdOf<T> = <T as FinancialNFTProtocol<AccountIdOf<T>>>::InstanceId;
	pub(crate) type MaxRewardAssetsOf<T> = <T as Config>::MaxRewardAssets;
	pub(crate) type MaxStakingPresetsOf<T> = <T as Config>::MaxStakingPresets;
	pub(crate) type CollectedRewardsOf<T> =
		BoundedBTreeMap<AssetIdOf<T>, CollectedReward, MaxRewardAssetsOf<T>>;
	pub(crate) type StakingNFTOf<T> =
		StakingNFT<AccountIdOf<T>, AssetIdOf<T>, BalanceOf<T>, CollectedRewardsOf<T>>;
	pub(crate) type StakingConfigOf<T> = StakingConfig<
		AccountIdOf<T>,
		BoundedBTreeMap<DurationSeconds, Perbill, MaxStakingPresetsOf<T>>,
		BoundedBTreeSet<AssetIdOf<T>, MaxRewardAssetsOf<T>>,
	>;
	pub(crate) type EpochDurationOf<T> = <T as Config>::EpochDuration;

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
		/// A nft has been tagged after expiry.
		Tagged { who: AccountIdOf<T>, beneficiary: AccountIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		NotConfigured,
		InvalidDurationPreset,
		TooManyRewardAssets,
		RewardAssetDisabled,
		CannotClaimIfPending,
		ClaimRequireRestake,
		AlreadyTagged,
	}

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ FinancialNFTProtocol<AccountIdOf<Self>, ClassId = NFTClass, Version = NFTVersion>
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

		/// The penalty applied to the reward transfered to the tagger.
		#[pallet::constant]
		type TagRewardPenatly: Get<Perbill>;
	}

	#[pallet::storage]
	#[pallet::getter(fn current_epoch_start)]
	pub type EpochStart<T: Config> = StorageValue<_, Timestamp, OptionQuery>;

	#[pallet::type_value]
	pub fn SharesOnEmpty<T: Config>() -> u128 {
		u128::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn total_shares)]
	pub type TotalShares<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, u128, ValueQuery, SharesOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn total_shares_pending)]
	pub type TotalSharesPending<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, u128, ValueQuery, SharesOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn collected_rewards)]
	pub type CollectedRewards<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		AssetIdOf<T>,
		CollectedReward,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn staking_configurations)]
	pub type StakingConfigurations<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, StakingConfigOf<T>, OptionQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Enable a protocol staking configuration.
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
		/// * `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
		///   by `instance_id`.
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
		/// * `origin` the origin that signed this extrinsic. Can be anyone. by `instance_id`.
		/// * `instance_id` the ID of the NFT that represent our staked position.
		/// * `to` the account in which the rewards will be transferred.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim(
			origin: OriginFor<T>,
			instance_id: InstanceIdOf<T>,
			to: AccountIdOf<T>,
			strategy: ClaimStrategy,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// Only the owner is able to select an arbitrary `to` account.
			let nft_owner = T::get_protocol_nft_owner::<StakingNFTOf<T>>(&instance_id)?;
			let to = if nft_owner == who { to } else { nft_owner };
			<Self as Staking>::claim(&instance_id, &to, strategy)?;
			Ok(().into())
		}

		/// Tag the given NFT.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Can be anyone.
		/// * `instance_id` the ID of the NFT we want to tag.
		/// * `beneficiary` the beneficiary of the reward when claiming.
		///
		/// NOTE: if logic gate pass, no fee applied.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn tag(
			origin: OriginFor<T>,
			instance_id: InstanceIdOf<T>,
			beneficiary: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_tag(&who, &instance_id, &beneficiary)?;
			Ok(Pays::No.into())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_: T::BlockNumber) -> Weight {
			Self::update_epoch();
			0
		}
	}

	impl<T: Config> Pallet<T> {
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
			TotalSharesPending::<T>::iter().for_each(|(pk, pv)| {
				TotalShares::<T>::mutate(pk, |v| {
					*v = v.checked_add(pv).expect("impossible; qed");
				});
				TotalSharesPending::<T>::insert(pk, 0);
			});
		}

		pub(crate) fn now() -> Timestamp {
			T::Time::now().as_secs()
		}

		pub(crate) fn epoch_start() -> Timestamp {
			EpochStart::<T>::get().unwrap_or(0)
		}

		/// The chaos protocol account. Derived from the chaos pallet id.
		pub(crate) fn account_id(asset: &AssetIdOf<T>) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(asset)
		}

		pub(crate) fn get_config(
			asset: &AssetIdOf<T>,
		) -> Result<StakingConfigOf<T>, DispatchError> {
			StakingConfigurations::<T>::get(asset).ok_or(Error::<T>::NotConfigured.into())
		}

		/// Current reward indexes.
		pub(crate) fn current_collected_rewards(
			asset: &AssetIdOf<T>,
			config: &StakingConfigOf<T>,
		) -> CollectedRewardsOf<T> {
			config
				.rewards
				.iter()
				.copied()
				.map(|x| {
					(x, CollectedRewards::<T>::get(asset, &x).unwrap_or(CollectedReward::zero()))
				})
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("map does not alter the length; qed;")
		}

		pub(crate) fn do_tag(
			tagger: &AccountIdOf<T>,
			instance_id: &InstanceIdOf<T>,
			beneficiary: &AccountIdOf<T>,
		) -> DispatchResult {
			T::try_mutate_protocol_nft(instance_id, |nft: &mut StakingNFTOf<T>| -> DispatchResult {
				let now = Self::now();
				let epoch_start = Self::epoch_start();
				match (nft.state(now, epoch_start), &nft.tag) {
					(PositionState::Expired, None) => {
						let config = Self::get_config(&nft.asset)?;
						nft.tag = Some(StakingTag {
							tagger: tagger.clone(),
							beneficiary: beneficiary.clone(),
							collected_rewards: Self::current_collected_rewards(&nft.asset, &config),
						});
						Self::deposit_event(Event::<T>::Tagged {
							who: tagger.clone(),
							beneficiary: beneficiary.clone(),
						});
						Ok(())
					},
					_ => Err(Error::<T>::AlreadyTagged.into()),
				}
			})
		}

		pub(crate) fn collect_rewards(
			asset: &AssetIdOf<T>,
			shares: u128,
			total_shares: u128,
			previously_collected_rewards: &CollectedRewardsOf<T>,
			current_collected_rewards: impl Fn(&AssetIdOf<T>) -> Option<CollectedReward>,
			to: &AccountIdOf<T>,
			penalty: Option<Perbill>,
		) -> DispatchResult {
			let compute_reward =
				|delta_collected: CollectedReward| -> Result<BalanceOf<T>, DispatchError> {
					// Always increment but delta can't be > max supply <= u128.
					let normalized_delta = u128::try_from(delta_collected)?;
					let reward = safe_multiply_by_rational(normalized_delta, shares, total_shares)?;
					penalty
						.map(|p| p.mul_floor(reward))
						.unwrap_or(reward)
						.try_into()
						.map_err(|_| ArithmeticError::Overflow.into())
				};
			let rewards = previously_collected_rewards
				.iter()
				.map(|(reward_asset, previously_collected)| -> Result<(AssetIdOf<T>, BalanceOf<T>), DispatchError> {
					match current_collected_rewards(reward_asset) {
						Some(current_collected) => {
							let delta_collected = current_collected.saturating_sub(*previously_collected);
							let reward = compute_reward(delta_collected)?;
							Ok((*reward_asset, reward))
						},
						None => Ok((*reward_asset, Zero::zero())),
					}
				})
				.collect::<Result<Vec<_>, _>>()?;
			let protocol_account = Self::account_id(asset);
			for (reward_asset, reward) in
				rewards.into_iter().filter(|(_, reward)| !reward.is_zero())
			{
				T::Assets::transfer(reward_asset, &protocol_account, to, reward, false)?;
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
			let config = Self::get_config(asset)?;
			// Make sure the asset has been registered as reward, otherwise we don't track it.
			ensure!(config.rewards.contains(&reward_asset), Error::<T>::RewardAssetDisabled);
			// Transfer the reward locally.
			let protocol_account = Self::account_id(asset);
			T::Assets::transfer(*reward_asset, from, &protocol_account, amount, keep_alive)?;
			// Increment the reward index, used to compute user rewards.
			CollectedRewards::<T>::try_mutate(asset, reward_asset, |entry| -> DispatchResult {
				let lifted_amount = CollectedReward::from(amount.saturated_into::<u128>());
				match entry {
					Some(collected_so_far) => {
						*collected_so_far = (*collected_so_far)
							.checked_add(lifted_amount)
							.ok_or(ArithmeticError::Overflow)?;
					},
					None => {
						*entry = Some(lifted_amount);
					},
				}
				Ok(())
			})?;
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
			let config = Self::get_config(asset)?;
			let reward_multiplier = *config
				.duration_presets
				.get(&duration)
				.ok_or(Error::<T>::InvalidDurationPreset)?;
			// Acquire protocol asset from user.
			let protocol_account = Self::account_id(asset);
			T::Assets::transfer(*asset, from, &protocol_account, amount, keep_alive)?;
			// Actually create the NFT representing the user position.
			let collected_rewards = Self::current_collected_rewards(asset, &config);
			let now = Self::now();
			let nft: StakingNFTOf<T> = StakingNFT {
				asset: *asset,
				stake: amount,
				lock_date: now,
				lock_duration: duration,
				collected_rewards,
				reward_multiplier,
				tag: None,
			};
			let instance_id = T::mint_protocol_nft(from, &nft)?;
			// Increment pending shares.
			TotalSharesPending::<T>::try_mutate(asset, |total_shares| -> DispatchResult {
				*total_shares = total_shares.safe_add(&nft.shares())?;
				Ok(())
			})?;
			// Trigger event
			Self::deposit_event(Event::<T>::Staked {
				who: from.clone(),
				stake: amount,
				nft: instance_id,
			});
			Ok(instance_id)
		}

		fn unstake(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult {
			let nft = T::get_protocol_nft::<StakingNFTOf<T>>(instance_id)?;
			let config = Self::get_config(&nft.asset)?;
			let protocol_account = Self::account_id(&nft.asset);
			let (stake, penalty) = match nft.state(Self::now(), Self::epoch_start()) {
				PositionState::Pending => {
					// When the position is not being rewarded yet, remove from the pending
					// amount.
					TotalSharesPending::<T>::try_mutate(
						nft.asset,
						|total_shares| -> DispatchResult {
							*total_shares = total_shares.safe_sub(&nft.shares())?;
							Ok(())
						},
					)?;
					Ok((nft.stake, BalanceOf::<T>::zero()))
				},
				PositionState::LockedRewarding => {
					// Decrement total shares.
					TotalShares::<T>::try_mutate(nft.asset, |total_shares| -> DispatchResult {
						*total_shares = total_shares.safe_sub(&nft.shares())?;
						Ok(())
					})?;
					nft.penalize(config.early_unstake_penalty)
				},
				PositionState::Expired => {
					// Decrement total shares.
					TotalShares::<T>::try_mutate(nft.asset, |total_shares| -> DispatchResult {
						*total_shares = total_shares.safe_sub(&nft.shares())?;
						Ok(())
					})?;
					Ok((nft.stake, BalanceOf::<T>::zero()))
				},
			}?;
			// NOTE(hussein-aitlahcen): no need to keep protocol account alive.
			T::Assets::transfer(nft.asset, &protocol_account, &to, stake, false)?;
			if penalty > BalanceOf::<T>::zero() {
				// Move penalty to configured beneficiary
				T::Assets::transfer(
					nft.asset,
					&protocol_account,
					&config.penalty_beneficiary,
					penalty,
					false,
				)?;
			}
			// Actually burn the NFT from the storage.
			T::burn_protocol_nft::<StakingNFTOf<T>>(instance_id)?;
			// Trigger event
			Self::deposit_event(Event::<T>::Unstaked {
				to: to.clone(),
				stake: nft.stake,
				penalty,
				nft: *instance_id,
			});
			Ok(())
		}

		fn claim(
			instance_id: &Self::InstanceId,
			to: &Self::AccountId,
			strategy: ClaimStrategy,
		) -> DispatchResult {
			T::try_mutate_protocol_nft(instance_id, |nft: &mut StakingNFTOf<T>| -> DispatchResult {
				let now = Self::now();
				let epoch_start = Self::epoch_start();
				let shares = nft.shares();
				let total_shares = Self::total_shares(nft.asset);
				let config = Self::get_config(&nft.asset)?;
				match (nft.state(now, epoch_start), strategy, &mut nft.tag) {
					// The position will start to be rewarded after the current epoch ended.
					(PositionState::Pending, _, _) => Err(Error::<T>::CannotClaimIfPending),
					// The user is claiming on an expired, tagged nft.
					// Move the apppropriate reward to the tagger.
					(PositionState::Expired, ClaimStrategy::RestakeOnExpiry, Some(tag)) => {
						// NOTE(hussein-aitlahcen): make sure the position transition from
						// Expired to Rewarding by making the new lock date started at the
						// previous epoch.
						nft.lock_date = epoch_start - 1;
						Self::collect_rewards(
							&nft.asset,
							shares,
							total_shares,
							&nft.collected_rewards,
							|x| tag.collected_rewards.get(x).copied(),
							to,
							None,
						)?;
						Self::collect_rewards(
							&nft.asset,
							shares,
							total_shares,
							&tag.collected_rewards,
							|x| Self::collected_rewards(&nft.asset, x),
							&tag.beneficiary,
							Some(T::TagRewardPenatly::get()),
						)?;
						tag.collected_rewards =
							Self::current_collected_rewards(&nft.asset, &config);
						nft.collected_rewards = tag.collected_rewards.clone();
						Ok(())
					},
					// The user is claiming and asking for a lock refresh.
					(PositionState::Expired, ClaimStrategy::RestakeOnExpiry, None) => {
						// NOTE(hussein-aitlahcen): make sure the position transition from
						// Expired to Rewarding by making the new lock date started at the
						// previous epoch.
						nft.lock_date = epoch_start - 1;
						Self::collect_rewards(
							&nft.asset,
							shares,
							total_shares,
							&nft.collected_rewards,
							|x| Self::collected_rewards(&nft.asset, x),
							to,
							None,
						)?;
						nft.collected_rewards =
							Self::current_collected_rewards(&nft.asset, &config);
						Ok(())
					},
					// Regardless of the claiming strategy, the user need to unstake. Otherwise
					// extended rewards will go to the tagger.
					(PositionState::Expired, ClaimStrategy::Canonical, Some(tag)) => {
						// On first claim between expiry and tag, the user is able to claim this
						// portion of reward.
						Self::collect_rewards(
							&nft.asset,
							shares,
							total_shares,
							&nft.collected_rewards,
							|x| tag.collected_rewards.get(x).copied(),
							to,
							None,
						)?;
						Self::collect_rewards(
							&nft.asset,
							shares,
							total_shares,
							&tag.collected_rewards,
							|x| Self::collected_rewards(&nft.asset, x),
							&tag.beneficiary,
							Some(T::TagRewardPenatly::get()),
						)?;
						// Adjust the cursors such that any reward coming after the first tag will
						// always go to the tagger.
						tag.collected_rewards =
							Self::current_collected_rewards(&nft.asset, &config);
						nft.collected_rewards = tag.collected_rewards.clone();
						Ok(())
					},
					// The user is trying ot claim on an expired nft, without refreshing the lock.
					(PositionState::Expired, ClaimStrategy::Canonical, None) =>
						Err(Error::<T>::ClaimRequireRestake),
					// The user is claiming rewards on a rewarding state nft.
					(PositionState::LockedRewarding, _, _) => {
						Self::collect_rewards(
							&nft.asset,
							shares,
							total_shares,
							&nft.collected_rewards,
							|x| Self::collected_rewards(&nft.asset, x),
							to,
							None,
						)?;
						nft.collected_rewards =
							Self::current_collected_rewards(&nft.asset, &config);
						Ok(())
					},
				}?;
				Ok(())
			})
		}
	}
}
