//! # Transaction Fee Pallet
//!
//! Loosely based on https://github.com/paritytech/substrate/blob/master/frame/transaction-payment/src/lib.rs
//! but with added support for `MultiCurrency` using a `Dex` interface.

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use composable_traits::dex::SimpleExchange;
use pallet_transaction_payment_rpc_runtime_api::{FeeDetails, InclusionFee, RuntimeDispatchInfo};
use primitives::currency::CurrencyId;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{
		CheckedSub, Convert, DispatchInfoOf, Dispatchable, PostDispatchInfoOf, SaturatedConversion,
		Saturating, SignedExtension, Zero,
	},
	transaction_validity::{
		InvalidTransaction, TransactionPriority, TransactionValidity, TransactionValidityError,
		ValidTransaction,
	},
	DispatchError, FixedPointNumber, FixedPointOperand, Perbill,
};
use sp_std::prelude::*;
use support::{
	dispatch::DispatchResult,
	traits::{Currency, ExistenceRequirement, Get, Imbalance, OnUnbalanced, WithdrawReasons},
	weights::{
		DispatchClass, DispatchInfo, GetDispatchInfo, Pays, PostDispatchInfo, Weight,
		WeightToFeeCoefficient, WeightToFeePolynomial,
	},
};

pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

pub mod fee_adjustment;
use fee_adjustment::MultiplierUpdate;

// Balance of `T::NativeCurrency`
type BalanceOf<T> =
	<<T as Config>::NativeCurrency as Currency<<T as system::Config>::AccountId>>::Balance;

// negative imbalance of `T::NativeCurrency`
type NegativeImbalanceOf<T> = <<T as Config>::NativeCurrency as Currency<
	<T as system::Config>::AccountId,
>>::NegativeImbalance;

// positive imbalance of `T::NativeCurrency`
type PositiveImbalanceOf<T> = <<T as Config>::NativeCurrency as Currency<
	<T as system::Config>::AccountId,
>>::PositiveImbalance;

#[support::pallet]
pub mod pallet {
	use super::*;
	use crate::fee_adjustment::Multiplier;
	use primitives::currency::CurrencyId;
	use support::pallet_prelude::*;
	use system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: system::Config {
		/// Handler for withdrawing, refunding and depositing the transaction fee.
		/// Transaction fees are withdrawn before the transaction is executed.
		/// After the transaction was executed the transaction weight can be
		/// adjusted, depending on the used resources by the transaction. If the
		/// transaction weight is lower than expected, parts of the transaction fee
		/// might be refunded. In the end the fees can be deposited.
		type OnChargeTransaction: OnUnbalanced<
			<Self::NativeCurrency as Currency<Self::AccountId>>::NegativeImbalance,
		>;

		/// Native currency type.
		type NativeCurrency: Currency<Self::AccountId>;

		/// Dex interface for executing swaps
		type Dex: SimpleExchange<
			AssetId = CurrencyId,
			Balance = <Self::NativeCurrency as Currency<Self::AccountId>>::Balance,
			AccountId = Self::AccountId,
			Error = DispatchError,
		>;

		/// The fee to be paid for making a transaction; the per-byte portion.
		#[pallet::constant]
		type TransactionByteFee: Get<BalanceOf<Self>>;

		/// Convert a weight value into a deductible fee based on the currency type.
		type WeightToFee: WeightToFeePolynomial<Balance = BalanceOf<Self>>;

		/// Update the multiplier of the next block, based on the previous block's weight.
		type FeeMultiplierUpdate: MultiplierUpdate;
	}

	#[pallet::extra_constants]
	impl<T: Config> Pallet<T> {
		//TODO: rename to snake case after https://github.com/paritytech/substrate/issues/8826 fixed.
		#[allow(non_snake_case)]
		/// The polynomial that is applied in order to derive fee from weight.
		fn WeightToFee() -> Vec<WeightToFeeCoefficient<BalanceOf<T>>> {
			T::WeightToFee::polynomial().to_vec()
		}
	}

