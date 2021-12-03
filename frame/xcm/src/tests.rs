#![cfg(test)]

use crate::{
	blockchain::KaruraTransferParams,
	mock::{ComposableXcm, ExtBuilder},
	IntoXcm, PalletApi, TransferParams,
};

#[test]
fn karura_transfer() {
	ExtBuilder::default().build().execute_with(|| {
		let karura_transfer_params = KaruraTransferParams;
		let transfer_params: TransferParams = IntoXcm::into_xcm((), karura_transfer_params);
		ComposableXcm::transfer(transfer_params);
	});
}
