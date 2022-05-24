#![doc = include_str!("../README.md")]

// TODO: mock, test, weights

pub use pallet::*;

pub mod models;
pub mod weights;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mocks;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		models::{Airdrop, AirdropState, Proof, RecipientFund, RemoteAccount},
		weights::WeightInfo,
	};
	use codec::{Codec, FullCodec, MaxEncodedLen};
	use composable_support::{
		abstractions::{
			nonce::{Increment, Nonce},
			utils::{increment::SafeIncrement, start_at::ZeroInit},
		},
		math::safe::{SafeAdd, SafeSub},
		types::{EcdsaSignature, EthereumAddress},
	};
	use composable_traits::airdrop::AirdropManagement;
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
		AccountId32, DispatchErrorWithPostInfo, MultiSignature,
	};
	use sp_std::{fmt::Debug, vec::Vec};

	/// [`AccountId`](frame_system::Config::AccountId) as configured by the pallet.
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	/// [`AirdropId`](Config::AirdropId) as configured by the pallet.
	pub type AirdropIdOf<T> = <T as Config>::AirdropId;
	/// [`Airdrop`](crate::models::Airdrop) as configured by the pallet.
	pub type AirdropOf<T> = Airdrop<
		<T as frame_system::Config>::AccountId,
		<T as Config>::Balance,
		<T as Config>::Moment,
	>;
	/// [`Balance`](Config::Balance) as configured by the pallet.
	pub type BalanceOf<T> = <T as Config>::Balance;
	/// [`RecipientFund`](crate::models::RecipientFund) as configured by the pallet.
	pub type RecipientFundOf<T> = RecipientFund<<T as Config>::Balance, <T as Config>::Moment>;
	/// [`Moment`](Config::Moment) as configured by the pallet.
	pub type MomentOf<T> = <T as Config>::Moment;
	/// ['Proof'](crate::models::Proof) as configured by the pallet
	pub type ProofOf<T> = Proof<<T as Config>::RelayChainAccountId>;
	pub type RemoteAccountOf<T> = RemoteAccount<<T as Config>::RelayChainAccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AirdropCreated {
			airdrop_id: T::AirdropId,
			by: T::AccountId,
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
		AirdropAlreadyStarted,
		AirdropDoesNotExist,
		AirdropIsNotEnabled,
		BackToTheFuture,
		NotAirdropCreator,
		NothingToClaim,
		RecipientAlreadyClaimed,
		RecipientNotFound,
		InvalidProof,
		UnclaimedFundsRemaining,
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
			+ Parameter
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

		/// Conversion function from [`Self::Moment`](Self::Moment) to
		/// [`Self::Balance`](Self::Balance)
		type Convert: Convert<Self::Moment, Self::Balance>;

		/// Time stamp
		type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen + FullCodec;

		/// Relay chain account ID
		type RelayChainAccountId: Parameter
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ Into<AccountId32>
			+ Ord;

		/// The asset type Recipients will claim from the Airdrops.
		type RecipientFundAsset: Inspect<Self::AccountId, Balance = Self::Balance>
			+ Transfer<Self::AccountId, Balance = Self::Balance>;

		/// Time provider
		type Time: Time<Moment = Self::Moment>;

		/// The pallet ID required for creating sub-accounts used by Airdrops.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The prefix used in proofs
		#[pallet::constant]
		type Prefix: Get<&'static [u8]>;

		/// The stake required to craete an Airdrop
		#[pallet::constant]
		type Stake: Get<BalanceOf<Self>>;

		/// The implementation of extrinsic weights.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The counter used to identify Airdrops.
	#[pallet::storage]
	#[pallet::getter(fn airdrop_count)]
	pub type AirdropCount<T: Config> =
		StorageValue<_, T::AirdropId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// Airdrops currently stored by the pallet.
	#[pallet::storage]
	#[pallet::getter(fn airdrops)]
	pub type Airdrops<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AirdropId, AirdropOf<T>, OptionQuery>;

	/// Associations of local accounts and an [`AirdropId`](Config::AirdropId) to a remote account.
	#[pallet::storage]
	#[pallet::getter(fn associations)]
	pub type Associations<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AirdropId,
		Blake2_128Concat,
		T::AccountId,
		RemoteAccountOf<T>,
		OptionQuery,
	>;

	/// Recipient funds of Airdrops stored by the pallet.
	#[pallet::storage]
	#[pallet::getter(fn recipient_funds)]
	pub type RecipientFunds<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AirdropId,
		Blake2_128Concat,
		RemoteAccountOf<T>,
		RecipientFundOf<T>,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new Airdrop. This requires that the user puts down a stake in PICA.
		///
		/// If `start_at` is `Some(MomentOf<T>)` and the `MomentOf<T>` is greater than the current
		/// block, the Airdrop will be scheduled to start automatically.
		///
		/// Can be called by any signed origin.
		#[pallet::weight(<T as Config>::WeightInfo::create_airdrop(1))]
		#[transactional]
		pub fn create_airdrop(
			origin: OriginFor<T>,
			start_at: Option<MomentOf<T>>,
			vesting_schedule: MomentOf<T>,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;

			<Self as AirdropManagement>::create_airdrop(creator, start_at, vesting_schedule)
		}

		/// Add one or more recipients to the Airdrop, specifying the token amount that each
		/// provided adress will receive.
		///
		/// Only callable by the origin that created the Airdrop.
		#[pallet::weight(<T as Config>::WeightInfo::add_recipient(10_000))]
		#[transactional]
		pub fn add_recipient(
			origin: OriginFor<T>,
			airdrop_id: T::AirdropId,
			recipients: Vec<(RemoteAccountOf<T>, BalanceOf<T>, bool)>,
		) -> DispatchResult {
			let origin_id = ensure_signed(origin)?;

			<Self as AirdropManagement>::add_recipient(origin_id, airdrop_id, recipients)
		}

		/// Remove a recipient from an Airdrop.
		///
		/// Only callable by the origin that created the Airdrop.
		#[pallet::weight(<T as Config>::WeightInfo::remove_recipient(10_000))]
		#[transactional]
		pub fn remove_recipient(
			origin: OriginFor<T>,
			airdrop_id: T::AirdropId,
			recipient: RemoteAccountOf<T>,
		) -> DispatchResult {
			let origin_id = ensure_signed(origin)?;

			<Self as AirdropManagement>::remove_recipient(origin_id, airdrop_id, recipient)
		}

		/// Start an Airdrop.
		///
		/// Only callable by the origin that created the Airdrop.
		///
		/// # Errors
		/// * If the Airdrop has been configured to start after a certain timestamp
		#[pallet::weight(<T as Config>::WeightInfo::enable_airdrop(1))]
		#[transactional]
		pub fn enable_airdrop(origin: OriginFor<T>, airdrop_id: T::AirdropId) -> DispatchResult {
			let origin_id = ensure_signed(origin)?;

			<Self as AirdropManagement>::enable_airdrop(origin_id, airdrop_id)
		}

		/// Stop an Airdrop.
		///
		/// Only callable by the origin that created the Airdrop.
		#[pallet::weight(<T as Config>::WeightInfo::disable_airdrop(1))]
		#[transactional]
		pub fn disable_airdrop(origin: OriginFor<T>, airdrop_id: T::AirdropId) -> DispatchResult {
			let origin_id = ensure_signed(origin)?;

			<Self as AirdropManagement>::disable_airdrop(origin_id, airdrop_id)?;
			Ok(())
		}

		/// Claim recipient funds from an Airdrop.
		///
		/// If no more funds are left to claim, the Airdrop will be removed.
		///
		/// Callable by any origin.
		#[pallet::weight(<T as Config>::WeightInfo::claim(1))]
		#[transactional]
		pub fn claim(
			origin: OriginFor<T>,
			airdrop_id: T::AirdropId,
			reward_account: T::AccountId,
			proof: ProofOf<T>,
		) -> DispatchResultWithPostInfo {
			let remote_account =
				Self::get_remote_account(proof, &reward_account, T::Prefix::get())?;

			match Associations::<T>::get(airdrop_id, reward_account.clone()) {
				// Confirm association matches
				Some(associated_account) => {
					ensure!(associated_account == remote_account, Error::<T>::InvalidProof);
				},
				// If no association exist, create a new one
				None => {
					Associations::<T>::insert(
						airdrop_id,
						reward_account.clone(),
						remote_account.clone(),
					);
				},
			}

			<Self as AirdropManagement>::claim(airdrop_id, remote_account, reward_account)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Gets the account ID to be used by the Airdrop.
		pub(crate) fn get_airdrop_account_id(airdrop_id: T::AirdropId) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(airdrop_id)
		}

		/// Gets the [`Airdrop`](crate::models::Airdrop) associated with the `airdrop_id`
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		pub(crate) fn get_airdrop(
			airdrop_id: &T::AirdropId,
		) -> Result<AirdropOf<T>, DispatchError> {
			Airdrops::<T>::try_get(airdrop_id).map_err(|_| Error::<T>::AirdropDoesNotExist.into())
		}

		/// Calculates the current [`AirdropState`](crate::models::AirdropState) of an Airdrop
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		pub(crate) fn get_airdrop_state(
			airdrop_id: T::AirdropId,
		) -> Result<AirdropState, DispatchError> {
			let airdrop = Self::get_airdrop(&airdrop_id)?;

			if airdrop.disabled {
				return Ok(AirdropState::Disabled)
			}

			airdrop.start.map_or(Ok(AirdropState::Created), |start| {
				if start <= T::Time::now() {
					Ok(AirdropState::Enabled)
				} else {
					Ok(AirdropState::Created)
				}
			})
		}

		/// Gets the [`RecipientFund`](crate::models::RecipientFund) of an Airdrop that is
		/// associated with the `remote_account`.
		///
		/// # Errors
		/// * `RecipientNotFound` - No recipient associated with the remote_account could be found.
		pub(crate) fn get_recipient_fund(
			airdrop_id: T::AirdropId,
			remote_account: RemoteAccountOf<T>,
		) -> Result<RecipientFundOf<T>, DispatchError> {
			RecipientFunds::<T>::try_get(airdrop_id, remote_account)
				.map_err(|_| Error::<T>::RecipientNotFound.into())
		}

		/// Gets the remote account address from the `Proof`.
		///
		/// If the proof is invalid, an error will be returned.
		pub(crate) fn get_remote_account(
			proof: ProofOf<T>,
			reward_account: &<T as frame_system::Config>::AccountId,
			prefix: &[u8],
		) -> Result<RemoteAccountOf<T>, DispatchErrorWithPostInfo<PostDispatchInfo>> {
			let remote_account = match proof {
				Proof::Ethereum(eth_proof) => {
					let reward_account_encoded =
						reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec());
					let eth_address =
						Self::ethereum_recover(prefix, &reward_account_encoded, &eth_proof)
							.ok_or(Error::<T>::InvalidProof)?;
					Result::<_, DispatchError>::Ok(RemoteAccount::Ethereum(eth_address))
				},
				Proof::RelayChain(relay_account, relay_proof) => {
					ensure!(
						Self::verify_relay(
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

		/// Verify the proof is valid for a given account.
		pub(crate) fn verify_relay(
			prefix: &[u8],
			reward_account: <T as frame_system::Config>::AccountId,
			relay_account: T::RelayChainAccountId,
			proof: &MultiSignature,
		) -> bool {
			// Polkadot.js wrapper tags
			const WRAPPED_PREFIX: &[u8] = b"<Bytes>";
			const WRAPPED_POSTFIX: &[u8] = b"</Bytes>";
			let mut msg = WRAPPED_PREFIX.to_vec();

			msg.append(&mut prefix.to_vec());
			msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
			msg.append(&mut WRAPPED_POSTFIX.to_vec());

			proof.verify(&msg[..], &relay_account.into())
		}

		/// Recover the public key of an `eth_sign` signature.
		///
		/// Requires the original message.
		pub(crate) fn ethereum_recover(
			prefix: &[u8],
			msg: &[u8],
			EcdsaSignature(sig): &EcdsaSignature,
		) -> Option<EthereumAddress> {
			let msg = keccak_256(&Self::ethereum_signable_message(prefix, msg));
			let mut address = EthereumAddress::default();

			address.0.copy_from_slice(
				&keccak_256(&sp_io::crypto::secp256k1_ecdsa_recover(sig, &msg).ok()?[..])[12..],
			);

			Some(address)
		}

		/// Sign a message to produce an `eth_sign` compatible signature.
		pub(crate) fn ethereum_signable_message(prefix: &[u8], msg: &[u8]) -> Vec<u8> {
			let mut length = prefix.len() + msg.len();
			let mut msg_len = Vec::new();

			while length > 0 {
				msg_len.push(b'0' + (length % 10) as u8);
				length /= 10;
			}

			let mut signed_message = b"\x19Ethereum Signed Message:\n".to_vec();
			signed_message.extend(msg_len.into_iter().rev());
			signed_message.extend_from_slice(prefix);
			signed_message.extend_from_slice(msg);

			signed_message
		}

		/// Start an Airdrop at a given moment.
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		/// * `AirdropAlreadyStarted` - The Airdrop has already started or has been scheduled to
		/// start
		/// * `BackToTheFuture` - The provided `start` has already passed
		pub(crate) fn start_airdrop_at(
			airdrop_id: T::AirdropId,
			start: T::Moment,
		) -> DispatchResult {
			// Airdrop exist and hasn't started
			let mut airdrop = Self::get_airdrop(&airdrop_id)?;
			ensure!(airdrop.start.is_none(), Error::<T>::AirdropAlreadyStarted);

			// Start is valid
			let now = T::Time::now();
			ensure!(start >= now, Error::<T>::BackToTheFuture);

			// Update Airdrop
			airdrop.start = Some(start);
			Airdrops::<T>::insert(airdrop_id, airdrop);

			Self::deposit_event(Event::AirdropStarted { airdrop_id, at: start });

			Ok(())
		}

		pub(crate) fn should_have_claimed(
			airdrop_id: T::AirdropId,
			fund: &RecipientFundOf<T>,
		) -> Result<T::Balance, DispatchError> {
			let airdrop = Airdrops::<T>::get(airdrop_id).ok_or(Error::<T>::AirdropDoesNotExist)?;
			let airdrop_state = Self::get_airdrop_state(airdrop_id)?;
			match (airdrop_state, airdrop.start) {
				(AirdropState::Enabled, Some(start)) => {
					let now = T::Time::now();
					let vesting_point = now.saturating_sub(start);

					// If the vesting period is over, the recipient should receive the remainder of
					// the fund
					if vesting_point >= fund.vesting_period {
						return Ok(fund.total)
					}

					// The current vesting window rounded to the previous window
					let vesting_window =
						vesting_point.saturating_sub(vesting_point % airdrop.schedule);

					let should_have_claimed =
						fund.total.saturating_mul(T::Convert::convert(vesting_window)) /
							T::Convert::convert(fund.vesting_period);

					Ok(should_have_claimed)
				},
				_ => Err(Error::<T>::AirdropIsNotEnabled.into()),
			}
		}

		/// Removes an Airdrop and associated data from the pallet iff all funds have been recorded
		/// as claimed.
		pub(crate) fn prune_airdrop(airdrop_id: T::AirdropId) -> Result<bool, DispatchError> {
			let airdrop = Self::get_airdrop(&airdrop_id)?;
			let airdrop_account = Self::get_airdrop_account_id(airdrop_id);

			if airdrop.total_funds > airdrop.claimed_funds {
				return Ok(false)
			}

			// Return remaining funds to the Airdrop creator
			T::RecipientFundAsset::transfer(
				&airdrop_account,
				&airdrop.creator,
				T::RecipientFundAsset::balance(&airdrop_account),
				false,
			)?;

			// Remove Airdrop and associated data from storage
			RecipientFunds::<T>::remove_prefix(airdrop_id, None);
			Associations::<T>::remove_prefix(airdrop_id, None);
			Airdrops::<T>::remove(airdrop_id);

			Ok(true)
		}
	}

	impl<T: Config> AirdropManagement for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AirdropId = AirdropIdOf<T>;
		type AirdropStart = MomentOf<T>;
		type Balance = BalanceOf<T>;
		type Proof = ProofOf<T>;
		type Recipient = RemoteAccountOf<T>;
		type RecipientCollection = Vec<(Self::Recipient, BalanceOf<T>, bool)>;
		type RemoteAccount = RemoteAccountOf<T>;
		type VestingSchedule = MomentOf<T>;

		/// Create a new Airdrop.
		///
		/// Provide `None` for `start` if starting the Airdrop manually is desired.
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		/// * `BackToTheFuture` - The provided `start` has already passed
		fn create_airdrop(
			creator_id: Self::AccountId,
			start: Option<Self::AirdropStart>,
			schedule: Self::VestingSchedule,
		) -> DispatchResult {
			let airdrop_id = AirdropCount::<T>::increment()?;
			let airdrop_account = Self::get_airdrop_account_id(airdrop_id);

			// Transfer stake into airdrop specific account.
			T::RecipientFundAsset::transfer(&creator_id, &airdrop_account, T::Stake::get(), false)?;

			// Insert newly created airdrop into pallet's list.
			Airdrops::<T>::insert(
				airdrop_id,
				Airdrop {
					creator: creator_id.clone(),
					total_funds: T::Balance::zero(),
					total_recipients: 0,
					claimed_funds: T::Balance::zero(),
					start: None,
					schedule,
					disabled: false,
				},
			);

			Self::deposit_event(Event::AirdropCreated { airdrop_id, by: creator_id });

			match start {
				Some(moment) => {
					Self::start_airdrop_at(airdrop_id, moment)?;
				},
				None => {},
			}

			Ok(())
		}

		/// Add one or more recipients to an Airdrop.
		///
		/// Airdrop creator is expected to be able to fund the Airdrop. If the Airdrops current
		/// funds aren't enough to supply all claims, the creator will be charged the difference.
		///
		/// If a recipient is already a member of an Airdrop, their previous entry will be
		/// replaced for that Airdrop.
		fn add_recipient(
			origin_id: Self::AccountId,
			airdrop_id: Self::AirdropId,
			recipients: Self::RecipientCollection,
		) -> DispatchResult {
			let airdrop = Self::get_airdrop(&airdrop_id)?;

			ensure!(airdrop.creator == origin_id, Error::<T>::NotAirdropCreator);

			// Calculate total funds and recipients local to this transaction
			let (transaction_funds, transaction_recipients) = recipients.iter().try_fold(
				(T::Balance::zero(), 0),
				|(transaction_funds, transaction_recipients),
				 (_, funds, _)|
				 -> Result<(T::Balance, u32), DispatchError> {
					Ok((transaction_funds.safe_add(&funds)?, transaction_recipients.safe_add(&1)?))
				},
			)?;

			// Funds currently owned by the Airdrop minus the creation stake
			let current_funds =
				T::RecipientFundAsset::balance(&Self::get_airdrop_account_id(airdrop_id))
					.safe_sub(&T::Stake::get())?;
			// Total amount of funds to be required by this Airdrop
			let total_funds = airdrop.total_funds.safe_add(&transaction_funds)?;
			let total_recipients = airdrop.total_recipients.safe_add(&transaction_recipients)?;

			// If the airdrop can't support the total amount of claimable funds
			if current_funds < total_funds {
				// Fund Airdrop account from creators account
				T::RecipientFundAsset::transfer(
					&airdrop.creator,
					&Self::get_airdrop_account_id(airdrop_id),
					total_funds.safe_sub(&current_funds)?,
					false,
				)?;
			}

			// Populate `RecipientFunds`
			recipients.iter().for_each(|(remote_account, funds, is_funded)| {
				RecipientFunds::<T>::insert(
					airdrop_id,
					remote_account,
					RecipientFundOf::<T> {
						total: *funds,
						claimed: T::Balance::zero(),
						vesting_period: airdrop.schedule,
						funded_claim: *is_funded,
					},
				);
			});

			// Update Airdrop statistics
			Airdrops::<T>::try_mutate(airdrop_id, |airdrop| {
				airdrop
					.as_mut()
					.map(|airdrop| {
						airdrop.total_funds = total_funds;
						airdrop.total_recipients = total_recipients;
						Ok(())
					})
					.unwrap_or_else(|| Err(Error::<T>::AirdropDoesNotExist))
			})?;

			Ok(())
		}

		/// Remove a recipient from an Airdrop.
		///
		/// Refunds the creator for the value of the recipient fund.
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		/// * `RecipientNotFound` - No recipient associated with the remote_account could be found.
		/// * `RecipientAlreadyClaimed` - The recipient has already began claiming their funds.
		fn remove_recipient(
			origin_id: Self::AccountId,
			airdrop_id: Self::AirdropId,
			recipient: Self::Recipient,
		) -> DispatchResult {
			let airdrop = Self::get_airdrop(&airdrop_id)?;
			ensure!(airdrop.creator == origin_id, Error::<T>::NotAirdropCreator);

			let airdrop_account = Self::get_airdrop_account_id(airdrop_id);
			let recipient_fund = Self::get_recipient_fund(airdrop_id, recipient.clone())?;

			ensure!(
				recipient_fund.claimed == T::Balance::zero(),
				Error::<T>::RecipientAlreadyClaimed
			);

			// Update Airdrop details
			let creator = Airdrops::<T>::try_mutate(airdrop_id, |airdrop| {
				airdrop
					.as_mut()
					.map(|airdrop| {
						airdrop.total_funds =
							airdrop.total_funds.saturating_sub(recipient_fund.total);
						Ok(airdrop.creator.clone())
					})
					.unwrap_or_else(|| Err(Error::<T>::AirdropDoesNotExist))
			})?;

			// Refund Airdrop creator for the recipient fund's value
			T::RecipientFundAsset::transfer(
				&airdrop_account,
				&creator,
				recipient_fund.total,
				false,
			)?;

			RecipientFunds::<T>::remove(airdrop_id, recipient);

			if Self::prune_airdrop(airdrop_id)? {
				Self::deposit_event(Event::AirdropEnded { airdrop_id, at: T::Time::now() })
			}

			Ok(())
		}

		/// Start an Airdrop.
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		/// * `AirdropAlreadyStarted` - The Airdrop has already started or has been scheduled to
		/// start
		/// * `BackToTheFuture` - The provided `start` has already passed
		fn enable_airdrop(
			origin_id: Self::AccountId,
			airdrop_id: Self::AirdropId,
		) -> DispatchResult {
			let airdrop = Self::get_airdrop(&airdrop_id)?;
			ensure!(airdrop.creator == origin_id, Error::<T>::NotAirdropCreator);

			Self::start_airdrop_at(airdrop_id, T::Time::now())?;
			Ok(())
		}

		/// Stop an Airdrop.
		///
		/// Returns the amount of unclaimed funds from the airdrop upon success.
		///
		/// # Errors
		/// * `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
		fn disable_airdrop(
			origin_id: Self::AccountId,
			airdrop_id: Self::AirdropId,
		) -> Result<Self::Balance, DispatchError> {
			let airdrop = Self::get_airdrop(&airdrop_id)?;
			ensure!(airdrop.creator == origin_id, Error::<T>::NotAirdropCreator);

			let unclaimed_funds = Airdrops::<T>::try_mutate(airdrop_id, |airdrop| {
				airdrop
					.as_mut()
					.map(|airdrop| {
						let at = T::Time::now();
						let unclaimed_funds = airdrop.total_funds - airdrop.claimed_funds;

						// REVEIW: Checking each recipient fund to see if they have started
						// cliaming could prove to be expensive. Should we instead require that all
						// funds be claimed for an airdrop to end?
						// sets claimed funds equal to total funds so the airdrop can be pruned
						airdrop.disabled = true;
						airdrop.claimed_funds = airdrop.total_funds;

						Self::deposit_event(Event::AirdropEnded { airdrop_id, at });

						Ok(unclaimed_funds)
					})
					.unwrap_or_else(|| Err(Error::<T>::AirdropDoesNotExist.into()))
			});

			Self::prune_airdrop(airdrop_id)?;

			unclaimed_funds
		}

		/// Claim a recipient reward from an Airdrop.
		fn claim(
			airdrop_id: Self::AirdropId,
			remote_account: Self::RemoteAccount,
			reward_account: Self::AccountId,
		) -> DispatchResultWithPostInfo {
			let airdrop_account = Self::get_airdrop_account_id(airdrop_id);
			let (available_to_claim, recipient_fund) =
				RecipientFunds::<T>::try_mutate(airdrop_id, remote_account.clone(), |fund| {
					fund.as_mut()
						.map(|fund| {
							let should_have_claimed =
								Self::should_have_claimed(airdrop_id, fund)
									.map_err(|_| Error::<T>::AirdropDoesNotExist)?;
							let available_to_claim =
								should_have_claimed.saturating_sub(fund.claimed);

							ensure!(
								available_to_claim > T::Balance::zero(),
								Error::<T>::NothingToClaim
							);

							// Update Airdrop and fund status
							(*fund).claimed = fund.claimed.saturating_add(available_to_claim);

							Ok((available_to_claim, fund.clone()))
						})
						.unwrap_or_else(|| Err(Error::<T>::RecipientNotFound))
				})?;

			T::RecipientFundAsset::transfer(
				&airdrop_account,
				&reward_account,
				available_to_claim,
				false,
			)?;

			Airdrops::<T>::try_mutate(airdrop_id, |airdrop| {
				airdrop
					.as_mut()
					.map(|airdrop| {
						airdrop.claimed_funds =
							airdrop.claimed_funds.saturating_add(available_to_claim);
						Ok(())
					})
					.unwrap_or_else(|| Err(Error::<T>::AirdropDoesNotExist))
			})?;

			if Self::prune_airdrop(airdrop_id)? {
				Self::deposit_event(Event::AirdropEnded { airdrop_id, at: T::Time::now() })
			}

			if recipient_fund.funded_claim {
				return Ok(Pays::No.into())
			}

			Ok(Pays::Yes.into())
		}
	}

	/// Ensures the following:
	/// * Only call can be called via an unsigned transaction
	/// * The Airdrop exists in the pallet's storage
	/// * The Airdrop has been enabled / has started
	/// * The provided proof is valid
	/// * The recipient has funds to claim
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::claim { airdrop_id, reward_account, proof } = call {
				// Validity Error if the airdrop does not exist
				let airdrop_state = Self::get_airdrop_state(*airdrop_id).map_err(|_| {
					Into::<TransactionValidityError>::into(InvalidTransaction::Custom(
						ValidityError::NotAnAirdrop as u8,
					))
				})?;

				// Validity Error if the airdrop has not started
				if airdrop_state != AirdropState::Enabled {
					return InvalidTransaction::Custom(ValidityError::NotClaimable as u8).into()
				}

				// Evaluate proof
				let remote_account =
					Self::get_remote_account(proof.clone(), reward_account, T::Prefix::get())
						.map_err(|_| {
							Into::<TransactionValidityError>::into(InvalidTransaction::Custom(
								ValidityError::InvalidProof as u8,
							))
						})?;

				match Associations::<T>::get(airdrop_id, reward_account) {
					// Validity Error if the account is already associated to another
					Some(associated_account) =>
						if associated_account != remote_account {
							return InvalidTransaction::Custom(
								ValidityError::AlreadyAssociated as u8,
							)
							.into()
						},
					// Association will be created during the transaction
					None => {},
				}

				// Validity Error if there are no funds for this recipient
				match RecipientFunds::<T>::get(airdrop_id, remote_account.clone()) {
					None => InvalidTransaction::Custom(ValidityError::NoFunds as u8).into(),
					Some(fund) if fund.total.is_zero() =>
						InvalidTransaction::Custom(ValidityError::NoFunds as u8).into(),
					Some(_) => ValidTransaction::with_tag_prefix("AirdropAssociationCheck")
						.and_provides(remote_account)
						.build(),
				}
			} else {
				// Only allow unsigned transactions for `claim`
				Err(InvalidTransaction::Call.into())
			}
		}
	}

	pub enum ValidityError {
		InvalidProof,
		AlreadyAssociated,
		NoFunds,
		NotClaimable,
		NotAnAirdrop,
	}
}
