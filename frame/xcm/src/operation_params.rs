pub struct TransferParams {}

pub trait IntoXcm<D, F> {
	fn into_xcm(data: D, from: F) -> Self;
}
