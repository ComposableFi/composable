use crate::{math::*, prelude::*, support::DefiMultiReservableCurrency, types::*};
pub use crate::{pallet::*, weights::WeightInfo};
use composable_support::abstractions::utils::increment::Increment;
use composable_traits::{
	defi::{DeFiComposableConfig, Sell, SellEngine, Take},
	time::TimeReleaseFunction,
	xcm::XcmSellInitialResponseTransact,
};
use frame_support::{
	traits::{tokens::fungible::Transfer as NativeTransfer, UnixTime},
	transactional,
};
use orml_traits::MultiReservableCurrency;
use sp_runtime::{traits::AccountIdConversion, DispatchError};
use sp_std::convert::TryInto;
use xcm::latest::{prelude::*, MultiAsset, WeightLimit::Unlimited};

impl<T: Config> Pallet<T> {
	#[transactional]
	pub fn take_order(
		order_id: <T as Config>::OrderId,
		mut takes: Vec<TakeOf<T>>,
	) -> Result<(), DispatchError> {
		<SellOrders<T>>::try_mutate_exists(order_id, |order_item| {
			if let Some(crate::types::SellOrder {
				order,
				context: _,
				from_to: ref seller,
				configuration: _,
				total_amount_received,
			}) = order_item
			{
				let mut amount_received = T::Balance::zero();
				// users payed N * WEIGHT before, we here pay N * (log N - 1) * Weight. We can
				// retain pure N by first served principle so, not highest price.
				takes.sort_by(|a, b| b.take.limit.cmp(&a.take.limit));
				// calculate real price
				for take in takes {
					let quote_amount = take.take.quote_limit_amount()?;
					// TODO: what to do with orders which nobody ever takes? some kind of dust
					// orders
					if order.take.amount == T::Balance::zero() {
						// bidder was unlucky because order was sol out
						T::MultiCurrency::unreserve(order.pair.quote, &take.from_to, quote_amount);
					} else {
						let take_amount = take.take.amount.min(order.take.amount);
						order.take.amount -= take_amount;
						let real_quote_amount = take.take.quote_amount(take_amount)?;

						T::MultiCurrency::exchange_reserved(
							order.pair.base,
							seller,
							take_amount,
							order.pair.quote,
							&take.from_to,
							real_quote_amount,
						)?;
						if real_quote_amount < quote_amount {
							T::MultiCurrency::unreserve(
								order.pair.quote,
								&take.from_to,
								quote_amount - real_quote_amount,
							);
						}
						amount_received += real_quote_amount;
					}
				}

				*total_amount_received += amount_received;

				if order.take.amount == T::Balance::zero() {
					Self::callback_xcm(order, seller, order_id, *total_amount_received)?;
					*order_item = None;
					Self::deposit_event(Event::OrderRemoved { order_id });
				}

				if amount_received > T::Balance::zero() {
					return Ok(())
				}
			}
			Err(Error::<T>::TakeOrderDidNotHappen.into())
		})
	}

