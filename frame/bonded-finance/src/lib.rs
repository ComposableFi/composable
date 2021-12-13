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

//! Bonded Finance pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! A simple pallet providing means of submitting bond offers.
//!
//! ## Interface
//!
//! This pallet implements the `BondedFinance` trait from `composable-traits`.
//!
//! ## Dispatchable Functions
//!
//! - `offer` - Register a new bond offer, allowing use to later bond it.
//! - `bond` - Bond to an offer, the user should provide the number of nb_of_bonds a user is willing
//!   to buy.
//! - `cancel_offer` - Cancel a running offer, blocking further bond but not cancelling the
//!   currently vested rewards.

mod mock;
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use composable_traits::{
		bonded_finance::{BondDuration, BondOffer, BondedFinance},
		math::WrappingNext,
		vesting::{VestedTransfer, VestingSchedule},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{Inspect as FungibleInspect, Transfer as FungibleTransfer},
			fungibles::{Inspect as FungiblesInspect, Transfer as FungiblesTransfer},
		},
		transactional, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use scale_info::TypeInfo;
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{AccountIdConversion, BlockNumberProvider, Convert, Zero},
		ArithmeticError,
	};
	use sp_std::fmt::Debug;

	pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> =
		<<T as Config>::Currency as FungiblesInspect<AccountIdOf<T>>>::AssetId;
	pub(crate) type BalanceOf<T> =
		<<T as Config>::Currency as FungiblesInspect<AccountIdOf<T>>>::Balance;
	pub(crate) type NativeBalanceOf<T> =
		<<T as Config>::NativeCurrency as FungibleInspect<AccountIdOf<T>>>::Balance;
	pub(crate) type BondOfferOf<T> = BondOffer<AssetIdOf<T>, BalanceOf<T>, BlockNumberOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new offer has been created.
		NewOffer { offer_id: T::BondOfferId },
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
		/// Not enough native currency to create a new offer.
		NotEnoughStake,
		/// Not enough asset to bond.
		NotEnoughAsset,
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
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The native currency, used for the stake required to create an offer.
		type NativeCurrency: FungibleTransfer<AccountIdOf<Self>>;

		/// The multi currency system offers are based on.
		type Currency: FungiblesTransfer<AccountIdOf<Self>>;

		/// The dependency managing vesting transfer of rewards.
		type Vesting: VestedTransfer<
			AssetId = AssetIdOf<Self>,
			AccountId = AccountIdOf<Self>,
			BlockNumber = BlockNumberOf<Self>,
			Balance = BalanceOf<Self>,
		>;

		/// The ID of a bond offer.
		type BondOfferId: Copy + Clone + Eq + Debug + Zero + WrappingNext + FullCodec + TypeInfo;

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
		type MinReward: Get<BalanceOf<Self>>;

		/// The origin that is allowed to cancel bond offers.
		type AdminOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn BondOfferOnEmpty<T: Config>() -> T::BondOfferId {
		T::BondOfferId::zero()
	}

	/// The counter used to uniquely identify bond offers within this pallet.
	#[pallet::storage]
	#[pallet::getter(fn bond_offer_count)]
	pub type BondOfferCount<T: Config> =
		StorageValue<_, T::BondOfferId, ValueQuery, BondOfferOnEmpty<T>>;

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
		/// Create a new offer.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must have the
		/// appropriate funds.
		///
		/// Emits a `NewOffer`.
		#[pallet::weight(10_000)]
		pub fn offer(origin: OriginFor<T>, offer: BondOfferOf<T>) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_offer(&from, offer)?;
			Ok(())
		}

		/// Bond to an offer.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must have the
		/// appropriate funds.
		///
		/// Emits a `NewBond`.
		/// Possibily Emits a `OfferCompleted`.
		#[pallet::weight(10_000)]
		pub fn bond(
			origin: OriginFor<T>,
			offer_id: T::BondOfferId,
			nb_of_bonds: BalanceOf<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_bond(offer_id, &from, nb_of_bonds)?;
			Ok(())
		}

		/// Cancel an offer.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must be `AdminOrigin`
		///
		/// Emits a `OfferCancelled`.
		#[pallet::weight(10_000)]
		pub fn cancel(origin: OriginFor<T>, offer_id: T::BondOfferId) -> DispatchResult {
			let (issuer, _) = Self::get_offer(offer_id)?;
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
			T::NativeCurrency::transfer(&offer_account, &issuer, T::Stake::get(), true)?;
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

		pub fn account_id(offer_id: T::BondOfferId) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(offer_id)
		}

		#[transactional]
		pub fn do_offer(
			from: &AccountIdOf<T>,
			offer: BondOfferOf<T>,
		) -> Result<T::BondOfferId, DispatchError> {
			ensure!(
				offer.valid(
					<T::Vesting as VestedTransfer>::MinVestedTransfer::get(),
					T::MinReward::get()
				),
				Error::<T>::InvalidBondOffer
			);
			let offer_id = BondOfferCount::<T>::mutate(|offer_id| {
				*offer_id = offer_id.next();
				*offer_id
			});
			let offer_account = Self::account_id(offer_id);
			T::NativeCurrency::transfer(from, &offer_account, T::Stake::get(), true)?;
			T::Currency::transfer(
				offer.reward.asset,
				from,
				&offer_account,
				offer.reward.amount,
				true,
			)?;
			BondOffers::<T>::insert(offer_id, (from.clone(), offer));
			Self::deposit_event(Event::<T>::NewOffer { offer_id });
			Ok(offer_id)
		}

		#[transactional]
		pub fn do_bond(
			offer_id: T::BondOfferId,
			from: &AccountIdOf<T>,
			nb_of_bonds: BalanceOf<T>,
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
						// NOTE(hussein-aitlahcen): can't overflow, subsumed by `offer.valid()` in
						// `do_offer`
						let value = nb_of_bonds * offer.bond_price;
						ensure!(
							T::Currency::can_withdraw(offer.asset, from, value)
								.into_result()
								.is_ok(),
							Error::<T>::NotEnoughAsset
						);
						let reward_share = T::Convert::convert(
							multiply_by_rational(
								T::Convert::convert(nb_of_bonds),
								T::Convert::convert(offer.reward.amount),
								T::Convert::convert(offer.nb_of_bonds),
							)
							.map_err(|_| ArithmeticError::Overflow)?,
						);
						let offer_account = Self::account_id(offer_id);
						T::Currency::transfer(offer.asset, from, &offer_account, value, true)?;
						let current_block = frame_system::Pallet::<T>::current_block_number();
						T::Vesting::vested_transfer(
							offer.reward.asset,
							&offer_account,
							from,
							VestingSchedule {
								start: current_block,
								period: offer.reward.maturity,
								period_count: 1,
								per_period: reward_share,
							},
						)?;
						match offer.maturity {
							BondDuration::Finite { return_in } => {
								T::Vesting::vested_transfer(
									offer.asset,
									&offer_account,
									from,
									VestingSchedule {
										start: current_block,
										period: return_in,
										period_count: 1,
										per_period: value,
									},
								)?;
							},
							BondDuration::Infinite => {
								// NOTE(hussein-aitlahcen): in the case of an inifite duration for
								// the offer, the liquidity is never returned to the bonder, meaning
								// that the protocol is now owning the funds.
							},
						}
						// NOTE(hussein-aitlahcen): can't overflow as checked to be <
						// offer.nb_of_bonds prior to this
						// Same goes for reward_share as nb_of_bonds * bond_price <= total_price
						(*offer).nb_of_bonds -= nb_of_bonds;
						(*offer).reward.amount -= reward_share;
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
								true,
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
	}

	impl<T: Config> BondedFinance for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type BlockNumber = BlockNumberOf<T>;
		type BondOfferId = T::BondOfferId;

		fn offer(
			from: &Self::AccountId,
			offer: BondOfferOf<T>,
		) -> Result<Self::BondOfferId, DispatchError> {
			Self::do_offer(from, offer)
		}

		fn bond(
			offer: Self::BondOfferId,
			from: &Self::AccountId,
			nb_of_bonds: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			Self::do_bond(offer, from, nb_of_bonds)
		}
	}
}