	#[pallet::type_value]
	pub fn NextFeeMultiplierOnEmpty() -> Multiplier {
		Multiplier::saturating_from_integer(1)
	}

	#[pallet::storage]
	#[pallet::getter(fn next_fee_multiplier)]
	// `NextFeeMultiplierOnEmpty` explicitly defines what happens on empty, so `ValueQuery` is
	// allowed.
	#[allow(clippy::disallowed_types)]
	pub type NextFeeMultiplier<T: Config> =
		StorageValue<_, Multiplier, ValueQuery, NextFeeMultiplierOnEmpty>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_: T::BlockNumber) {
			<NextFeeMultiplier<T>>::mutate(|fm| {
				*fm = T::FeeMultiplierUpdate::convert(*fm);
			});
		}

		// `integrity_test` is allowed to panic.
		#[allow(clippy::disallowed_methods, clippy::expect_used)]
		fn integrity_test() {
			// given weight == u64, we build multipliers from `diff` of two weight values, which can
			// at most be maximum block weight. Make sure that this can fit in a multiplier without
			// loss.
			use sp_std::convert::TryInto;
			assert!(
				<Multiplier as sp_runtime::traits::Bounded>::max_value() >=
					Multiplier::checked_from_integer(
						T::BlockWeights::get()
							.max_block
							.try_into()
							.expect("Blockweights.max_block should be present")
					)
					.expect("Multiplier from Blockweights should not overflow"),
			);

			// This is the minimum value of the multiplier. Make sure that if we collapse to this
			// value, we can recover with a reasonable amount of traffic. For this test we assert
			// that if we collapse to minimum, the trend will be positive with a weight value
			// which is 1% more than the target.
			let min_value = T::FeeMultiplierUpdate::min();
			let mut target = T::FeeMultiplierUpdate::target() *
				T::BlockWeights::get().get(DispatchClass::Normal).max_total.expect(
					"Setting `max_total` for `Normal` dispatch class is not compatible with \
					`transaction-payment` pallet.",
				);
			// add 1 percent;
			let addition = target / 100;
			if addition == 0 {
				// this is most likely because in a test setup we set everything to ().
				return
			}
			target += addition;

			#[cfg(any(feature = "std", test))]
			sp_io::TestExternalities::new_empty().execute_with(|| {
				<system::Pallet<T>>::set_block_consumed_resources(target, 0);
				let next = T::FeeMultiplierUpdate::convert(min_value);
				assert!(
					next > min_value,
					"The minimum bound of the multiplier is too low. When \
					block saturation is more than target by 1% and multiplier is minimal then \
					the multiplier doesn't increase."
				);
			});
		}
	}
}

