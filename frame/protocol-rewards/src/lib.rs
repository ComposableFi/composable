//! Overview
//! Allows to add new assets internally. User facing mutating API is provided by other pallets.
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

#[frame_support::pallet]
pub mod pallet {
	use composable_traits::{
		financial_nft::{DefaultFinancialNFTProtocol, FinancialNFTProtocol},
		oracle::Oracle,
		protocol_rewards::{
			ProtocolReward, ProtocolStaking, ProtocolStakingConfig, ProtocolStakingNFT,
		},
		time::Timestamp,
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
	use sp_core::U256;
	use sp_runtime::{
		traits::{AccountIdConversion, Zero},
		ArithmeticError, SaturatedConversion,
	};
	use sp_std::collections::btree_map::BTreeMap;

	pub(crate) type RewardIndex = U256;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type InstanceIdOf<T> = <T as FinancialNFTProtocol>::InstanceId;
	pub(crate) type MaxRewardAssetsOf<T> = <T as Config>::MaxRewardAssets;
	pub(crate) type MaxStakingPresetsOf<T> = <T as Config>::MaxStakingPresets;
	pub(crate) type ProtocolRewardStateOf<T> =
		BoundedBTreeMap<AssetIdOf<T>, RewardIndex, MaxRewardAssetsOf<T>>;
	pub(crate) type ProtocolStakingNFTOf<T> =
		ProtocolStakingNFT<AssetIdOf<T>, BalanceOf<T>, ProtocolRewardStateOf<T>>;
	pub(crate) type ProtocolStakingConfigOf<T> = ProtocolStakingConfig<
		AccountIdOf<T>,
		BoundedBTreeSet<Timestamp, MaxStakingPresetsOf<T>>,
		BoundedBTreeSet<AssetIdOf<T>, MaxRewardAssetsOf<T>>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An asset has been configured for staking.
		Configured { asset: AssetIdOf<T>, configuration: ProtocolStakingConfigOf<T> },
		/// A user staked his protocol asset. Yield a NFT represeting his position.
		Staked { who: AccountIdOf<T>, stake: BalanceOf<T>, nft: InstanceIdOf<T> },
		/// A user unstaked his protocol asset.
		Unstaked {
			to: AccountIdOf<T>,
			stake: BalanceOf<T>,
			penalty: BalanceOf<T>,
			nft: InstanceIdOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		NotConfigured,
		InvalidDurationPreset,
		TooManyRewardAssets,
		RewardAssetDisabled,
	}

	#[pallet::config]
	pub trait Config:
		frame_system::Config + DefaultFinancialNFTProtocol<AccountId = AccountIdOf<Self>>
	{
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The ID that uniquely identify an asset.
		type AssetId: AssetId + Ord;

		type Balance: Balance + TryFrom<U256>;

		/// The underlying currency system.
		type Assets: FungiblesInspect<
				AccountIdOf<Self>,
				AssetId = AssetIdOf<Self>,
				Balance = BalanceOf<Self>,
			> + FungiblesMutate<AccountIdOf<Self>>
			+ FungiblesTransfer<AccountIdOf<Self>>;

		/// The time provider.
		type Time: UnixTime;

		/// The oracle implementation to check for priceable assets.
		type Oracle: Oracle<AssetId = AssetIdOf<Self>, Balance = BalanceOf<Self>>;

		/// The admin origin, allowed to update sensitive values such as the unlock penalty.
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// The pallet id, used to uniquely identify this pallet.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The maximum number of staking duration presets.
		#[pallet::constant]
		type MaxStakingPresets: Get<u32>;

		/// The maximum number of reward assets protocol asset can handle.
		#[pallet::constant]
		type MaxRewardAssets: Get<u32>;
	}

	#[pallet::type_value]
	pub fn TotalSharesOnEmpty<T: Config>() -> U256 {
		U256::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn total_shares)]
	pub type TotalShares<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, U256, ValueQuery, TotalSharesOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn reward_indexes)]
	pub type RewardIndexes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetIdOf<T>,
		Blake2_128Concat,
		AssetIdOf<T>,
		RewardIndex,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn staking_configurations)]
	pub type StakingConfigurations<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, ProtocolStakingConfigOf<T>, OptionQuery>;

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
		/// * `origin` the origin that signed this extrinsic, must be `T::AdminOrigin`.
		/// * `staking_configuration` the staking configuration for the given protocol `asset`.
		#[pallet::weight(10_000)]
		pub fn configure(
			origin: OriginFor<T>,
			asset: AssetIdOf<T>,
			config: ProtocolStakingConfigOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			StakingConfigurations::<T>::insert(asset, config);
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
			<Self as ProtocolStaking>::stake(&asset, &from, amount, duration, keep_alive)?;
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
			T::ensure_protocol_nft_owner::<ProtocolStakingNFTOf<T>>(&owner, &instance_id)?;
			<Self as ProtocolStaking>::unstake(&instance_id, &to)?;
			Ok(().into())
		}

		/// Claim the current available rewards.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
		///   by `instance_id`.
		/// * `instance_id` the ID of the NFT that represent our staked position.
		/// * `to` the account in which the rewards will be transferred.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim(
			origin: OriginFor<T>,
			instance_id: InstanceIdOf<T>,
			to: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			T::ensure_protocol_nft_owner::<ProtocolStakingNFTOf<T>>(&owner, &instance_id)?;
			<Self as ProtocolStaking>::claim(&instance_id, &to)?;
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The chaos protocol account. Derived from the chaos pallet id.
		pub(crate) fn account_id(asset: &AssetIdOf<T>) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(asset)
		}

		pub(crate) fn get_config(
			asset: &AssetIdOf<T>,
		) -> Result<ProtocolStakingConfigOf<T>, DispatchError> {
			StakingConfigurations::<T>::get(asset).ok_or(Error::<T>::NotConfigured.into())
		}

		/// Current reward indexes.
		pub(crate) fn current_reward_indexes(
			asset: &AssetIdOf<T>,
			config: &ProtocolStakingConfigOf<T>,
		) -> ProtocolRewardStateOf<T> {
			config
				.rewards
				.iter()
				.copied()
				.map(|x| (x, RewardIndexes::<T>::get(asset, &x).unwrap_or(RewardIndex::zero())))
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("map does not alter the length; qed;")
		}
	}

	impl<T: Config> ProtocolReward for Pallet<T> {
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
			RewardIndexes::<T>::try_mutate(asset, reward_asset, |x| {
				let lifted_amount = RewardIndex::from(amount.saturated_into::<u128>());
				match x {
					Some(index) => {
						*index =
							(*index).checked_add(lifted_amount).ok_or(ArithmeticError::Overflow)?;
					},
					None => {
						*x = Some(lifted_amount);
					},
				}
				Ok(())
			})
		}
	}

	impl<T: Config> ProtocolStaking for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type InstanceId = T::InstanceId;

		fn stake(
			asset: &Self::AssetId,
			from: &Self::AccountId,
			amount: Self::Balance,
			duration: Timestamp,
			keep_alive: bool,
		) -> Result<Self::InstanceId, DispatchError> {
			let config = Self::get_config(asset)?;

			// Make sure the duration is a valid preset.
			ensure!(config.duration_presets.contains(&duration), Error::<T>::InvalidDurationPreset);

			// Acquire protocol asset from user.
			let protocol_account = Self::account_id(asset);
			T::Assets::transfer(*asset, from, &protocol_account, amount, keep_alive)?;

			// Actually create the NFT representing the user position.
			let reward_indexes = Self::current_reward_indexes(asset, &config);
			let now = T::Time::now();
			let nft = ProtocolStakingNFT {
				asset,
				stake: amount,
				reward_indexes,
				lock_date: now.as_secs(),
				lock_duration: duration,
			};
			let instance_id = T::mint_protocol_nft(from, &nft)?;

			// Increment total shares.
			TotalShares::<T>::try_mutate(asset, |x| {
				x.checked_add(amount.saturated_into::<u128>().into())
					.ok_or(ArithmeticError::Overflow)
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
			// Make sure we execute a final claim before unstaking.
			<Self as ProtocolStaking>::claim(instance_id, to)?;

			let nft = T::get_protocol_nft::<ProtocolStakingNFTOf<T>>(instance_id)?;

			let now = T::Time::now();

			let config = Self::get_config(&nft.asset)?;

			// Possibly penalize the staked asset, protocol asset in this case.
			let (penalized_stake, penalty_amount) =
				nft.penalize_early_unstake_amount(now.as_secs(), config.early_unstake_penalty)?;

			// Transfer back the (possibly penalized) staked amount.
			let pallet_account = Self::account_id(&nft.asset);
			// NOTE(hussein-aitlahcen): no need to keep protocol account alive.
			T::Assets::transfer(nft.asset, &pallet_account, &to, penalized_stake, false)?;

			// Move penalty to configured beneficiary
			T::Assets::transfer(
				nft.asset,
				&pallet_account,
				&config.penalty_beneficiary,
				penalty_amount,
				false,
			)?;

			// Decrement total shares.
			TotalShares::<T>::try_mutate(nft.asset, |x| {
				x.checked_sub(nft.stake.saturated_into::<u128>().into())
					.ok_or(ArithmeticError::Underflow)
			})?;

			// Actually burn the NFT from the storage.
			T::burn_protocol_nft::<ProtocolStakingNFTOf<T>>(instance_id)?;

			// Trigger event
			Self::deposit_event(Event::<T>::Unstaked {
				to: to.clone(),
				stake: nft.stake,
				penalty: penalty_amount,
				nft: *instance_id,
			});

			Ok(())
		}

		fn claim(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult {
			T::try_mutate_protocol_nft(instance_id, |nft: &mut ProtocolStakingNFTOf<T>| {
				let config = Self::get_config(&nft.asset)?;

				// NOTE(hussein-aitlahcen): user is still able to claim even if his position
				// 'expired', do we allow that?

				let share = U256::from(nft.stake.saturated_into::<u128>());
				let total_shares = TotalShares::<T>::get(nft.asset);

				// TODO(hussein-aitlahcen): extract pure maths to their own functions
				let compute_reward = |delta_index: U256| -> Result<BalanceOf<T>, DispatchError> {
					delta_index
						.checked_mul(share)
						.and_then(|x| x.checked_div(total_shares))
						.ok_or(ArithmeticError::Overflow.into())
						.and_then(|x| {
							TryFrom::<U256>::try_from(x)
								.map_err(|_| ArithmeticError::Overflow.into())
						})
				};

				let rewards = nft
					.reward_indexes
					.iter()
					.map(|(reward_asset, index)| -> Result<(AssetIdOf<T>, BalanceOf<T>), DispatchError> {
						match RewardIndexes::<T>::get(nft.asset, &reward_asset) {
							Some(current_index) => {
								let delta_index = current_index.saturating_sub(*index);
								let reward = compute_reward(delta_index)?;
								Ok((*reward_asset, reward))
							},
							None => Ok((*reward_asset, Zero::zero())),
						}
					})
					.collect::<Result<Vec<_>, _>>()?;

				let pallet_account = Self::account_id(&nft.asset);
				for (reward_asset, reward) in rewards {
					T::Assets::transfer(reward_asset, &pallet_account, to, reward, false)?;
				}

				// NOTE(hussein-aitahcen): the reward computation is based on the index delta,
				// hence we need to update the indexes after having claimed the rewards.
				nft.reward_indexes = Self::current_reward_indexes(&nft.asset, &config);

				Ok(())
			})
		}
	}
}
