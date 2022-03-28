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
		chaos::{ChaosProtocol, ChaosStakingNFT},
		financial_nft::{DefaultFinancialNFTProtocol, FinancialNFTProtocol},
		oracle::Oracle,
		time::Timestamp,
	};
	use frame_support::{
		pallet_prelude::*,
		storage::{bounded_btree_map::BoundedBTreeMap, bounded_btree_set::BoundedBTreeSet},
		traits::{
			fungible::{Inspect as FungibleInspect, Transfer as FungibleTransfer},
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
	use sp_core::{U256, U512};
	use sp_runtime::{
		traits::{AccountIdConversion, Zero},
		ArithmeticError, Permill, SaturatedConversion,
	};
	use sp_std::collections::btree_map::BTreeMap;

	pub(crate) type RewardIndex = U256;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type InstanceIdOf<T> = <T as FinancialNFTProtocol>::InstanceId;
	pub(crate) type MaxRewardAssetsOf<T> = <T as Config>::MaxRewardAssets;
	pub(crate) type MaxStakingPresetsOf<T> = <T as Config>::MaxStakingPresets;
	pub(crate) type ChaosStateOf<T> =
		BoundedBTreeMap<AssetIdOf<T>, RewardIndex, MaxRewardAssetsOf<T>>;
	pub(crate) type ChaosStakingNFTOf<T> = ChaosStakingNFT<BalanceOf<T>, ChaosStateOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user staked his Chaos. Yield a NFT represeting his position.
		Staked { who: AccountIdOf<T>, stake: BalanceOf<T>, nft: InstanceIdOf<T> },
		/// A user unstaked his Chaos.
		Unstaked {
			to: AccountIdOf<T>,
			stake: BalanceOf<T>,
			penalty: BalanceOf<T>,
			nft: InstanceIdOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidStakingPreset,
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

		type Balance: Balance + TryFrom<U512>;

		/// The underlying currency system.
		type Assets: FungiblesInspect<
				AccountIdOf<Self>,
				AssetId = AssetIdOf<Self>,
				Balance = BalanceOf<Self>,
			> + FungiblesMutate<AccountIdOf<Self>>
			+ FungiblesTransfer<AccountIdOf<Self>>;

		/// The currency system with constant asset (Chaos).
		type ChaosAsset: FungibleInspect<AccountIdOf<Self>, Balance = BalanceOf<Self>>
			+ FungiblesMutate<AccountIdOf<Self>>
			+ FungibleTransfer<AccountIdOf<Self>>;

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

		/// The maximum number of reward assets Chaos can handle.
		#[pallet::constant]
		type MaxRewardAssets: Get<u32>;
	}

	#[pallet::type_value]
	pub fn TotalSharesOnEmpty<T: Config>() -> U512 {
		U512::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn total_shares)]
	pub type TotalShares<T: Config> = StorageValue<_, U512, ValueQuery, TotalSharesOnEmpty<T>>;

	#[pallet::type_value]
	pub fn EarlyUnstakePenaltyOnEmpty<T: Config>() -> Permill {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn early_unstake_penalty)]
	pub type EarlyUnstakePenalty<T: Config> =
		StorageValue<_, Permill, ValueQuery, EarlyUnstakePenaltyOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn reward_indexes)]
	pub type RewardIndexes<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, RewardIndex, OptionQuery>;

	#[pallet::type_value]
	pub fn RewardAssetsOnEmpty<T: Config>() -> BoundedBTreeSet<AssetIdOf<T>, MaxRewardAssetsOf<T>> {
		BoundedBTreeSet::<AssetIdOf<T>, MaxRewardAssetsOf<T>>::new()
	}

	#[pallet::storage]
	#[pallet::getter(fn reward_assets)]
	pub type RewardAssets<T: Config> = StorageValue<
		_,
		BoundedBTreeSet<AssetIdOf<T>, MaxRewardAssetsOf<T>>,
		ValueQuery,
		RewardAssetsOnEmpty<T>,
	>;

	#[pallet::type_value]
	pub fn StakingPresetsOnEmpty<T: Config>() -> BoundedBTreeSet<Timestamp, MaxStakingPresetsOf<T>>
	{
		BoundedBTreeSet::<Timestamp, MaxStakingPresetsOf<T>>::new()
	}

	#[pallet::storage]
	#[pallet::getter(fn staking_presets)]
	pub type StakingPresets<T: Config> = StorageValue<
		_,
		BoundedBTreeSet<Timestamp, MaxStakingPresetsOf<T>>,
		ValueQuery,
		StakingPresetsOnEmpty<T>,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set enabled reward assets.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic, must be `T::AdminOrigin`.
		/// * `reward_assets` the set of enabled reward assets.
		#[pallet::weight(10_000)]
		pub fn set_reward_assets(
			origin: OriginFor<T>,
			reward_assets: BoundedBTreeSet<AssetIdOf<T>, MaxRewardAssetsOf<T>>,
		) -> DispatchResultWithPostInfo {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			RewardAssets::<T>::set(reward_assets);
			Ok(().into())
		}

		/// Set enabled staking presets.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic, must be `T::AdminOrigin`.
		/// * `staking_presets` the set of enabled staking presets.
		#[pallet::weight(10_000)]
		pub fn set_staking_presets(
			origin: OriginFor<T>,
			staking_presets: BoundedBTreeSet<Timestamp, MaxStakingPresetsOf<T>>,
		) -> DispatchResultWithPostInfo {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			StakingPresets::<T>::set(staking_presets);
			Ok(().into())
		}

		/// Overwrite the unlock penalty.
		/// The penalty will be cut off the stake asset, Chaos in this case.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic, must be `T::AdminOrigin`.
		/// * `penalty` the penalty applied to users unstaking early.
		#[pallet::weight(10_000)]
		pub fn set_unlock_penalty(
			origin: OriginFor<T>,
			penalty: Permill,
		) -> DispatchResultWithPostInfo {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			EarlyUnstakePenalty::<T>::set(penalty);
			Ok(().into())
		}

		/// Stake an amount of Chaos tokens. Generating an NFT for the staked position.
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
			amount: BalanceOf<T>,
			duration: Timestamp,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let account_id = ensure_signed(origin)?;
			<Self as ChaosProtocol>::stake(&account_id, amount, duration, keep_alive)?;
			Ok(().into())
		}

		/// Unstake an amount of Chaos tokens.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
		///   by `instance_id`.
		/// * `to` the account in which the rewards will be transferred before unstaking.
		/// * `instance_id` the ID of the NFT that represent our staked position.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn unstake(
			origin: OriginFor<T>,
			to: AccountIdOf<T>,
			instance_id: InstanceIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			T::ensure_protocol_nft_owner::<ChaosStakingNFTOf<T>>(&owner, &instance_id)?;
			<Self as ChaosProtocol>::unstake(&to, &instance_id)?;
			Ok(().into())
		}

		/// Claim the current available rewards.
		///
		/// Arguments
		///
		/// * `origin` the origin that signed this extrinsic. Must be the owner of the NFT targeted
		///   by `instance_id`.
		/// * `to` the account in which the rewards will be transferred.
		/// * `instance_id` the ID of the NFT that represent our staked position.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim(
			origin: OriginFor<T>,
			to: AccountIdOf<T>,
			instance_id: InstanceIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			T::ensure_protocol_nft_owner::<ChaosStakingNFTOf<T>>(&owner, &instance_id)?;
			<Self as ChaosProtocol>::claim(&to, &instance_id)?;
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The chaos protocol account. Derived from the chaos pallet id.
		pub(crate) fn account_id() -> AccountIdOf<T> {
			T::PalletId::get().into_account()
		}

		/// Current reward indexes.
		pub(crate) fn current_reward_indexes() -> ChaosStateOf<T> {
			RewardAssets::<T>::get()
				.into_iter()
				.map(|x| (x, RewardIndexes::<T>::get(&x).unwrap_or(RewardIndex::zero())))
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("map does not alter the length; qed;")
		}
	}

	impl<T: Config> ChaosProtocol for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type InstanceId = T::InstanceId;

		fn stake(
			from: &Self::AccountId,
			amount: Self::Balance,
			duration: Timestamp,
			keep_alive: bool,
		) -> Result<Self::InstanceId, DispatchError> {
			// Make sure the duration is a valid preset.
			ensure!(Self::staking_presets().contains(&duration), Error::<T>::InvalidStakingPreset);

			// Acquire Chaos from user.
			let chaos_account = Self::account_id();
			T::ChaosAsset::transfer(from, &chaos_account, amount, keep_alive)?;

			// Actually create the NFT representing the user position.
			let reward_indexes = Self::current_reward_indexes();
			let now = T::Time::now();
			let nft = ChaosStakingNFT {
				stake: amount,
				reward_indexes,
				lock_date: now.as_secs(),
				lock_duration: duration,
			};
			let instance_id = T::mint_protocol_nft(from, &nft)?;

			// Increment total shares.
			TotalShares::<T>::try_mutate(|x| {
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

		fn unstake(
			to: &Self::AccountId,
			instance_id: &Self::InstanceId,
		) -> DispatchResult {
			// Make sure we execute a final claim before unstaking.
			<Self as ChaosProtocol>::claim(to, instance_id)?;

			let nft = T::get_protocol_nft::<ChaosStakingNFTOf<T>>(instance_id)?;

			let now = T::Time::now();

			// Possibly penalize the staked asset, Chaos in this case.
			let (penalized_stake, penalty_amount) =
				nft.penalize_early_unstake_amount(now.as_secs(), EarlyUnstakePenalty::<T>::get())?;

			// Transfer back the (possibly penalized) staked amount.
			let chaos_account = Self::account_id();
      // NOTE(hussein-aitlahcen): no need to keep protocol account alive.
			T::ChaosAsset::transfer(&chaos_account, &to, penalized_stake, false)?;

			// Decrement total shares.
			// TODO: decrement by penalized only????? the protocol would own this fraction
			// indefinitely
			TotalShares::<T>::try_mutate(|x| {
				x.checked_sub(nft.stake.saturated_into::<u128>().into())
					.ok_or(ArithmeticError::Underflow)
			})?;

			// Actually burn the NFT from the storage.
			T::burn_protocol_nft::<ChaosStakingNFTOf<T>>(instance_id)?;

			// Trigger event
			Self::deposit_event(Event::<T>::Unstaked {
				to: to.clone(),
				stake: nft.stake,
				penalty: penalty_amount,
				nft: *instance_id,
			});

			Ok(())
		}

		fn transfer_reward(
			from: &Self::AccountId,
			asset: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			// Make sure the asset has been registered as reward, otherwise we don't track it.
			ensure!(RewardAssets::<T>::get().contains(&asset), Error::<T>::RewardAssetDisabled);

			// Transfer the reward locally.
			let chaos_account = Self::account_id();
			T::Assets::transfer(asset, from, &chaos_account, amount, keep_alive)?;

			// Increment the reward index, used to compute user rewards.
			RewardIndexes::<T>::try_mutate(asset, |x| {
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

		fn claim(
			to: &Self::AccountId,
			instance_id: &Self::InstanceId,
		) -> DispatchResult {
			T::try_mutate_protocol_nft(
				instance_id,
				|nft: &mut ChaosStakingNFTOf<T>| -> Result<(), DispatchError> {
					// NOTE(hussein-aitlahcen): user is still able to claim even if his position
					// 'expired', do we allow that?

					let share = U512::from(nft.stake.saturated_into::<u128>());
					let total_shares = TotalShares::<T>::get();

					// TODO(hussein-aitlahcen): extract pure maths to their own functions
					let compute_reward = |delta_index| -> Result<BalanceOf<T>, DispatchError> {
						U512::from(delta_index)
							.checked_mul(share)
							.and_then(|x| x.checked_div(total_shares))
							.ok_or(ArithmeticError::Overflow.into())
							.and_then(|x| {
								TryFrom::<U512>::try_from(x)
									.map_err(|_| ArithmeticError::Overflow.into())
							})
					};

					let rewards = nft
				    .reward_indexes
				    .iter()
				    .map(|(asset, index)| -> Result<(AssetIdOf<T>, BalanceOf<T>), DispatchError> {
					    match RewardIndexes::<T>::get(&asset) {
						    Some(current_index) => {
							    let delta_index = current_index.saturating_sub(*index);
							    let reward = compute_reward(delta_index)?;
							    Ok((*asset, reward))
						    },
						    None => Ok((*asset, Zero::zero())),
					    }
				    })
				    .collect::<Result<Vec<_>, _>>()?;

					let chaos_account = Self::account_id();
					for (asset, reward) in rewards {
						T::Assets::transfer(asset, &chaos_account, to, reward, false)?;
					}

					// NOTE(hussein-aitahcen): the reward computation is based on the index delta,
					// hence we need to update the indexes after having claimed the rewards.
					nft.reward_indexes = Self::current_reward_indexes();

					Ok(())
				},
			)
		}
	}
}
