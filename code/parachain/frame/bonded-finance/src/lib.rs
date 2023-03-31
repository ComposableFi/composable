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
)] // allow in tests#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	bad_style,
	bare_trait_objects,
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
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarks;

mod mock;
mod tests;
pub mod weights;

pub use crate::weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{
				increment::{Increment, SafeIncrement},
				start_at::ZeroInit,
			},
		},
		math::safe::{safe_multiply_by_rational, SafeAdd},
		validation::Validated,
	};
	use composable_traits::bonded_finance::{
		BondDuration, BondOffer, BondedFinance, ValidBondOffer,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{self, Inspect as FungibleInspect, Transfer as FungibleTransfer},
			fungibles::{self, Inspect as FungiblesInspect, Transfer as FungiblesTransfer},
		},
		transactional, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use pallet_vesting::{
		VestedTransfer, VestingScheduleInfo, VestingWindow::BlockNumberBased,
	};
	use scale_info::TypeInfo;
	use sp_runtime::traits::{AccountIdConversion, BlockNumberProvider, Convert, One, Zero};
	use sp_std::fmt::Debug;

	use crate::weights::WeightInfo;

	pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> =
		<<T as Config>::Currency as FungiblesInspect<AccountIdOf<T>>>::AssetId;
	pub(crate) type BalanceOf<T> =
		<<T as Config>::Currency as FungiblesInspect<AccountIdOf<T>>>::Balance;
	pub(crate) type NativeBalanceOf<T> =
		<<T as Config>::NativeCurrency as FungibleInspect<AccountIdOf<T>>>::Balance;
	pub(crate) type BondOfferOf<T> =
		BondOffer<AccountIdOf<T>, AssetIdOf<T>, BalanceOf<T>, BlockNumberOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new offer has been created.
		NewOffer { offer_id: T::BondOfferId, beneficiary: AccountIdOf<T> },
		/// A new bond has been registered.
		NewBond { offer_id: T::BondOfferId, who: AccountIdOf<T>, nb_of_bonds: BalanceOf<T> },
		/// An offer has been cancelled by the `AdminOrigin`.
		OfferCancelled { offer_id: T::BondOfferId },
		/// An offer has been completed.
		OfferCompleted { offer_id: T::BondOfferId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The offer could not be found.
		BondOfferNotFound,
		/// Someone tried  to submit an invalid offer.
		InvalidBondOffer,
		/// Someone tried to bond an already completed offer.
		OfferCompleted,
		/// Someone tried to bond with an invalid number of nb_of_bonds.
		InvalidNumberOfBonds,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The native currency, used for the stake required to create an offer.
		type NativeCurrency: fungible::Mutate<AccountIdOf<Self>>
			+ fungible::Transfer<AccountIdOf<Self>>;

		/// The multi currency system offers are based on.
		type Currency: fungibles::Mutate<AccountIdOf<Self>> + FungiblesTransfer<AccountIdOf<Self>>;

		/// The dependency managing vesting transfer of rewards.
		type Vesting: VestedTransfer<
			AssetId = AssetIdOf<Self>,
			AccountId = AccountIdOf<Self>,
			BlockNumber = BlockNumberOf<Self>,
			Balance = BalanceOf<Self>,
		>;

		/// The ID of a bond offer.
		type BondOfferId: Copy
			+ Clone
			+ Eq
			+ Debug
			+ Zero
			+ SafeAdd
			+ One
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo;

		/// The dependency managing conversion of balance to u128 required for reward computation.
		type Convert: Convert<BalanceOf<Self>, u128> + Convert<u128, BalanceOf<Self>>;

		#[pallet::constant]
		/// The pallet ID, required to create sub-accounts used by offers.
		type PalletId: Get<PalletId>;

		/// The stake a user has to put to create an offer.
		#[pallet::constant]
		type Stake: Get<NativeBalanceOf<Self>>;

		/// The minimum reward for an offer.
		///
		/// Must be > T::Vesting::MinVestedTransfer.
		// NOTE: can be zero for low amount tokens. either define normalized (e.g. to stable or
		// native token), or better have min per bond setup (if min == total will make Sell type
		// setup)
		#[pallet::constant]
		type MinReward: Get<BalanceOf<Self>>;

		/// The origin that is allowed to cancel bond offers.
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weights
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The counter used to uniquely identify bond offers within this pallet.
	#[pallet::storage]
	#[pallet::getter(fn bond_offer_count)]
	#[allow(clippy::disallowed_types)] // nonce, ValueQuery is OK
	pub type BondOfferCount<T: Config> =
		StorageValue<_, T::BondOfferId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// A mapping from offer ID to the pair: (issuer, offer)
	#[pallet::storage]
	#[pallet::getter(fn offers)]
	pub type BondOffers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::BondOfferId,
		(AccountIdOf<T>, BondOfferOf<T>),
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new bond offer. To be `bond` to later.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must have the
		/// appropriate funds to stake the offer.
		///
		/// Allows the issuer to ask for their account to be kept alive using the `keep_alive`
		/// parameter.
		///
		/// Emits a `NewOffer`.
		#[pallet::weight(T::WeightInfo::offer())]
		pub fn offer(
			origin: OriginFor<T>,
			offer: Validated<
				BondOfferOf<T>,
				ValidBondOffer<T::MinReward, <T::Vesting as VestedTransfer>::MinVestedTransfer>,
			>,
			keep_alive: bool,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let value = offer.value();
			Self::do_offer(&from, value, keep_alive)?;
			Ok(())
		}
		/// Bond to an offer.
		///
		/// The issuer should provide the number of contracts they are willing to buy.
		/// Once there are no more contracts available on the offer, the `stake` put by the
		/// offer creator is refunded.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must have the
		/// appropriate funds to buy the desired number of contracts.
		///
		/// Allows the issuer to ask for their account to be kept alive using the `keep_alive`
		/// parameter.
		///
		/// Emits a `NewBond`.
		/// Possibly Emits a `OfferCompleted`.
		#[pallet::weight(T::WeightInfo::bond())]
		pub fn bond(
			origin: OriginFor<T>,
			offer_id: T::BondOfferId,
			nb_of_bonds: BalanceOf<T>,
			keep_alive: bool,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_bond(offer_id, &from, nb_of_bonds, keep_alive)?;
			Ok(())
		}

		/// Cancel a running offer.
		///
		/// Blocking further bonds but not cancelling the currently vested rewards. The `stake` put
		/// by the offer creator is refunded.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must be `AdminOrigin`
		///
		/// Emits a `OfferCancelled`.
		#[pallet::weight(T::WeightInfo::cancel())]
		#[transactional]
		pub fn cancel(origin: OriginFor<T>, offer_id: T::BondOfferId) -> DispatchResult {
			let (issuer, offer) = Self::get_offer(offer_id)?;
			match (ensure_signed(origin.clone()), T::AdminOrigin::ensure_origin(origin)) {
				// Continue on admin origin
				(_, Ok(_)) => {},
				// Only issuer is allowed
				(Ok(account), _) =>
					if issuer != account {
						return Err(DispatchError::BadOrigin)
					},
				_ => return Err(DispatchError::BadOrigin),
			};
			let offer_account = Self::account_id(offer_id);
			// NOTE(hussein-aitlahcen): no need to keep the offer account alive
			T::NativeCurrency::transfer(&offer_account, &issuer, T::Stake::get(), false)?;
			T::Currency::transfer(
				offer.reward.asset,
				&offer_account,
				&issuer,
				offer.reward.amount,
				false,
			)?;
			BondOffers::<T>::remove(offer_id);
			Self::deposit_event(Event::<T>::OfferCancelled { offer_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_offer(
			offer_id: T::BondOfferId,
		) -> Result<(AccountIdOf<T>, BondOfferOf<T>), DispatchError> {
			BondOffers::<T>::try_get(offer_id).map_err(|_| Error::<T>::BondOfferNotFound.into())
		}

		#[transactional]
		pub fn do_offer(
			from: &AccountIdOf<T>,
			offer: BondOfferOf<T>,
			keep_alive: bool,
		) -> Result<T::BondOfferId, DispatchError> {
			let offer_id = BondOfferCount::<T>::increment()?;
			let beneficiary = offer.beneficiary.clone();
			let offer_account = Self::account_id(offer_id);
			T::NativeCurrency::transfer(from, &offer_account, T::Stake::get(), keep_alive)?;
			T::Currency::transfer(
				offer.reward.asset,
				from,
				&offer_account,
				offer.reward.amount,
				keep_alive,
			)?;
			BondOffers::<T>::insert(offer_id, (from.clone(), offer));
			Self::deposit_event(Event::<T>::NewOffer { offer_id, beneficiary });
			Ok(offer_id)
		}

		#[transactional]
		pub fn do_bond(
			offer_id: T::BondOfferId,
			from: &AccountIdOf<T>,
			nb_of_bonds: BalanceOf<T>,
			keep_alive: bool,
		) -> Result<BalanceOf<T>, DispatchError> {
			BondOffers::<T>::try_mutate(offer_id, |offer| {
				match offer.as_mut() {
					None => Err(Error::<T>::BondOfferNotFound.into()),
					Some((issuer, offer)) => {
						ensure!(
							offer.nb_of_bonds > BalanceOf::<T>::zero(),
							Error::<T>::OfferCompleted
						);
						ensure!(
							nb_of_bonds > BalanceOf::<T>::zero() &&
								nb_of_bonds <= offer.nb_of_bonds,
							Error::<T>::InvalidNumberOfBonds
						);
						// can't overflow, subsumed by `offer.valid()` in
						// `do_offer`
						let value = nb_of_bonds * offer.bond_price;
						let reward_share = T::Convert::convert(safe_multiply_by_rational(
							T::Convert::convert(nb_of_bonds),
							T::Convert::convert(offer.reward.amount),
							T::Convert::convert(offer.nb_of_bonds),
						)?);
						let offer_account = Self::account_id(offer_id);
						T::Currency::transfer(
							offer.asset,
							from,
							&offer.beneficiary,
							value,
							keep_alive,
						)?;
						let current_block = frame_system::Pallet::<T>::current_block_number();
						// Schedule the vesting of the reward.
						T::Vesting::vested_transfer(
							offer.reward.asset,
							&offer_account,
							from,
							VestingScheduleInfo {
								window: BlockNumberBased {
									start: current_block,
									period: offer.reward.maturity,
								},
								period_count: 1,
								per_period: reward_share,
							},
						)?;
						match offer.maturity {
							BondDuration::Finite { return_in } => {
								// Schedule the return of the bonded amount
								T::Vesting::vested_transfer(
									offer.asset,
									&offer.beneficiary,
									from,
									VestingScheduleInfo {
										window: BlockNumberBased {
											start: current_block,
											period: return_in,
										},
										period_count: 1,
										per_period: value,
									},
								)?;
							},
							BondDuration::Infinite => {
								// the offer, the liquidity is never returned to the bonder, meaning
								// that the protocol is now owning the funds.
							},
						}
						// NOTE(hussein-aitlahcen): can't overflow as checked to be <=
						// offer.nb_of_bonds prior to this
						// Same goes for reward_share as nb_of_bonds * bond_price <= total_price is
						// checked by the `Validate` instance of `BondOffer`
						offer.nb_of_bonds -= nb_of_bonds;
						offer.reward.amount -= reward_share;
						let new_bond_event = || {
							Self::deposit_event(Event::<T>::NewBond {
								offer_id,
								who: from.clone(),
								nb_of_bonds,
							});
						};
						if offer.completed() {
							T::NativeCurrency::transfer(
								&offer_account,
								issuer,
								T::Stake::get(),
								// NOTE(hussein-aitlahcen): no need to keep the offer account alive
								false,
							)?;
							new_bond_event();
							Self::deposit_event(Event::<T>::OfferCompleted { offer_id });
						} else {
							new_bond_event();
						}
						Ok(reward_share)
					},
				}
			})
		}

		pub(crate) fn account_id(offer_id: T::BondOfferId) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account_truncating(offer_id)
		}
	}

	impl<T: Config> BondedFinance for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type BlockNumber = BlockNumberOf<T>;
		type BondOfferId = T::BondOfferId;
		type MinReward = T::MinReward;
		type MinVestedTransfer = <T::Vesting as VestedTransfer>::MinVestedTransfer;

		fn offer(
			from: &Self::AccountId,
			offer: Validated<
				BondOfferOf<T>,
				ValidBondOffer<Self::MinReward, Self::MinVestedTransfer>,
			>,
			keep_alive: bool,
		) -> Result<Self::BondOfferId, DispatchError> {
			Self::do_offer(from, offer.value(), keep_alive)
		}

		fn bond(
			offer: Self::BondOfferId,
			from: &Self::AccountId,
			nb_of_bonds: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			Self::do_bond(offer, from, nb_of_bonds, keep_alive)
		}
	}
}
