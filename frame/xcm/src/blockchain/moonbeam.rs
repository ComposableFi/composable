use crate::{IntoXcm, TransferParams};

pub struct MoonbeamTransferParams;

impl IntoXcm<(), MoonbeamTransferParams> for TransferParams {
	fn into_xcm(_data: (), _from: MoonbeamTransferParams) -> Self {
		Self {}
	}
}
