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
		NewOffer { offer: T::BondOfferId },
		/// Someone did bond to an offer.
		NewBond { offer: T::BondOfferId, who: AccountIdOf<T>, bonded_amount: BalanceOf<T> },
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
		/// Someone tried to bond with an invalid amount.
		InvalidBondAmount,
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
		type MinReward: Get<BalanceOf<Self>>;

		/// The minimum asset amount for an offer.
		type MinOffer: Get<BalanceOf<Self>>;

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

	/// A mapping from offer ID to the pair: (creator, offer)
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
		#[pallet::weight(10_000)]
		pub fn offer(origin: OriginFor<T>, offer: BondOfferOf<T>) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_offer(&from, offer)?;
			Ok(())
		}

		/// Bond to an existing offer.
		#[pallet::weight(10_000)]
		pub fn bond(
			origin: OriginFor<T>,
			offer: T::BondOfferId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_bond(offer, &from, amount)?;
			Ok(())
		}

		/// Cancel an existing offer.
		#[pallet::weight(10_000)]
		pub fn cancel_offer(origin: OriginFor<T>, offer: T::BondOfferId) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_cancel_offer(offer)
		}
	}

	impl<T: Config> Pallet<T> {
		fn account_id(offer: T::BondOfferId) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(offer)
		}

		pub fn do_cancel_offer(offer_id: T::BondOfferId) -> DispatchResult {
			let (creator, _) =
				BondOffers::<T>::try_get(offer_id).map_err(|_| Error::<T>::BondOfferNotFound)?;
			let offer_account = Self::account_id(offer_id);
			T::NativeCurrency::transfer(&offer_account, &creator, T::Stake::get(), true)?;
			BondOffers::<T>::remove(offer_id);
			Ok(())
		}

		#[transactional]
		pub fn do_offer(
			from: &AccountIdOf<T>,
			offer: BondOfferOf<T>,
		) -> Result<T::BondOfferId, DispatchError> {
			ensure!(
				offer.valid(T::MinOffer::get(), T::MinReward::get()),
				Error::<T>::InvalidBondOffer
			);
			let offer_id = BondOfferCount::<T>::mutate(|offer_id| {
				*offer_id = offer_id.next();
				*offer_id
			});
			let offer_account = Self::account_id(offer_id);
			// TODO(hussein-aitlahcen): when should we rapatriate the stake?
			T::NativeCurrency::transfer(from, &offer_account, T::Stake::get(), true)?;
			T::Currency::transfer(
				offer.reward_asset,
				from,
				&offer_account,
				offer.reward_amount,
				true,
			)?;
			BondOffers::<T>::insert(offer_id, (from.clone(), offer));
			Self::deposit_event(Event::<T>::NewOffer { offer: offer_id });
			Ok(offer_id)
		}

		#[transactional]
		pub fn do_bond(
			offer_id: T::BondOfferId,
			from: &AccountIdOf<T>,
			amount: BalanceOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			BondOffers::<T>::try_mutate(offer_id, |offer| {
				offer
					.as_mut()
					.map(|(_, offer)| {
						ensure!(amount <= offer.amount, Error::<T>::InvalidBondAmount);
						ensure!(
							T::Currency::can_withdraw(offer.asset, from, amount)
								.into_result()
								.is_ok(),
							Error::<T>::NotEnoughAsset
						);
						let offer_account = Self::account_id(offer_id);
						T::Currency::transfer(offer.asset, from, &offer_account, amount, true)?;
						let reward_share = T::Convert::convert(
							multiply_by_rational(
								T::Convert::convert(amount),
								T::Convert::convert(offer.reward_amount),
								T::Convert::convert(offer.amount),
							)
							.map_err(|_| ArithmeticError::Overflow)?,
						);
						let block = frame_system::Pallet::<T>::current_block_number();
						T::Vesting::vested_transfer(
							offer.reward_asset,
							&offer_account,
							from,
							VestingSchedule {
								start: block,
								period: offer.reward_duration,
								period_count: 1,
								per_period: reward_share,
							},
							true,
						)?;
						match offer.duration {
							BondDuration::Finite { blocks } => {
								T::Vesting::vested_transfer(
									offer.asset,
									&offer_account,
									from,
									VestingSchedule {
										start: block,
										period: blocks,
										period_count: 1,
										per_period: amount,
									},
									true,
								)?;
							},
							BondDuration::Infinite => {
								// the protocol now owns the liquidity
							},
						}
						(*offer).amount -= amount;
						(*offer).reward_amount -= reward_share;
						Self::deposit_event(Event::<T>::NewBond {
							offer: offer_id,
							who: from.clone(),
							bonded_amount: amount,
						});
						Ok(reward_share)
					})
					.unwrap_or_else(|| Err(Error::<T>::BondOfferNotFound.into()))
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
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			Self::do_bond(offer, from, amount)
		}
	}
}