impl<T: Config> Pallet<T>
where
	BalanceOf<T>: FixedPointOperand,
{
	/// Query the data that we know about the fee of a given `call`.
	///
	/// This pallet is not and cannot be aware of the internals of a signed extension, for example
	/// a tip. It only interprets the extrinsic as some encoded value and accounts for its weight
	/// and length, the runtime's extrinsic base weight, and the current fee multiplier.
	///
	/// All dispatchables must be annotated with weight and will have some fee info. This function
	/// always returns.
	pub fn query_info<E>(unchecked_extrinsic: E, len: u32) -> RuntimeDispatchInfo<BalanceOf<T>>
	where
		T::Call: Dispatchable<Info = DispatchInfo>,
		E: GetDispatchInfo,
	{
		// NOTE: we can actually make it understand `ChargeTransactionPayment`, but would be some
		// hassle for sure. We have to make it aware of the index of `ChargeTransactionPayment` in
		// `Extra`. Alternatively, we could actually execute the tx's per-dispatch and record the
		// balance of the sender before and after the pipeline.. but this is way too much hassle for
		// a very very little potential gain in the future.
		let dispatch_info = <E as GetDispatchInfo>::get_dispatch_info(&unchecked_extrinsic);

		let partial_fee = Self::compute_fee(len, &dispatch_info, 0_u32.into());
		let DispatchInfo { weight, class, .. } = dispatch_info;

		RuntimeDispatchInfo { weight, class, partial_fee }
	}

	/// Query the detailed fee of a given `call`.
	pub fn query_fee_details<E>(unchecked_extrinsic: E, len: u32) -> FeeDetails<BalanceOf<T>>
	where
		T::Call: Dispatchable<Info = DispatchInfo>,
		E: GetDispatchInfo,
	{
		let dispatch_info = <E as GetDispatchInfo>::get_dispatch_info(&unchecked_extrinsic);
		Self::compute_fee_details(len, &dispatch_info, 0_u32.into())
	}

	/// Compute the final fee value for a particular transaction.
	pub fn compute_fee(len: u32, info: &DispatchInfoOf<T::Call>, tip: BalanceOf<T>) -> BalanceOf<T>
	where
		T::Call: Dispatchable<Info = DispatchInfo>,
	{
		Self::compute_fee_details(len, info, tip).final_fee()
	}

	/// Compute the fee details for a particular transaction.
	pub fn compute_fee_details(
		len: u32,
		info: &DispatchInfoOf<T::Call>,
		tip: BalanceOf<T>,
	) -> FeeDetails<BalanceOf<T>>
	where
		T::Call: Dispatchable<Info = DispatchInfo>,
	{
		Self::compute_fee_raw(len, info.weight, tip, info.pays_fee, info.class)
	}

	/// Compute the actual post dispatch fee for a particular transaction.
	///
	/// Identical to `compute_fee` with the only difference that the post dispatch corrected
	/// weight is used for the weight fee calculation.
	pub fn compute_actual_fee(
		len: u32,
		info: &DispatchInfoOf<T::Call>,
		post_info: &PostDispatchInfoOf<T::Call>,
		tip: BalanceOf<T>,
	) -> BalanceOf<T>
	where
		T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	{
		Self::compute_actual_fee_details(len, info, post_info, tip).final_fee()
	}

	/// Compute the actual post dispatch fee details for a particular transaction.
	pub fn compute_actual_fee_details(
		len: u32,
		info: &DispatchInfoOf<T::Call>,
		post_info: &PostDispatchInfoOf<T::Call>,
		tip: BalanceOf<T>,
	) -> FeeDetails<BalanceOf<T>>
	where
		T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	{
		Self::compute_fee_raw(
			len,
			post_info.calc_actual_weight(info),
			tip,
			post_info.pays_fee(info),
			info.class,
		)
	}

	fn compute_fee_raw(
		len: u32,
		weight: Weight,
		tip: BalanceOf<T>,
		pays_fee: Pays,
		class: DispatchClass,
	) -> FeeDetails<BalanceOf<T>> {
		if pays_fee == Pays::Yes {
			let len = <BalanceOf<T>>::from(len);
			let per_byte = T::TransactionByteFee::get();

			// length fee. this is not adjusted.
			let fixed_len_fee = per_byte.saturating_mul(len);

			// the adjustable part of the fee.
			let unadjusted_weight_fee = Self::weight_to_fee(weight);
			let multiplier = Self::next_fee_multiplier();
			// final adjusted weight fee.
			let adjusted_weight_fee = multiplier.saturating_mul_int(unadjusted_weight_fee);

			let base_fee = Self::weight_to_fee(T::BlockWeights::get().get(class).base_extrinsic);
			FeeDetails {
				inclusion_fee: Some(InclusionFee {
					base_fee,
					len_fee: fixed_len_fee,
					adjusted_weight_fee,
				}),
				tip,
			}
		} else {
			FeeDetails { inclusion_fee: None, tip }
		}
	}

	fn weight_to_fee(weight: Weight) -> BalanceOf<T> {
		// cap the weight to the maximum defined in runtime, otherwise it will be the
		// `Bounded` maximum of its data type, which is not desired.
		let capped_weight = weight.min(T::BlockWeights::get().max_block);
		T::WeightToFee::calc(&capped_weight)
	}

	fn can_pay_fee(
		who: &T::AccountId,
		fee: BalanceOf<T>,
		reason: WithdrawReasons,
		slippage: &Perbill,
		asset_id: &Option<CurrencyId>,
	) -> Result<(), DispatchError> {
		let native_existential_deposit = T::NativeCurrency::minimum_balance();
		let total_native = T::NativeCurrency::total_balance(who);

		// check native balance if is enough
		let native_is_enough = fee.saturating_add(native_existential_deposit) <= total_native &&
			T::NativeCurrency::free_balance(who).checked_sub(&fee).map_or(
				false,
				|new_free_balance| {
					T::NativeCurrency::ensure_can_withdraw(who, fee, reason, new_free_balance)
						.is_ok()
				},
			);

		// native is not enough, try swap native to pay fee
		match (asset_id, native_is_enough) {
			// user specified some asset to pay and they dont have enough native tokens to pay
			(Some(asset_id), false) => {
				// add extra gap to keep alive after swap
				let amount =
					fee.saturating_add(native_existential_deposit.saturating_sub(total_native));
				T::Dex::exchange(
					*asset_id,
					who.clone(),
					CurrencyId::LAYR,
					who.clone(),
					amount,
					*slippage,
				)?;
			},
			// user didn't specify some asset to pay and they dont have enough native tokens to pay
			(None, false) => return Err(DispatchError::Other("Not enough tokens")),
			// they have enough native tokens to pay.
			_ => {},
		}

		Ok(())
	}
}

