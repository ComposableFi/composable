//! # Transaction Fee Pallet
//!
//! Loosely based on https://github.com/paritytech/substrate/blob/master/frame/transaction-payment/src/lib.rs
//! but with added support for `MultiCurrency` using a `Dex` interface.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use composable_traits::dex::SimpleExchange;
use pallet_transaction_payment_rpc_runtime_api::{FeeDetails, InclusionFee, RuntimeDispatchInfo};
use primitives::currency::CurrencyId;
use sp_runtime::{
	traits::{
		CheckedSub, Convert, DispatchInfoOf, Dispatchable, PostDispatchInfoOf, SaturatedConversion,
		Saturating, SignedExtension, Zero,
	},
	transaction_validity::{
		InvalidTransaction, TransactionPriority, TransactionValidity, TransactionValidityError,
		ValidTransaction,
	},
	FixedPointNumber, FixedPointOperand, Perbill,
};
use sp_std::prelude::*;
use support::{
	dispatch::DispatchResult,
	traits::{
		Currency, ExistenceRequirement, Get, Imbalance, OnUnbalanced, SameOrOther, WithdrawReasons,
	},
	weights::{
		DispatchClass, DispatchInfo, GetDispatchInfo, Pays, PostDispatchInfo, Weight,
		WeightToFeeCoefficient, WeightToFeePolynomial,
	},
};

pub use pallet::*;
pub mod fee_adjustment;
use fee_adjustment::MultiplierUpdate;

// Balance of `T::NativeCurrency`
type BalanceOf<T> =
	<<T as Config>::NativeCurrency as Currency<<T as system::Config>::AccountId>>::Balance;

// negative imbalance of `T::NativeCurrency`
type NegativeImbalanceOf<T> = <<T as Config>::NativeCurrency as Currency<
	<T as system::Config>::AccountId,
