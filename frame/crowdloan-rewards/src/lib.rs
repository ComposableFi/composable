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
		clippy::disallowed_method,
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
	unused_extern_crates
)]

pub use pallet::*;

pub mod models;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{pallet_prelude::*, traits::fungible::Mutate, transactional};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::keccak_256;
	use sp_runtime::{
		traits::{
			AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert, Saturating, Verify,
			Zero,
		},
		AccountId32, MultiSignature, Perbill,
	};
	use sp_std::vec::Vec;

	use crate::weights::WeightInfo;

	use super::models::{EcdsaSignature, EthereumAddress, Proof, RemoteAccount};

	#[derive(Encode, Decode, PartialEq, Copy, Clone, MaxEncodedLen, TypeInfo)]
	pub struct Reward<Balance, BlockNumber> {
		pub(crate) total: Balance,
		pub(crate) claimed: Balance,
		pub(crate) vesting_period: BlockNumber,
	}

	pub type RemoteAccountOf<T> = RemoteAccount<<T as Config>::RelayChainAccountId>;
	pub type RewardOf<T> = Reward<<T as Config>::Balance, <T as frame_system::Config>::BlockNumber>;
	pub type VestingPeriodOf<T> = <T as frame_system::Config>::BlockNumber;
	pub type RewardAmountOf<T> = <T as Config>::Balance;
	pub type ProofOf<T> = Proof<<T as Config>::RelayChainAccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
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
		InvalidProof,
		InvalidClaim,
		NothingToClaim,
		NotAssociated,
		AlreadyAssociated,
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
			+ Zero;

		/// The currency used to mint the rewards
		type Currency: Mutate<Self::AccountId, Balance = Self::Balance>;

		/// The origin that is allowed to `initialize` the pallet.
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// A conversion function frop `Self::BlockNumber` to `Self::Balance`
		type Convert: Convert<Self::BlockNumber, Self::Balance>;

		/// The relay chain account id.
		type RelayChainAccountId: Parameter + MaybeSerializeDeserialize + Into<AccountId32> + Ord;

		/// The upfront liquidity unlocked at first claim.
		#[pallet::constant]
		type InitialPayment: Get<Perbill>;

		/// The number of blocks a fragment of the reward is vested.
		#[pallet::constant]
		type VestingStep: Get<Self::BlockNumber>;

		/// The arbitrary prefix used for the proof
		#[pallet::constant]
		type Prefix: Get<&'static [u8]>;

		// Extrinsic weights
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type Rewards<T: Config> =
		StorageMap<_, Blake2_128Concat, RemoteAccountOf<T>, RewardOf<T>, OptionQuery>;

	/// The total amount of rewards to be claimed.
	#[pallet::storage]
	#[pallet::getter(fn total_rewards)]
	// Absence of total rewards is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type TotalRewards<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	/// The rewards claimed so far.
	#[pallet::storage]
	#[pallet::getter(fn claimed_rewards)]
	// Absence of claimed rewards is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type ClaimedRewards<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	/// The total number of contributors.
	#[pallet::storage]
	#[pallet::getter(fn total_contributors)]
	// Absence of total contributors is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type TotalContributors<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// The block at which the users are able to claim their rewards.
	#[pallet::storage]
	#[pallet::getter(fn vesting_block_start)]
	pub type VestingBlockStart<T: Config> = StorageValue<_, T::BlockNumber, OptionQuery>;

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
			ensure!(!VestingBlockStart::<T>::exists(), Error::<T>::AlreadyInitialized);
			Self::do_initialize()
		}

		/// Populate pallet by adding more rewards.
		/// Can be called multiple times. Idempotent.
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

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_initialize() -> DispatchResult {
			let current_block = frame_system::Pallet::<T>::block_number();
			VestingBlockStart::<T>::set(Some(current_block));
			Ok(())
		}

		pub(crate) fn do_associate(
			reward_account: T::AccountId,
			proof: ProofOf<T>,
		) -> DispatchResultWithPostInfo {
			let remote_account = get_remote_account::<T>(proof, &reward_account, T::Prefix::get())?;
			// NOTE(hussein-aitlahcen): this is also checked by the signed extension. theoretically
			// useless, but 1:1 to make it clear
			ensure!(
				!Associations::<T>::contains_key(reward_account.clone()),
				Error::<T>::AlreadyAssociated
			);
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
			ensure!(!VestingBlockStart::<T>::exists(), Error::<T>::AlreadyInitialized);
			let _ = Rewards::<T>::remove_all(None);
			rewards
				.into_iter()
				.for_each(|(remote_account, account_reward, vesting_period)| {
					// Populate and possibly overwrite.
					Rewards::<T>::insert(
						remote_account,
						Reward {
							total: account_reward,
							claimed: T::Balance::zero(),
							vesting_period,
						},
					);
				});
			// NOTE(hussein-aitlahcen): recompute instead of adding to avoid issues with duplicates
			let (total_rewards, total_contributors) = Rewards::<T>::iter_values().fold(
				(T::Balance::zero(), 0),
				|(total_rewards, total_contributors), contributor_reward| {
					(total_rewards + contributor_reward.total, total_contributors + 1)
				},
			);
			TotalRewards::<T>::set(total_rewards);
			TotalContributors::<T>::set(total_contributors);
			Ok(())
		}

		/// Do claim the reward for a given remote account, rewarding the `reward_account`.
		/// Returns `InvalidProof` if the user is not a contributor or `NothingToClaim` if not
		/// reward can be claimed yet.
		pub(crate) fn do_claim(
			remote_account: RemoteAccountOf<T>,
			reward_account: &T::AccountId,
		) -> Result<T::Balance, DispatchError> {
			let start = VestingBlockStart::<T>::get().ok_or(Error::<T>::NotInitialized)?;
			Rewards::<T>::try_mutate(remote_account, |reward| {
				reward
					.as_mut()
					.map(|reward| {
						let upfront_payment = T::InitialPayment::get().mul_floor(reward.total);
						let should_have_claimed = {
							let current_block = frame_system::Pallet::<T>::block_number();
							// Current point in time
							let vesting_point = current_block.saturating_sub(start);
							if vesting_point >= reward.vesting_period {
								// If the user is claiming when the period is over, he should
								// probably have already claimed everything.
								reward.total
							} else {
								let vesting_step = T::VestingStep::get();
								// Current window, rounded to previous window.
								let vesting_window = vesting_point - (vesting_point % vesting_step);
								// The user should have claimed the upfront payment + the vested
								// amount until this window point.
								let vested_reward = reward.total - upfront_payment;
								upfront_payment +
									(vested_reward
										.saturating_mul(T::Convert::convert(vesting_window)) /
										T::Convert::convert(reward.vesting_period))
							}
						};
						let available_to_claim = should_have_claimed - reward.claimed;
						ensure!(
							available_to_claim > T::Balance::zero(),
							Error::<T>::NothingToClaim
						);
						T::Currency::mint_into(reward_account, available_to_claim)?;
						(*reward).claimed += available_to_claim;
						ClaimedRewards::<T>::mutate(|x| *x += available_to_claim);
						Ok(available_to_claim)
					})
					.unwrap_or_else(|| Err(Error::<T>::InvalidProof.into()))
			})
		}
	}

	pub fn get_remote_account<T: Config>(
		proof: Proof<<T as Config>::RelayChainAccountId>,
		reward_account: &<T as frame_system::Config>::AccountId,
		prefix: &[u8],
	) -> Result<
		RemoteAccount<<T as Config>::RelayChainAccountId>,
		sp_runtime::DispatchErrorWithPostInfo<frame_support::dispatch::PostDispatchInfo>,
	> {
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
		let wrapped_prefix: &[u8] = b"<Bytes>";
		let wrapped_postfix: &[u8] = b"</Bytes>";
		let mut msg = wrapped_prefix.to_vec();
		msg.append(&mut prefix.to_vec());
		msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
		msg.append(&mut wrapped_postfix.to_vec());
		proof.verify(&msg[..], &relay_account.into())
	}

	/// Signable message that would be generated by eth_sign
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

	/// Recover the public key
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
	}
}