impl<T> Convert<Weight, BalanceOf<T>> for Pallet<T>
where
	T: Config,
	BalanceOf<T>: FixedPointOperand,
{
	/// Compute the fee for the specified weight.
	///
	/// This fee is already adjusted by the per block fee adjustment factor and is therefore the
	/// share that the weight contributes to the overall fee of a transaction. It is mainly
	/// for informational purposes and not used in the actual fee calculation.
	fn convert(weight: Weight) -> BalanceOf<T> {
		<NextFeeMultiplier<T>>::get().saturating_mul_int(Self::weight_to_fee(weight))
	}
}

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChargeTransactionFee<T: Config>(
	// tip
	#[codec(compact)] BalanceOf<T>,
	// max slippage
	Perbill,
	// token to pay fee with, defaults to native
	Option<CurrencyId>,
);

impl<T: Config> ChargeTransactionFee<T>
where
	T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	BalanceOf<T>: Send + Sync + FixedPointOperand,
{
	/// utility constructor. Used only in client/factory code.
	pub fn from(tip: BalanceOf<T>, slippage: Perbill, asset_id: Option<CurrencyId>) -> Self {
		Self(tip, slippage, asset_id)
	}

	/// Returns the tip as being choosen by the transaction sender.
	pub fn tip(&self) -> BalanceOf<T> {
		self.0
	}

	fn withdraw_fee(
		&self,
		who: &T::AccountId,
		info: &DispatchInfoOf<T::Call>,
		len: usize,
	) -> Result<(BalanceOf<T>, Option<NegativeImbalanceOf<T>>), TransactionValidityError> {
		let ChargeTransactionFee(tip, slippage, asset_id) = self;
		let fee = Pallet::<T>::compute_fee(len as u32, info, *tip);

		// Only mess with balances if fee is not zero.
		if fee.is_zero() {
			return Ok((fee, None))
		}

		let reason = if tip.is_zero() {
			WithdrawReasons::TRANSACTION_PAYMENT
		} else {
			WithdrawReasons::TRANSACTION_PAYMENT | WithdrawReasons::TIP
		};

		Pallet::<T>::can_pay_fee(who, fee, reason, slippage, asset_id)
			.map_err(|_| InvalidTransaction::Payment)?;

		// withdraw native currency as fee
		match T::NativeCurrency::withdraw(who, fee, reason, ExistenceRequirement::KeepAlive) {
			Ok(imbalance) => Ok((fee, Some(imbalance))),
			Err(_) => Err(InvalidTransaction::Payment.into()),
		}
	}

	/// Get an appropriate priority for a transaction with the given length and info.
	///
	/// This will try and optimize the `fee/weight` `fee/length`, whichever is consuming more of the
	/// maximum corresponding limit.
	///
	/// For example, if a transaction consumed 1/4th of the block length and half of the weight, its
	/// final priority is `fee * min(2, 4) = fee * 2`. If it consumed `1/4th` of the block length
	/// and the entire block weight `(1/1)`, its priority is `fee * min(1, 4) = fee * 1`. This means
	///  that the transaction which consumes more resources (either length or weight) with the same
	/// `fee` ends up having lower priority.
	fn priority(
		len: usize,
		info: &DispatchInfoOf<T::Call>,
		final_fee: BalanceOf<T>,
	) -> TransactionPriority {
		let weight_saturation = T::BlockWeights::get().max_block / info.weight.max(1);
		let max_block_length = *T::BlockLength::get().max.get(DispatchClass::Normal);
		let len_saturation = max_block_length as u64 / (len as u64).max(1);
		let coefficient: BalanceOf<T> =
			weight_saturation.min(len_saturation).saturated_into::<BalanceOf<T>>();
		final_fee.saturating_mul(coefficient).saturated_into::<TransactionPriority>()
	}
}

