/*
Crowdloan rewards pallet used by contributors to claim their rewards.

A user is able to claim rewards once it has an associated account. Associating
an account using the `associate` extrinsic automatically yield the upfront
liquidity (% of the vested reward). The rest of the reward can be claimed every
`VestingStep` starting at the block when the pallet has been initialized
using the `initialize` extrinsic.

Proof to provide when associating a reward account:
```haskell
proof = sign (concat prefix (hex reward_account))
```

Reference for proof mechanism: https://github.com/paritytech/polkadot/blob/master/runtime/common/src/claims.rs
*/

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

// FIXME: runtime signature generation must use host features.
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::models::{Proof, RemoteAccount, Reward};
	use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_support::{
		math::safe::SafeAdd,
		types::{EcdsaSignature, EthereumAddress},
	};
	use frame_support::{
		dispatch::PostDispatchInfo,
		pallet_prelude::*,
		traits::{
			fungible::{Inspect, Transfer},
			Time,
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
		Initialized {
			at: MomentOf<T>,
		},
		Claimed {
			remote_account: RemoteAccountOf<T>,
			reward_account: T::AccountId,
			amount: T::Balance,
		},
		Associated {
			remote_account: RemoteAccountOf<T>,
			reward_account: T::AccountId,
		},
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

		/// The RewardAsset used to transfer the rewards
		type RewardAsset: Inspect<Self::AccountId, Balance = Self::Balance>
			+ Transfer<Self::AccountId, Balance = Self::Balance>;

		type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen + FullCodec;

		/// The time provider.
		type Time: Time<Moment = Self::Moment>;

		/// The origin that is allowed to `initialize` the pallet.
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// A conversion function frop `Self::Moment` to `Self::Balance`
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

		/// The time you have to wait to unlock another part of your reward.
		#[pallet::constant]
		type VestingStep: Get<MomentOf<Self>>;

		/// The arbitrary prefix used for the proof
		#[pallet::constant]
		type Prefix: Get<&'static [u8]>;

		/// The implementation of extrinsics weight.
		type WeightInfo: WeightInfo;

		/// The unique identifier of this pallet.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
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

	/// The block at which the users are able to claim their rewards.
	#[pallet::storage]
	#[pallet::getter(fn vesting_block_start)]
	pub type VestingTimeStart<T: Config> = StorageValue<_, MomentOf<T>, OptionQuery>;

	/// Associate a local account with a remote one.
	#[pallet::storage]
	#[pallet::getter(fn associations)]
	pub type Associations<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, RemoteAccountOf<T>, OptionQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initialize the pallet at the current transaction block.
		#[pallet::weight(<T as Config>::WeightInfo::initialize(TotalContributors::<T>::get()))]
		#[transactional]
		pub fn initialize(origin: OriginFor<T>) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			let now = T::Time::now();
			Self::do_initialize(now)
		}

		/// Initialize the pallet at the given transaction block.
		#[pallet::weight(<T as Config>::WeightInfo::initialize(TotalContributors::<T>::get()))]
		#[transactional]
		pub fn initialize_at(origin: OriginFor<T>, at: MomentOf<T>) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_initialize(at)
		}

		/// Populate pallet by adding more rewards.
		/// Can be called multiple times. If an remote account already has a reward, it will be
		/// replaced by the new reward value.
		/// Can only be called before `initialize`.
		#[pallet::weight(<T as Config>::WeightInfo::populate(rewards.len() as u32))]
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
	}

	#[pallet::extra_constants]
	impl<T: Config> Pallet<T> {
		/// The AccountId of this pallet.
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_initialize(at: MomentOf<T>) -> DispatchResult {
			ensure!(!VestingTimeStart::<T>::exists(), Error::<T>::AlreadyInitialized);
			let now = T::Time::now();
			ensure!(at >= now, Error::<T>::BackToTheFuture);
			VestingTimeStart::<T>::set(Some(at));
			Self::deposit_event(Event::Initialized { at });
			Ok(())
		}

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
			// upfront payment, which will allow the user to execute transactions because he had 0
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

		pub(crate) fn do_populate(
			rewards: Vec<(RemoteAccountOf<T>, RewardAmountOf<T>, VestingPeriodOf<T>)>,
		) -> DispatchResult {
			ensure!(!VestingTimeStart::<T>::exists(), Error::<T>::AlreadyInitialized);
			rewards
				.into_iter()
				.for_each(|(remote_account, account_reward, vesting_period)| {
					// This will eliminate duplicated entries.
					Rewards::<T>::insert(
						remote_account,
						Reward {
							total: account_reward,
							claimed: T::Balance::zero(),
							vesting_period,
						},
					);
				});
			let (total_rewards, total_contributors) = Rewards::<T>::iter_values().try_fold(
				(T::Balance::zero(), 0),
				|(total_rewards, total_contributors),
				 contributor_reward|
				 -> Result<(T::Balance, u32), DispatchError> {
					Ok((
						total_rewards.safe_add(&contributor_reward.total)?,
						total_contributors.safe_add(&1)?,
					))
				},
			)?;
			TotalRewards::<T>::set(total_rewards);
			TotalContributors::<T>::set(total_contributors);
			let available_funds = T::RewardAsset::balance(&Self::account_id());
			ensure!(available_funds == total_rewards, Error::<T>::RewardsNotFunded);
			Ok(())
		}

		/// Do claim the reward for a given remote account, rewarding the `reward_account`.
		/// Returns `InvalidProof` if the user is not a contributor or `NothingToClaim` if no
		/// reward can be claimed yet.
		pub(crate) fn do_claim(
			remote_account: RemoteAccountOf<T>,
			reward_account: &T::AccountId,
		) -> Result<T::Balance, DispatchError> {
			Rewards::<T>::try_mutate(remote_account, |reward| {
				reward
					.as_mut()
					.map(|reward| {
						let should_have_claimed = should_have_claimed::<T>(reward)?;
						let available_to_claim = should_have_claimed.saturating_sub(reward.claimed);
						ensure!(
							available_to_claim > T::Balance::zero(),
							Error::<T>::NothingToClaim
						);
						let funds_account = Self::account_id();
						// No need to keep the pallet account alive.
						T::RewardAsset::transfer(
							&funds_account,
							reward_account,
							available_to_claim,
							false,
						)?;
						reward.claimed = available_to_claim.saturating_add(reward.claimed);
						ClaimedRewards::<T>::mutate(|x| *x = x.saturating_add(available_to_claim));
						Ok(available_to_claim)
					})
					.unwrap_or_else(|| Err(Error::<T>::InvalidProof.into()))
			})
		}
	}

	/// The reward amount a user should have claimed until now.
	pub fn should_have_claimed<T: Config>(
		reward: &RewardOf<T>,
	) -> Result<T::Balance, DispatchError> {
		let start = VestingTimeStart::<T>::get().ok_or(Error::<T>::NotInitialized)?;
		let upfront_payment = T::InitialPayment::get().mul_floor(reward.total);

		let now = T::Time::now();
		// Current point in time
		let vesting_point = now.saturating_sub(start);
		if vesting_point >= reward.vesting_period {
			// If the user is claiming when the period is over, he should
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

	pub fn get_remote_account<T: Config>(
		proof: ProofOf<T>,
		reward_account: &<T as frame_system::Config>::AccountId,
		prefix: &[u8],
	) -> Result<RemoteAccountOf<T>, DispatchErrorWithPostInfo<PostDispatchInfo>> {
		let remote_account = match proof {
			Proof::Ethereum(eth_proof) => {
				let reward_account_encoded =
					reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec());
				let ethereum_address =
					ethereum_recover(prefix, &reward_account_encoded, &eth_proof)
						.ok_or(Error::<T>::InvalidProof)?;
				Result::<_, DispatchError>::Ok(RemoteAccount::Ethereum(ethereum_address))
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
		}?;
		Ok(remote_account)
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
