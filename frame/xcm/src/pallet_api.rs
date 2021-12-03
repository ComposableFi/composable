use crate::TransferParams;

pub trait PalletApi {
	fn transfer(params: TransferParams);
}