impl<T: Config> sp_std::fmt::Debug for ChargeTransactionFee<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "ChargeTransactionPayment<{:?}>", self.0)
	}
	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T: Config> SignedExtension for ChargeTransactionFee<T>
where
	BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
	T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo> + TypeInfo,
{
	const IDENTIFIER: &'static str = "ChargeTransactionFee";
	type AccountId = T::AccountId;
	type Call = T::Call;
	type AdditionalSigned = ();
	type Pre = (
		// tip
		BalanceOf<T>,
		// who paid the fee
		Self::AccountId,
		// imbalance from withdrawing the fee
		Option<NegativeImbalanceOf<T>>,
		// actual fee value
		BalanceOf<T>,
	);
	fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
		Ok(())
	}

	fn validate(
		&self,
		who: &Self::AccountId,
		_call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		let (fee, _) = self.withdraw_fee(who, info, len)?;
		Ok(ValidTransaction { priority: Self::priority(len, info, fee), ..Default::default() })
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		_call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		let (fee, imbalance) = self.withdraw_fee(who, info, len)?;
		Ok((self.0, who.clone(), imbalance, fee))
	}

	fn post_dispatch(
		pre: Option<Self::Pre>,
		info: &DispatchInfoOf<Self::Call>,
		post_info: &PostDispatchInfoOf<Self::Call>,
		len: usize,
		_result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		if let Some((tip, who, Some(paid), fee)) = pre {
			let actual_fee = Pallet::<T>::compute_actual_fee(len as u32, info, post_info, tip);
			let refund = fee.saturating_sub(actual_fee);
			// refund to the the account that paid the fees. If this fails, the
			// account might have dropped below the existential balance. In
			// that case we don't refund anything.
			let refund_imbalance =
				<T::NativeCurrency as Currency<T::AccountId>>::deposit_into_existing(&who, refund)
					.unwrap_or_else(|_| PositiveImbalanceOf::<T>::zero());
			// merge the imbalance caused by paying the fees and refunding parts of it again.
			let actual_payment =
				paid.offset(refund_imbalance).same().map_err(|_| InvalidTransaction::Payment)?;
			let (tip, fee) = actual_payment.split(tip);

			// distribute fee
			T::OnChargeTransaction::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));
		}
		Ok(())
	}
}