	pub fn callback_xcm(
		order: &Sell<
			<T as DeFiComposableConfig>::MayBeAssetId,
			<T as DeFiComposableConfig>::Balance,
		>,
		seller: &<T as frame_system::Config>::AccountId,
		order_id: <T as Config>::OrderId,
		received_amount: <T as DeFiComposableConfig>::Balance,
	) -> Result<(), DispatchError> {
		LocalOrderIdToRemote::<T>::try_mutate_exists(order_id, |xcm_order_item| {
			if let Some((parachain_id, xcm_order_id)) = xcm_order_item {
				let parachain_id = *parachain_id;
				let xcm_order_id = *xcm_order_id;
				let mut account = vec![0_u8; 32];
				seller.encode_to(&mut account);
				let account: [u8; 32] =
					account.try_into().expect("cumulus runtime has no account with 33 bytes");
				// as of now we do only final sell
				// setting up XCM message
				let asset_id = MultiLocation {
					parents: 1,
					interior: X2(
						AccountId32 { network: Any, id: account },
						GeneralKey(
							frame_support::storage::weak_bounded_vec::WeakBoundedVec::force_from(
								order.pair.encode(),
								None,
							),
						),
					),
				};
				let asset_id = AssetId::Concrete(asset_id);
				let assets = MultiAsset { fun: Fungible(received_amount.into()), id: asset_id };
				let callback =
					composable_traits::xcm::SellResponse::Final(XcmSellInitialResponseTransact {
						total_amount_taken: received_amount.into(),
						minimal_price: composable_traits::xcm::Balance::one(), /* auction goes to
						                                                        * minimal price,
						                                                        * can
						                                                        * thin about
						                                                        * better
						                                                        * later */
						order_id: xcm_order_id,
					});
				if let Some(method) = ParachainXcmCallbackLocation::<T>::get(parachain_id) {
					let callback = composable_traits::xcm::XcmCumulusDispatch::new(
						method.pallet_instance,
						method.method_id,
						callback,
					);
					let callback = vec![
						WithdrawAsset(assets.clone().into()),
						BuyExecution { fees: assets.clone(), weight_limit: Unlimited },
						TransferReserveAsset {
							assets: assets.into(),
							dest: (
								Parent,
								X3(
									Parachain(parachain_id.into()),
									AccountId32 { network: Any, id: account },
						      GeneralKey(frame_support::storage::weak_bounded_vec::WeakBoundedVec::force_from(order.pair.encode(), None)),
								),
							)
								.into(),
							xcm: Xcm(vec![Transact {
								origin_type: OriginKind::Native,
								require_weight_at_most: 0, /* TODO: make sure that
								                            * callbacks are free (if
								                            * correct) or specify
								                            * price */
								call: callback.encode().into(),
							}]),
						},
					];
					let msg = Xcm(callback);
					let result = T::XcmSender::send_xcm(
						(Parent, Junction::Parachain(parachain_id.into())),
						msg,
					);
					match result {
						Ok(_) => {
							// TODO: decide if need to send event about sent XCM
							*xcm_order_item = None;
						},
						Err(_) => {
							// TODO: insert here event to allow to act on failure
							return Err(Error::<T>::TakeOrderDidNotHappen.into())
						},
					}
				}
			}
			Ok(())
		})
	}
}

impl<T: Config + DeFiComposableConfig> SellEngine<TimeReleaseFunction> for Pallet<T> {
	type OrderId = T::OrderId;
	fn ask(
		from_to: &Self::AccountId,
		order: Sell<Self::MayBeAssetId, Self::Balance>,
		configuration: TimeReleaseFunction,
	) -> Result<Self::OrderId, DispatchError> {
		ensure!(order.is_valid(), Error::<T>::OrderParametersIsInvalid,);
		let order_id = <OrdersIndex<T>>::increment();
		let treasury = &T::PalletId::get().into_account_truncating();
		let deposit = T::PositionExistentialDeposit::get();
		<T::NativeCurrency as NativeTransfer<T::AccountId>>::transfer(
			from_to, treasury, deposit, true,
		)?;

		let now = T::UnixTime::now().as_secs();
		let order = SellOf::<T> {
			from_to: from_to.clone(),
			configuration,
			order,
			context: EDContext::<Self::Balance> { added_at: now, deposit },
			total_amount_received: Self::Balance::zero(),
		};

		T::MultiCurrency::reserve(order.order.pair.base, from_to, order.order.take.amount)?;
		SellOrders::<T>::insert(order_id, order);

		Ok(order_id)
	}

	fn take(
		from_to: &Self::AccountId,
		order_id: Self::OrderId,
		take: Take<Self::Balance>,
	) -> Result<(), DispatchError> {
		ensure!(take.is_valid(), Error::<T>::TakeParametersIsInvalid,);
		let order = <SellOrders<T>>::try_get(order_id)
			.map_err(|_x| Error::<T>::RequestedOrderDoesNotExists)?;
		ensure!(order.order.take.limit <= take.limit, Error::<T>::TakeLimitDoesNotSatisfyOrder,);
		let limit = order.order.take.limit;
		// may consider storing calculation results within single block, so that finalize does
		// not recalculates
		let passed = T::UnixTime::now().as_secs() - order.context.added_at;
		let _limit = order.configuration.price(limit, passed)?;
		let quote_amount = take.quote_limit_amount()?;

		T::MultiCurrency::reserve(order.order.pair.quote, from_to, quote_amount)?;
		<Takes<T>>::append(order_id, TakeOf::<T> { from_to: from_to.clone(), take });

		Ok(())
	}
}
