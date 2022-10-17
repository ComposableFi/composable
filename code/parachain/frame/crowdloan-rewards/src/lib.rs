/*
Crowdloan rewards pallet used by contributors to claim their rewards.

A user is able to claim rewards once it has an associated account. Associating
an account using the `associate` extrinsic automatically yield the upfront
liquidity (% of the vested reward). The rest of the reward can be claimed every
`VestingStep` starting at the timestamp when the pallet was initialized
using the `initialize` extrinsic.

Proof to provide when associating a reward account:
```haskell
proof = sign (concat prefix (hex reward_account))
```

Reference for proof mechanism: https://github.com/paritytech/polkadot/blob/master/runtime/common/src/claims.rs
*/

#![doc = include_str!("../README.md")]
//! ## Pallet Modules
//! * [`Config`]
//! * [`Pallet`]

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
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
	unused_extern_crates
)]

pub use pallet::*;

pub mod models;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::models::{Proof, RemoteAccount, Reward};
	use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_support::{
		math::safe::{SafeAdd, SafeSub},
		types::{EcdsaSignature, EthereumAddress},
	};
	use frame_support::{
		dispatch::PostDispatchInfo,
		pallet_prelude::*,
		traits::{
			fungible::{Inspect, Mutate, Transfer},
			tokens::WithdrawReasons,
			LockIdentifier, LockableCurrency, Time,
		},
		transactional, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::keccak_256;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32Bit, AtLeast32BitUnsigned, CheckedAdd, CheckedMul,
			CheckedSub, Convert, Saturating, Verify, Zero,
		},
		AccountId32, DispatchErrorWithPostInfo, MultiSignature, Perbill,
	};
	use sp_std::vec::Vec;

	pub type MomentOf<T> = <T as Config>::Moment;
	pub type RemoteAccountOf<T> = RemoteAccount<<T as Config>::RelayChainAccountId>;
	pub type RewardOf<T> = Reward<<T as Config>::Balance, MomentOf<T>>;
	pub type VestingPeriodOf<T> = MomentOf<T>;
	pub type RewardAmountOf<T> = <T as Config>::Balance;
	pub type ProofOf<T> = Proof<<T as Config>::RelayChainAccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The crowdloan has been initialized or set to initialize at some time.
		Initialized { at: MomentOf<T> },
		/// A claim has been made.
		Claimed {
			remote_account: RemoteAccountOf<T>,
			reward_account: T::AccountId,
			amount: T::Balance,
		},
		/// A remote account has been associated with a reward account.
		Associated { remote_account: RemoteAccountOf<T>, reward_account: T::AccountId },
		/// The crowdloan was successfully initialized, but with excess funds that won't be
		/// claimed.
		OverFunded { excess_funds: T::Balance },
		/// A portion of rewards have been unlocked and future claims will not have locks
		RewardsUnlocked { at: MomentOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		NotInitialized,
		AlreadyInitialized,
		BackToTheFuture,
		RewardsNotFunded,
		InvalidProof,
		InvalidClaim,
		NothingToClaim,
		NotAssociated,
		AlreadyAssociated,
		NotClaimableYet,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[allow(missing_docs)]
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

		/// The RewardAsset used to transfer the rewards.
		type RewardAsset: Inspect<Self::AccountId, Balance = Self::Balance>
			+ Transfer<Self::AccountId, Balance = Self::Balance>
			+ Mutate<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Balance = Self::Balance>;

		/// Type used to express timestamps.
		type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen + FullCodec;

		/// The time provider.
		type Time: Time<Moment = Self::Moment>;

		/// The origin that is allowed to `initialize` the pallet.
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// A conversion function from `Self::Moment` to `Self::Balance`
		type Convert: Convert<Self::Moment, Self::Balance>;

		/// The relay chain account id.
		type RelayChainAccountId: Parameter
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ Into<AccountId32>
			+ Ord;

		/// The upfront liquidity unlocked at first claim.
		#[pallet::constant]
		type InitialPayment: Get<Perbill>;

		/// The percentage of excess funds required to trigger the `OverFunded` event.
		#[pallet::constant]
		type OverFundedThreshold: Get<Perbill>;

		/// The time you have to wait to unlock another part of your reward.
		#[pallet::constant]
		type VestingStep: Get<MomentOf<Self>>;

		/// The arbitrary prefix used for the proof.
		#[pallet::constant]
		type Prefix: Get<&'static [u8]>;

		/// The implementation of extrinsics weight.
		type WeightInfo: WeightInfo;

		/// The unique identifier of this pallet.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The unique identifier for locks maintained by this pallet.
		#[pallet::constant]
		type LockId: Get<LockIdentifier>;

		/// If claimed amounts should be locked by the pallet
		#[pallet::constant]
		type LockByDefault: Get<bool>;
	}

	#[pallet::storage]
	pub type Rewards<T: Config> =
		StorageMap<_, Blake2_128Concat, RemoteAccountOf<T>, RewardOf<T>, OptionQuery>;

	/// The total amount of rewards to be claimed.
	#[pallet::storage]
	#[pallet::getter(fn total_rewards)]
	// Absence of total rewards is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type TotalRewards<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	/// The rewards claimed so far.
	#[pallet::storage]
	#[pallet::getter(fn claimed_rewards)]
	// Absence of claimed rewards is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type ClaimedRewards<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	/// The total number of contributors.
	#[pallet::storage]
	#[pallet::getter(fn total_contributors)]
	// Absence of total contributors is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type TotalContributors<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// The timestamp at which the users are able to claim their rewards.
	#[pallet::storage]
	// REVIEW(connor): Can we change this getter without breaking a lot of other things?
	#[pallet::getter(fn vesting_block_start)]
	pub type VestingTimeStart<T: Config> = StorageValue<_, MomentOf<T>, OptionQuery>;

	/// Associations of reward accounts to remote accounts.
	#[pallet::storage]
	#[pallet::getter(fn associations)]
	pub type Associations<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, RemoteAccountOf<T>, OptionQuery>;

	/// If set, new locks will not be added to claims
	#[pallet::storage]
	#[pallet::getter(fn remove_reward_locks)]
	pub type RemoveRewardLocks<T: Config> = StorageValue<_, (), OptionQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initialize the pallet at the current timestamp.
		#[pallet::weight(<T as Config>::WeightInfo::initialize(TotalContributors::<T>::get()))]
		#[transactional]
		pub fn initialize(origin: OriginFor<T>) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			let now = T::Time::now();
			Self::do_initialize(now)
		}

		/// Initialize the pallet at the given timestamp.
		#[pallet::weight(<T as Config>::WeightInfo::initialize(TotalContributors::<T>::get()))]
		#[transactional]
		pub fn initialize_at(origin: OriginFor<T>, at: MomentOf<T>) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_initialize(at)
		}

		/// Populate pallet by adding more rewards.
		///
		/// Each index in the rewards vector should contain: `remote_account`, `reward_account`,
		/// `vesting_period`.
		///
		/// Can be called multiple times. If an remote account
		/// already has a reward, it will be replaced by the new reward value.
		///
		/// Can only be called before `initialize`.
		#[pallet::weight(<T as Config>::WeightInfo::populate(rewards.len() as _))]
		#[transactional]
		pub fn populate(
			origin: OriginFor<T>,
			rewards: Vec<(RemoteAccountOf<T>, RewardAmountOf<T>, VestingPeriodOf<T>)>,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_populate(rewards)
		}

		/// Associate a reward account. A valid proof has to be provided.
		/// This call also claim the first reward (a.k.a. the first payment, which is a % of the
		/// vested reward).
		/// If logic gate pass, no fees are applied.
		///
		/// The proof should be:
		/// ```haskell
		/// proof = sign (concat prefix (hex reward_account))
		/// ```
		#[pallet::weight(<T as Config>::WeightInfo::associate(TotalContributors::<T>::get()))]
		#[transactional]
		pub fn associate(
			origin: OriginFor<T>,
			reward_account: T::AccountId,
			proof: ProofOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;
			Self::do_associate(reward_account, proof)
		}

		/// Claim a reward from the associated reward account.
		/// A previous call to `associate` should have been made.
		/// If logic gate pass, no fees are applied.
		#[pallet::weight(<T as Config>::WeightInfo::claim(TotalContributors::<T>::get()))]
		#[transactional]
		pub fn claim(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let reward_account = ensure_signed(origin)?;
			let remote_account = Associations::<T>::try_get(&reward_account)
				.map_err(|_| Error::<T>::NotAssociated)?;
			let claimed = Self::do_claim(remote_account.clone(), &reward_account)?;
			Self::deposit_event(Event::Claimed { remote_account, reward_account, amount: claimed });
			Ok(Pays::No.into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::unlock_rewards_for(reward_accounts.len() as _))]
		pub fn unlock_rewards_for(
			origin: OriginFor<T>,
			reward_accounts: Vec<T::AccountId>,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_unlock(reward_accounts);
			Ok(())
		}
	}

	#[pallet::extra_constants]
	impl<T: Config> Pallet<T> {
		/// The AccountId of this pallet.
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}

	impl<T: Config> Pallet<T> {
		/// Initialize the Crowdloan at a given timestamp.
		///
		/// If the Crowdloan is over funded by more than the `OverFundedThreshold`, the `OverFunded`
		/// event will be emitted with the excess amount.
		///
		/// # Errors
		/// * `AlreadyInitialized` - The Crowdloan has already been scheduled to start at some time
		/// * `BackToTheFuture` - The given timestamp, `at`, is before the current time
		/// * `RewardsNotFunded` - The Crowdloan has not been funded with the minimum amount of
		///   funds to provide the total rewards
		pub(crate) fn do_initialize(at: MomentOf<T>) -> DispatchResult {
			ensure!(!VestingTimeStart::<T>::exists(), Error::<T>::AlreadyInitialized);

			let now = T::Time::now();
			ensure!(at >= now, Error::<T>::BackToTheFuture);

			let available_funds = T::RewardAsset::balance(&Self::account_id());
			let total_rewards = TotalRewards::<T>::get();
			let excess_funds = available_funds
				.checked_sub(&total_rewards)
				.ok_or(Error::<T>::RewardsNotFunded)?;

			if excess_funds > T::OverFundedThreshold::get().mul_floor(total_rewards) {
				Self::deposit_event(Event::OverFunded { excess_funds })
			}

			VestingTimeStart::<T>::set(Some(at));
			Self::deposit_event(Event::Initialized { at });

			Ok(())
		}

		/// Associates a reward account with some remote account provided by a proof. Calls
		/// `do_claim` to perform the first claim.
		///
		/// # Errors
		/// * `NotInitialized` - The Crowdloan has not been initialized yet
		/// * `NotClaimableYet` - The Crowdloan has been initialized, but the redemption period has
		///   not begun
		/// * `AlreadyAssociated` - The reward account has already been associated
		pub(crate) fn do_associate(
			reward_account: T::AccountId,
			proof: ProofOf<T>,
		) -> DispatchResultWithPostInfo {
			let now = T::Time::now();
			let enabled = VestingTimeStart::<T>::get().ok_or(Error::<T>::NotInitialized)? <= now;
			ensure!(enabled, Error::<T>::NotClaimableYet);
			let remote_account = get_remote_account::<T>(proof, &reward_account, T::Prefix::get())?;
			// NOTE(hussein-aitlahcen): this is also checked by the ValidateUnsigned implementation
			// of the pallet. theoretically useless, but 1:1 to make it clear
			ensure!(
				!Associations::<T>::contains_key(reward_account.clone()),
				Error::<T>::AlreadyAssociated
			);
			// NOTE(hussein-aitlahcen): very important to have a claim here because we do the
			// upfront payment, which will allow the user to execute transactions because they had 0
			// funds prior to this call.
			let claimed = Self::do_claim(remote_account.clone(), &reward_account)?;
			Associations::<T>::insert(reward_account.clone(), remote_account.clone());
			Self::deposit_event(Event::Associated {
				remote_account: remote_account.clone(),
				reward_account: reward_account.clone(),
			});
			Self::deposit_event(Event::Claimed { remote_account, reward_account, amount: claimed });
			Ok(Pays::No.into())
		}

		/// Populates the `Rewards` while updating `TotalRewards` and `TotalContributors`
		///
		/// If a reward already exits, the reward and respective totals will be updated to account
		/// for the new values.
		///
		/// # Errors
		/// * `AlreadyInitialized` - The crowdloan has been set to initialize, population may no
		///   longer commence
		/// * `ArithmeticError` - Overflow/Underflow detected while calculating totals
		pub(crate) fn do_populate(
			rewards: Vec<(RemoteAccountOf<T>, RewardAmountOf<T>, VestingPeriodOf<T>)>,
		) -> DispatchResult {
			ensure!(!VestingTimeStart::<T>::exists(), Error::<T>::AlreadyInitialized);

			let total_rewards: T::Balance = TotalRewards::<T>::get();
			let total_contributors: u32 = TotalContributors::<T>::get();

			let (total_rewards, total_contributors) = rewards.into_iter().try_fold(
				(total_rewards, total_contributors),
				|(total_rewards, total_contributors),
				 (remote_account, account_total, vesting_period)| {
					Rewards::<T>::try_mutate_exists::<_, _, DispatchError, _>(
						remote_account,
						|reward| match reward {
							Some(reward) => {
								let total_rewards = total_rewards
									.safe_sub(&reward.total)?
									.safe_add(&account_total)?;

								reward.total = account_total;
								reward.vesting_period = vesting_period;

								Ok((total_rewards, total_contributors))
							},
							None => {
								let total_rewards = total_rewards.safe_add(&account_total)?;
								let total_contributors = total_contributors.safe_add(&1)?;

								reward.replace(Reward {
									total: account_total,
									claimed: T::Balance::zero(),
									vesting_period,
								});

								Ok((total_rewards, total_contributors))
							},
						},
					)
				},
			)?;

			TotalRewards::<T>::set(total_rewards);
			TotalContributors::<T>::set(total_contributors);
			Ok(())
		}

		/// Do claim the reward for a given remote account, rewarding the `reward_account`.
		///
		/// # Errors
		/// * `NothingToClaim` - No rewards are available to claim at this time
		/// * `InvalidProof` - The user is not a contributor
		pub(crate) fn do_claim(
			remote_account: RemoteAccountOf<T>,
			reward_account: &T::AccountId,
		) -> Result<T::Balance, DispatchError> {
			Rewards::<T>::try_mutate(remote_account, |reward| {
				if let Some(reward) = reward {
					let should_have_claimed = should_have_claimed::<T>(reward)?;
					let available_to_claim = should_have_claimed.saturating_sub(reward.claimed);
					ensure!(available_to_claim > T::Balance::zero(), Error::<T>::NothingToClaim);

					reward.claimed = available_to_claim.saturating_add(reward.claimed);
					if T::LockByDefault::get() && !RemoveRewardLocks::<T>::exists() {
						T::RewardAsset::set_lock(
							T::LockId::get(),
							reward_account,
							reward.claimed,
							WithdrawReasons::all(),
						);
					}

					let funds_account = Self::account_id();
					// No need to keep the pallet account alive.
					T::RewardAsset::transfer(
						&funds_account,
						reward_account,
						available_to_claim,
						false,
					)?;

					ClaimedRewards::<T>::mutate(|x| *x = x.saturating_add(available_to_claim));

					Ok(available_to_claim)
				} else {
					Err(Error::<T>::InvalidProof.into())
				}
			})
		}

		/// Sets `RemoveRewardLocks`, removes `RewardAsset` locks on provided accounts, emits
		/// `RewardsUnlocked`.
		fn do_unlock(reward_accounts: Vec<T::AccountId>) {
			RemoveRewardLocks::<T>::put(());

			reward_accounts.iter().for_each(|reward_account| {
				T::RewardAsset::remove_lock(T::LockId::get(), reward_account);
			});

			Self::deposit_event(Event::<T>::RewardsUnlocked { at: T::Time::now() });
		}
	}

	/// The reward amount a user should have claimed until now.
	///
	/// # Errors
	/// * `NotInitialized` - The Crowdloan has not been initialized
	pub fn should_have_claimed<T: Config>(
		reward: &RewardOf<T>,
	) -> Result<T::Balance, DispatchError> {
		let start = VestingTimeStart::<T>::get().ok_or(Error::<T>::NotInitialized)?;
		let upfront_payment = T::InitialPayment::get().mul_floor(reward.total);

		let now = T::Time::now();
		// Current point in time
		let vesting_point = now.saturating_sub(start);
		if vesting_point >= reward.vesting_period {
			// If the user is claiming when the period is over, they should
			// probably have already claimed everything.
			Ok(reward.total)
		} else {
			let vesting_step = T::VestingStep::get();
			// Current window, rounded to previous window.
			let vesting_window = vesting_point.saturating_sub(vesting_point % vesting_step);
			// The user should have claimed the upfront payment + the vested
			// amount until this window point.
			let vested_reward = reward.total.saturating_sub(upfront_payment);
			Ok(upfront_payment.saturating_add(
				vested_reward.saturating_mul(T::Convert::convert(vesting_window)) /
					T::Convert::convert(reward.vesting_period),
			))
		}
	}

	/// Returns the amount available to claim for the specified account.
	pub fn amount_available_to_claim_for<T: Config>(
		account_id: <T as frame_system::Config>::AccountId,
	) -> Result<T::Balance, DispatchError> {
		let association = Associations::<T>::get(account_id).ok_or(Error::<T>::NotAssociated)?;
		let reward = Rewards::<T>::get(association).ok_or(Error::<T>::InvalidProof)?;
		let should_have_claimed = should_have_claimed::<T>(&reward)?;
		let available_to_claim = should_have_claimed.saturating_sub(reward.claimed);
		Ok(available_to_claim)
	}

	/// Retrieves the remote account from a proof
	///
	/// # Errors
	/// * `InvalidProof` - The proof was invalid for the reward account
	pub fn get_remote_account<T: Config>(
		proof: ProofOf<T>,
		reward_account: &<T as frame_system::Config>::AccountId,
		prefix: &[u8],
	) -> Result<RemoteAccountOf<T>, DispatchErrorWithPostInfo<PostDispatchInfo>> {
		match proof {
			Proof::Ethereum(eth_proof) => {
				let reward_account_encoded =
					reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec());
				let ethereum_address =
					ethereum_recover(prefix, &reward_account_encoded, &eth_proof)
						.ok_or(Error::<T>::InvalidProof)?;
				Ok(RemoteAccount::Ethereum(ethereum_address))
			},
			Proof::RelayChain(relay_account, relay_proof) => {
				ensure!(
					verify_relay(
						prefix,
						reward_account.clone(),
						relay_account.clone(),
						&relay_proof
					),
					Error::<T>::InvalidProof
				);
				Ok(RemoteAccount::RelayChain(relay_account))
			},
		}
	}

	/// Verify that the proof is valid for the given account.
	pub fn verify_relay<AccountId: Encode, RelayChainAccountId: Into<AccountId32>>(
		prefix: &[u8],
		reward_account: AccountId,
		relay_account: RelayChainAccountId,
		proof: &MultiSignature,
	) -> bool {
		/// Polkadotjs wraps the message in this tag before signing.
		const WRAPPED_PREFIX: &[u8] = b"<Bytes>";
		const WRAPPED_POSTFIX: &[u8] = b"</Bytes>";
		let mut msg = WRAPPED_PREFIX.to_vec();
		msg.append(&mut prefix.to_vec());
		msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
		msg.append(&mut WRAPPED_POSTFIX.to_vec());
		proof.verify(&msg[..], &relay_account.into())
	}

	/// Sign a message and produce an `eth_sign` compatible signature.
	pub fn ethereum_signable_message(prefix: &[u8], msg: &[u8]) -> Vec<u8> {
		let mut l = prefix.len() + msg.len();
		let mut msg_len = Vec::new();
		while l > 0 {
			msg_len.push(b'0' + (l % 10) as u8);
			l /= 10;
		}
		let mut v = b"\x19Ethereum Signed Message:\n".to_vec();
		v.extend(msg_len.into_iter().rev());
		v.extend_from_slice(prefix);
		v.extend_from_slice(msg);
		v
	}

	/// Recover the public key of an `eth_sign` signature.
	/// The original message is required for this extraction to be possible.
	pub fn ethereum_recover(
		prefix: &[u8],
		msg: &[u8],
		EcdsaSignature(sig): &EcdsaSignature,
	) -> Option<EthereumAddress> {
		let msg = keccak_256(&ethereum_signable_message(prefix, msg));
		let mut addr = EthereumAddress::default();
		let _ = &addr.0.copy_from_slice(
			&keccak_256(&sp_io::crypto::secp256k1_ecdsa_recover(sig, &msg).ok()?[..])[12..],
		);
		Some(addr)
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::associate { reward_account, proof } = call {
				let now = T::Time::now();
				let enabled = VestingTimeStart::<T>::get()
					.ok_or(InvalidTransaction::Custom(ValidityError::NotClaimableYet as u8))? <=
					now;
				if !enabled {
					return InvalidTransaction::Custom(ValidityError::NotClaimableYet as u8).into()
				}

				if Associations::<T>::get(reward_account).is_some() {
					return InvalidTransaction::Custom(ValidityError::AlreadyAssociated as u8).into()
				}
				let remote_account =
					get_remote_account::<T>(proof.clone(), reward_account, T::Prefix::get())
						.map_err(|_| {
							Into::<TransactionValidityError>::into(InvalidTransaction::Custom(
								ValidityError::InvalidProof as u8,
							))
						})?;
				match Rewards::<T>::get(remote_account.clone()) {
					None => InvalidTransaction::Custom(ValidityError::NoReward as u8).into(),
					Some(reward) if reward.total.is_zero() =>
						InvalidTransaction::Custom(ValidityError::NoReward as u8).into(),
					Some(_) =>
						ValidTransaction::with_tag_prefix("CrowdloanRewardsAssociationCheck")
							.and_provides(remote_account)
							.build(),
				}
			} else {
				Err(InvalidTransaction::Call.into())
			}
		}
	}

	#[repr(u8)]
	pub enum ValidityError {
		InvalidProof = 0,
		NoReward = 1,
		AlreadyAssociated = 2,
		NotClaimableYet = 3,
	}
}
