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
// TODO: allow until pallet fully implemented
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{
		auction::DutchAuction,
		dex::{Orderbook, SimpleExchange},
		lending::Lending,
		liquidation::Liquidation,
		math::LiftedFixedBalance,
	};
	use frame_support::{
		dispatch::DispatchResult,
		log,
		pallet_prelude::MaybeSerializeDeserialize,
		traits::{Hooks, IsType, UnixTime},
		transactional, PalletId, Parameter,
	};
	use frame_system::{offchain::Signer, pallet_prelude::*, Account};
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, DispatchError, FixedPointNumber, FixedPointOperand, FixedU128, Percent,
		Perquintill,
	};
	use sp_std::{fmt::Debug, vec::Vec};
	pub trait DeFiComposablePallet {
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Default;
	}

	pub const PALLET_ID: PalletId = PalletId(*b"Liqudati");

	#[pallet::config]
	pub trait Config: frame_system::Config + DeFiComposablePallet {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ FixedPointOperand
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
		type UnixTime: UnixTime;

		type Lending: Lending;

		type DutchAuction: DutchAuction<
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			AssetId = Self::AssetId,
			OrderId = u128,
		>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PositionWasSentToLiquidation {},
	}
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1234)]
		pub fn liquidate_many(
			sender: OriginFor<T>,
			block_number: T::BlockNumber,
		) -> DispatchResult {
			// how in PF
			// ask ask all illiquid borrow
			// collect collaterals and borrows
			// make sure that can transfer these to dutch auction (API in lending)
			for (
				source_account,
				&source_asset_id,
				&source_asset_price,
				&target_asset_id,
				target_account,
				&total_amount,
			) in Vec::new().iter()
			{
				let _liquidation_id = Self::liquidate(
					source_account,
					source_asset_id,
					source_asset_price,
					target_asset_id,
					target_account,
					total_amount,
				)?;
			}
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			// for off chain worker need  to implement transaction signer to call into extrinsic
			// let signer = Signer::<T, T::AccountId>::any_account();
			// if !signer.can_sign() {
			// 	return Err(Error::<T>::NoAvailableAccount);
			// }
			// if let Err(e) = Self::liquidate_many(signer, block_number) {
			//     log::error!("Failed to run offchain liquidation: {:?}", e);
			// }
		}
	}

	impl<T: Config> Liquidation for Pallet<T> {
		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;

		type LiquidationId = u128;

		fn liquidate(
			source_account: &Self::AccountId,
			source_asset_id: Self::AssetId,
			source_asset_price: Self::Balance,
			target_asset_id: Self::AssetId,
			target_account: &Self::AccountId,
			total_amount: Self::Balance,
		) -> Result<Self::LiquidationId, DispatchError> {
			let order_id = <T as Config>::DutchAuction::start(
				source_account,
				source_asset_id,
				source_account,
				target_asset_id,
				target_account,
				total_amount,
				source_asset_price,
				composable_traits::auction::AuctionStepFunction::default(),
			)?;
			Ok(order_id)
		}

		fn has_been_liquidated(liquidation_id: &Self::LiquidationId) -> bool {
			// TODO: implement
			false
		}
	}
}
