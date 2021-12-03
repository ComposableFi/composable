use crate::{IntoXcm, TransferParams};

pub struct KaruraTransferParams;

impl IntoXcm<(), KaruraTransferParams> for TransferParams {
	fn into_xcm(_data: (), _from: KaruraTransferParams) -> Self {
		Self {}
	}
}