>>::NegativeImbalance;

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
			Error = (),
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
	pub type NextFeeMultiplier<T: Config> =
		StorageValue<_, Multiplier, ValueQuery, NextFeeMultiplierOnEmpty>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_: T::BlockNumber) {
			<NextFeeMultiplier<T>>::mutate(|fm| {
				*fm = T::FeeMultiplierUpdate::convert(*fm);
			});
		}

		fn integrity_test() {
			// given weight == u64, we build multipliers from `diff` of two weight values, which can
			// at most be maximum block weight. Make sure that this can fit in a multiplier without
			// loss.
			use sp_std::convert::TryInto;
			assert!(
				<Multiplier as sp_runtime::traits::Bounded>::max_value()
					>= Multiplier::checked_from_integer(
						T::BlockWeights::get().max_block.try_into().unwrap()
					)
					.unwrap(),
			);

			// This is the minimum value of the multiplier. Make sure that if we collapse to this
			// value, we can recover with a reasonable amount of traffic. For this test we assert
			// that if we collapse to minimum, the trend will be positive with a weight value
			// which is 1% more than the target.
			let min_value = T::FeeMultiplierUpdate::min();
			let mut target = T::FeeMultiplierUpdate::target()
				* T::BlockWeights::get().get(DispatchClass::Normal).max_total.expect(
					"Setting `max_total` for `Normal` dispatch class is not compatible with \
					`transaction-payment` pallet.",
				);
			// add 1 percent;
			let addition = target / 100;
			if addition == 0 {
				// this is most likely because in a test setup we set everything to ().
				return;
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

		let partial_fee = Self::compute_fee(len, &dispatch_info, 0u32.into());
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
		Self::compute_fee_details(len, &dispatch_info, 0u32.into())
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
	) -> Result<(), ()> {
		let native_existential_deposit = T::NativeCurrency::minimum_balance();
		let total_native = T::NativeCurrency::total_balance(who);

		// check native balance if is enough
		let native_is_enough = fee.saturating_add(native_existential_deposit) <= total_native
			&& T::NativeCurrency::free_balance(who).checked_sub(&fee).map_or(
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
				let native_asset_id = Default::default();
				// add extra gap to keep alive after swap
				let amount =
					fee.saturating_add(native_existential_deposit.saturating_sub(total_native));
				T::Dex::exchange(
					*asset_id,
					who.clone(),
					native_asset_id,
					who.clone(),
					amount,
					*slippage,
				)?;
			}
			// user didn't specify some asset to pay and they dont have enough native tokens to pay
			(None, false) => return Err(()),
			// they have enough native tokens to pay.
			_ => {}
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
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
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
			return Ok((fee, None));
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
	/// This will try and optimise the `fee/weight` `fee/length`, whichever is consuming more of the
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
	T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
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
		pre: Self::Pre,
		info: &DispatchInfoOf<Self::Call>,
		post_info: &PostDispatchInfoOf<Self::Call>,
		len: usize,
		_result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		let (tip, who, imbalance, fee) = pre;
		if let Some(payed) = imbalance {
			let actual_fee = Pallet::<T>::compute_actual_fee(len as u32, info, post_info, tip);
			let refund = fee.saturating_sub(actual_fee);
			let actual_payment = match T::NativeCurrency::deposit_into_existing(&who, refund) {
				Ok(refund_imbalance) => {
					// The refund cannot be larger than the up front payed max weight.
					// `PostDispatchInfo::calc_unspent` guards against such a case.
					match payed.offset(refund_imbalance) {
						SameOrOther::Same(actual_payment) => actual_payment,
						SameOrOther::None => Default::default(),
						_ => return Err(InvalidTransaction::Payment.into()),
					}
				}
				// We do not recreate the account using the refund. The up front payment
				// is gone in that case.
				Err(_) => payed,
			};
			let (tip, fee) = actual_payment.split(tip);

			// distribute fee
			T::OnChargeTransaction::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate as pallet_transaction_payment;

	use std::cell::RefCell;

	use codec::Encode;
	use smallvec::smallvec;

	use sp_core::H256;
	use sp_runtime::{
		testing::{Header, TestXt},
		traits::{BlakeTwo256, IdentityLookup, One},
		transaction_validity::InvalidTransaction,
		Perbill,
	};

	use pallet_balances::Call as BalancesCall;
	use support::{
		assert_noop, assert_ok, parameter_types,
		traits::{Currency, Imbalance, OnUnbalanced},
		weights::{
			DispatchClass, DispatchInfo, GetDispatchInfo, PostDispatchInfo, Weight,
			WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
		},
	};
	use system;

	type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Runtime>;
	type Block = system::mocking::MockBlock<Runtime>;

	support::construct_runtime!(
		pub enum Runtime where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: system::{Pallet, Call, Config, Storage, Event<T>},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
		}
	);

	const CALL: &<Runtime as system::Config>::Call = &Call::Balances(BalancesCall::transfer(2, 69));

	thread_local! {
		static EXTRINSIC_BASE_WEIGHT: RefCell<u64> = RefCell::new(0);
	}

	pub struct BlockWeights;
	impl Get<system::limits::BlockWeights> for BlockWeights {
		fn get() -> system::limits::BlockWeights {
			system::limits::BlockWeights::builder()
				.base_block(0)
				.for_class(DispatchClass::all(), |weights| {
					weights.base_extrinsic = EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow()).into();
				})
				.for_class(DispatchClass::non_mandatory(), |weights| {
					weights.max_total = 1024.into();
				})
				.build_or_panic()
		}
	}

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub static TransactionByteFee: u64 = 1;
		pub static WeightToFee: u64 = 1;
	}

	impl system::Config for Runtime {
		type BaseCallFilter = ();
		type BlockWeights = BlockWeights;
		type BlockLength = ();
		type DbWeight = ();
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Call = Call;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
	}

	impl pallet_balances::Config for Runtime {
		type Balance = u64;
		type Event = Event;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type WeightInfo = ();
	}

	impl WeightToFeePolynomial for WeightToFee {
		type Balance = u64;

		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				coeff_frac: Perbill::zero(),
				coeff_integer: WEIGHT_TO_FEE.with(|v| *v.borrow()),
				negative: false,
			}]
		}
	}

	thread_local! {
		static TIP_UNBALANCED_AMOUNT: RefCell<u64> = RefCell::new(0);
		static FEE_UNBALANCED_AMOUNT: RefCell<u64> = RefCell::new(0);
	}

	pub struct DealWithFees;
	impl OnUnbalanced<pallet_balances::NegativeImbalance<Runtime>> for DealWithFees {
		fn on_unbalanceds<B>(
			mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<Runtime>>,
		) {
			if let Some(fees) = fees_then_tips.next() {
				FEE_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += fees.peek());
				if let Some(tips) = fees_then_tips.next() {
					TIP_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += tips.peek());
				}
			}
		}
	}

	impl Config for Runtime {
		type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
		type TransactionByteFee = TransactionByteFee;
		type WeightToFee = WeightToFee;
		type FeeMultiplierUpdate = ();
	}

	pub struct ExtBuilder {
		balance_factor: u64,
		base_weight: u64,
		byte_fee: u64,
		weight_to_fee: u64,
	}

	impl Default for ExtBuilder {
		fn default() -> Self {
			Self { balance_factor: 1, base_weight: 0, byte_fee: 1, weight_to_fee: 1 }
		}
	}

	impl ExtBuilder {
		pub fn base_weight(mut self, base_weight: u64) -> Self {
			self.base_weight = base_weight;
			self
		}
		pub fn byte_fee(mut self, byte_fee: u64) -> Self {
			self.byte_fee = byte_fee;
			self
		}
		pub fn weight_fee(mut self, weight_to_fee: u64) -> Self {
			self.weight_to_fee = weight_to_fee;
			self
		}
		pub fn balance_factor(mut self, factor: u64) -> Self {
			self.balance_factor = factor;
			self
		}
		fn set_constants(&self) {
			EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow_mut() = self.base_weight);
			TRANSACTION_BYTE_FEE.with(|v| *v.borrow_mut() = self.byte_fee);
			WEIGHT_TO_FEE.with(|v| *v.borrow_mut() = self.weight_to_fee);
		}
		pub fn build(self) -> sp_io::TestExternalities {
			self.set_constants();
			let mut t = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
			pallet_balances::GenesisConfig::<Runtime> {
				balances: if self.balance_factor > 0 {
					vec![
						(1, 10 * self.balance_factor),
						(2, 20 * self.balance_factor),
						(3, 30 * self.balance_factor),
						(4, 40 * self.balance_factor),
						(5, 50 * self.balance_factor),
						(6, 60 * self.balance_factor),
					]
				} else {
					vec![]
				},
			}
			.assimilate_storage(&mut t)
			.unwrap();
			t.into()
		}
	}

	/// create a transaction info struct from weight. Handy to avoid building the whole struct.
	pub fn info_from_weight(w: Weight) -> DispatchInfo {
		// pays_fee: Pays::Yes -- class: DispatchClass::Normal
		DispatchInfo { weight: w, ..Default::default() }
	}

	fn post_info_from_weight(w: Weight) -> PostDispatchInfo {
		PostDispatchInfo { actual_weight: Some(w), pays_fee: Default::default() }
	}

	fn post_info_from_pays(p: Pays) -> PostDispatchInfo {
		PostDispatchInfo { actual_weight: None, pays_fee: p }
	}

	fn default_post_info() -> PostDispatchInfo {
		PostDispatchInfo { actual_weight: None, pays_fee: Default::default() }
	}

	#[test]
	fn signed_extension_transaction_payment_work() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(5)
			.build()
			.execute_with(|| {
				let len = 10;
				let pre = ChargeTransactionFee::<Runtime>::from(0)
					.pre_dispatch(&1, CALL, &info_from_weight(5), len)
					.unwrap();
				assert_eq!(Balances::free_balance(1), 100 - 5 - 5 - 10);

				assert_ok!(ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&info_from_weight(5),
					&default_post_info(),
					len,
					&Ok(())
				));
				assert_eq!(Balances::free_balance(1), 100 - 5 - 5 - 10);
				assert_eq!(FEE_UNBALANCED_AMOUNT.with(|a| a.borrow().clone()), 5 + 5 + 10);
				assert_eq!(TIP_UNBALANCED_AMOUNT.with(|a| a.borrow().clone()), 0);

				FEE_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() = 0);

				let pre = ChargeTransactionFee::<Runtime>::from(5 /* tipped */)
					.pre_dispatch(&2, CALL, &info_from_weight(100), len)
					.unwrap();
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 100 - 5);

				assert_ok!(ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&info_from_weight(100),
					&post_info_from_weight(50),
					len,
					&Ok(())
				));
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 50 - 5);
				assert_eq!(FEE_UNBALANCED_AMOUNT.with(|a| a.borrow().clone()), 5 + 10 + 50);
				assert_eq!(TIP_UNBALANCED_AMOUNT.with(|a| a.borrow().clone()), 5);
			});
	}

	#[test]
	fn signed_extension_transaction_payment_multiplied_refund_works() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(5)
			.build()
			.execute_with(|| {
				let len = 10;
				<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));

				let pre = ChargeTransactionFee::<Runtime>::from(5 /* tipped */)
					.pre_dispatch(&2, CALL, &info_from_weight(100), len)
					.unwrap();
				// 5 base fee, 10 byte fee, 3/2 * 100 weight fee, 5 tip
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 150 - 5);

				assert_ok!(ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&info_from_weight(100),
					&post_info_from_weight(50),
					len,
					&Ok(())
				));
				// 75 (3/2 of the returned 50 units of weight) is refunded
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 75 - 5);
			});
	}

	#[test]
	fn signed_extension_transaction_payment_is_bounded() {
		ExtBuilder::default().balance_factor(1000).byte_fee(0).build().execute_with(|| {
			// maximum weight possible
			assert_ok!(ChargeTransactionPayment::<Runtime>::from(0).pre_dispatch(
				&1,
				CALL,
				&info_from_weight(Weight::max_value()),
				10
			));
			// fee will be proportional to what is the actual maximum weight in the runtime.
			assert_eq!(
				Balances::free_balance(&1),
				(10000 - <Runtime as system::Config>::BlockWeights::get().max_block) as u64
			);
		});
	}

	#[test]
	fn signed_extension_allows_free_transactions() {
		ExtBuilder::default()
			.base_weight(100)
			.balance_factor(0)
			.build()
			.execute_with(|| {
				// 1 ain't have a penny.
				assert_eq!(Balances::free_balance(1), 0);

				let len = 100;

				// This is a completely free (and thus wholly insecure/DoS-ridden) transaction.
				let operational_transaction = DispatchInfo {
					weight: 0,
					class: DispatchClass::Operational,
					pays_fee: Pays::No,
				};
				assert_ok!(ChargeTransactionPayment::<Runtime>::from(0).validate(
					&1,
					CALL,
					&operational_transaction,
					len
				));

				// like a InsecureFreeNormal
				let free_transaction =
					DispatchInfo { weight: 0, class: DispatchClass::Normal, pays_fee: Pays::Yes };
				assert_noop!(
					ChargeTransactionPayment::<Runtime>::from(0).validate(
						&1,
						CALL,
						&free_transaction,
						len
					),
					TransactionValidityError::Invalid(InvalidTransaction::Payment),
				);
			});
	}

	#[test]
	fn signed_ext_length_fee_is_also_updated_per_congestion() {
		ExtBuilder::default()
			.base_weight(5)
			.balance_factor(10)
			.build()
			.execute_with(|| {
				// all fees should be x1.5
				<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));
				let len = 10;

				assert_ok!(ChargeTransactionPayment::<Runtime>::from(10) // tipped
					.pre_dispatch(&1, CALL, &info_from_weight(3), len));
				assert_eq!(
					Balances::free_balance(1),
					100 // original
							- 10 // tip
							- 5 // base
							- 10 // len
							- (3 * 3 / 2) // adjusted weight
				);
			})
	}

	#[test]
	fn query_info_works() {
		let call = Call::Balances(BalancesCall::transfer(2, 69));
		let origin = 111111;
		let extra = ();
		let xt = TestXt::new(call, Some((origin, extra)));
		let info = xt.get_dispatch_info();
		let ext = xt.encode();
		let len = ext.len() as u32;
		ExtBuilder::default().base_weight(5).weight_fee(2).build().execute_with(|| {
			// all fees should be x1.5
			<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));

			assert_eq!(
				TransactionPayment::query_info(xt, len),
				RuntimeDispatchInfo {
					weight: info.weight,
					class: info.class,
					partial_fee: 5 * 2 /* base * weight_fee */
								+ len as u64  /* len * 1 */
								+ info.weight.min(BlockWeights::get().max_block) as u64 * 2 * 3 / 2 /* weight */
				},
			);
		});
	}

	#[test]
	fn compute_fee_works_without_multiplier() {
		ExtBuilder::default()
			.base_weight(100)
			.byte_fee(10)
			.balance_factor(0)
			.build()
			.execute_with(|| {
				// Next fee multiplier is zero
				assert_eq!(<NextFeeMultiplier<Runtime>>::get(), Multiplier::one());

				// Tip only, no fees works
				let dispatch_info = DispatchInfo {
					weight: 0,
					class: DispatchClass::Operational,
					pays_fee: Pays::No,
				};
				assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 10), 10);
				// No tip, only base fee works
				let dispatch_info = DispatchInfo {
					weight: 0,
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 100);
				// Tip + base fee works
				assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 69), 169);
				// Len (byte fee) + base fee works
				assert_eq!(Pallet::<Runtime>::compute_fee(42, &dispatch_info, 0), 520);
				// Weight fee + base fee works
				let dispatch_info = DispatchInfo {
					weight: 1000,
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 1100);
			});
	}

	#[test]
	fn compute_fee_works_with_multiplier() {
		ExtBuilder::default()
			.base_weight(100)
			.byte_fee(10)
			.balance_factor(0)
			.build()
			.execute_with(|| {
				// Add a next fee multiplier. Fees will be x3/2.
				<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));
				// Base fee is unaffected by multiplier
				let dispatch_info = DispatchInfo {
					weight: 0,
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 100);

				// Everything works together :)
				let dispatch_info = DispatchInfo {
					weight: 123,
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				// 123 weight, 456 length, 100 base
				assert_eq!(
					Pallet::<Runtime>::compute_fee(456, &dispatch_info, 789),
					100 + (3 * 123 / 2) + 4560 + 789,
				);
			});
	}

	#[test]
	fn compute_fee_works_with_negative_multiplier() {
		ExtBuilder::default()
			.base_weight(100)
			.byte_fee(10)
			.balance_factor(0)
			.build()
			.execute_with(|| {
				// Add a next fee multiplier. All fees will be x1/2.
				<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(1, 2));

				// Base fee is unaffected by multiplier.
				let dispatch_info = DispatchInfo {
					weight: 0,
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 100);

				// Everything works together.
				let dispatch_info = DispatchInfo {
					weight: 123,
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				// 123 weight, 456 length, 100 base
				assert_eq!(
					Pallet::<Runtime>::compute_fee(456, &dispatch_info, 789),
					100 + (123 / 2) + 4560 + 789,
				);
			});
	}

	#[test]
	fn compute_fee_does_not_overflow() {
		ExtBuilder::default()
			.base_weight(100)
			.byte_fee(10)
			.balance_factor(0)
			.build()
			.execute_with(|| {
				// Overflow is handled
				let dispatch_info = DispatchInfo {
					weight: Weight::max_value(),
					class: DispatchClass::Operational,
					pays_fee: Pays::Yes,
				};
				assert_eq!(
					Pallet::<Runtime>::compute_fee(u32::MAX, &dispatch_info, u64::MAX),
					u64::MAX
				);
			});
	}

	#[test]
	fn refund_does_not_recreate_account() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(5)
			.build()
			.execute_with(|| {
				// So events are emitted
				System::set_block_number(10);
				let len = 10;
				let pre = ChargeTransactionFee::<Runtime>::from(5 /* tipped */)
					.pre_dispatch(&2, CALL, &info_from_weight(100), len)
					.unwrap();
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 100 - 5);

				// kill the account between pre and post dispatch
				assert_ok!(Balances::transfer(Some(2).into(), 3, Balances::free_balance(2)));
				assert_eq!(Balances::free_balance(2), 0);

				assert_ok!(ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&info_from_weight(100),
					&post_info_from_weight(50),
					len,
					&Ok(())
				));
				assert_eq!(Balances::free_balance(2), 0);
				// Transfer Event
				System::assert_has_event(Event::Balances(pallet_balances::Event::Transfer(
					2, 3, 80,
				)));
				// Killed Event
				System::assert_has_event(Event::System(system::Event::KilledAccount(2)));
			});
	}

	#[test]
	fn actual_weight_higher_than_max_refunds_nothing() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(5)
			.build()
			.execute_with(|| {
				let len = 10;
				let pre = ChargeTransactionFee::<Runtime>::from(5 /* tipped */)
					.pre_dispatch(&2, CALL, &info_from_weight(100), len)
					.unwrap();
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 100 - 5);

				assert_ok!(ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&info_from_weight(100),
					&post_info_from_weight(101),
					len,
					&Ok(())
				));
				assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 100 - 5);
			});
	}

	#[test]
	fn zero_transfer_on_free_transaction() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(5)
			.build()
			.execute_with(|| {
				// So events are emitted
				System::set_block_number(10);
				let len = 10;
				let dispatch_info =
					DispatchInfo { weight: 100, pays_fee: Pays::No, class: DispatchClass::Normal };
				let user = 69;
				let pre = ChargeTransactionFee::<Runtime>::from(0)
					.pre_dispatch(&user, CALL, &dispatch_info, len)
					.unwrap();
				assert_eq!(Balances::total_balance(&user), 0);
				assert_ok!(ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&dispatch_info,
					&default_post_info(),
					len,
					&Ok(())
				));
				assert_eq!(Balances::total_balance(&user), 0);
				// No events for such a scenario
				assert_eq!(System::events().len(), 0);
			});
	}

	#[test]
	fn refund_consistent_with_actual_weight() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(7)
			.build()
			.execute_with(|| {
				let info = info_from_weight(100);
				let post_info = post_info_from_weight(33);
				let prev_balance = Balances::free_balance(2);
				let len = 10;
				let tip = 5;

				<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(5, 4));

				let pre = ChargeTransactionFee::<Runtime>::from(tip)
					.pre_dispatch(&2, CALL, &info, len)
					.unwrap();

				ChargeTransactionFee::<Runtime>::post_dispatch(
					pre,
					&info,
					&post_info,
					len,
					&Ok(()),
				)
				.unwrap();

				let refund_based_fee = prev_balance - Balances::free_balance(2);
				let actual_fee =
					Pallet::<Runtime>::compute_actual_fee(len as u32, &info, &post_info, tip);

				// 33 weight, 10 length, 7 base, 5 tip
				assert_eq!(actual_fee, 7 + 10 + (33 * 5 / 4) + 5);
				assert_eq!(refund_based_fee, actual_fee);
			});
	}

	#[test]
	fn post_info_can_change_pays_fee() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(7)
			.build()
			.execute_with(|| {
				let info = info_from_weight(100);
				let post_info = post_info_from_pays(Pays::No);
				let prev_balance = Balances::free_balance(2);
				let len = 10;
				let tip = 5;

				<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(5, 4));

				let pre = ChargeTransactionFee::<Runtime>::from(tip)
					.pre_dispatch(&2, CALL, &info, len)
					.unwrap();

				ChargeTransactionFee::<Runtime>::post_dispatch(
					pre,
					&info,
					&post_info,
					len,
					&Ok(()),
				)
				.unwrap();

				let refund_based_fee = prev_balance - Balances::free_balance(2);
				let actual_fee =
					Pallet::<Runtime>::compute_actual_fee(len as u32, &info, &post_info, tip);

				// Only 5 tip is paid
				assert_eq!(actual_fee, 5);
				assert_eq!(refund_based_fee, actual_fee);
			});
	}
}
