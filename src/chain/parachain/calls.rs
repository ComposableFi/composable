use codec::{Decode, Encode};
use subxt::{Client, Config, SubmittableExtrinsic};

use super::polkadot;
pub use ibc_primitives::OpenChannelParams;
pub use pallet_ibc::Any as RawAny;
pub use ping::SendPingParams;
pub use transfer::{PalletParams, TransferParams};

// Statically define Pallet Ibc Calls
#[derive(Decode)]
pub struct Sudo<'a, C, T: Config> {
	// T should be a call
	pub call: C,
	#[codec(skip)]
	pub client: &'a Client<T>,
}

#[derive(Encode, Decode, Clone)]
pub struct Deliver {
	pub messages: Vec<RawAny>,
}

#[derive(Encode, Decode, Clone)]
pub struct DeliverPermissioned {
	pub messages: Vec<RawAny>,
}

impl<'a, C: codec::Codec + subxt::Call + Clone, T: Config> subxt::Call for Sudo<'a, C, T> {
	const PALLET: &'static str = "Sudo";
	const FUNCTION: &'static str = "sudo";
}

impl<'a, C: codec::Codec + subxt::Call + Clone, T: Config> Encode for Sudo<'a, C, T> {
	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		let locked_metadata = self.client.metadata();
		let metadata = locked_metadata.read();
		let pallet = metadata
			.pallet(C::PALLET)
			.expect("Encoding failed, could not find pallet index");
		let index = pallet.index();
		let call_index =
			pallet.call_index::<C>().expect("Encoding failed, could not find call index");
		(index, call_index, self.call.clone()).using_encoded(f)
	}
}

impl subxt::Call for Deliver {
	const PALLET: &'static str = "Ibc";
	const FUNCTION: &'static str = "deliver";
}

impl subxt::Call for DeliverPermissioned {
	const PALLET: &'static str = "Ibc";
	const FUNCTION: &'static str = "deliver_permissioned";
}

pub fn deliver<T: Config, X: subxt::extrinsic::ExtrinsicParams<T>>(
	client: &Client<T>,
	call: Deliver,
) -> SubmittableExtrinsic<T, X, Deliver, polkadot::api::runtime_types::sp_runtime::DispatchError, ()>
{
	SubmittableExtrinsic::new(client, call)
}

pub fn sudo_call<
	'a,
	T: Config,
	X: subxt::extrinsic::ExtrinsicParams<T>,
	C: Send + Sync + codec::Codec + subxt::Call + Clone,
>(
	client: &'a Client<T>,
	call: Sudo<'a, C, T>,
) -> SubmittableExtrinsic<
	'a,
	T,
	X,
	Sudo<'a, C, T>,
	polkadot::api::runtime_types::sp_runtime::DispatchError,
	(),
> {
	SubmittableExtrinsic::new(client, call)
}

#[derive(Encode, Decode, Clone)]
pub struct Transfer {
	pub params: TransferParams,
	pub asset_id: u128,
	pub amount: u128,
}

impl subxt::Call for Transfer {
	const PALLET: &'static str = "Transfer";
	const FUNCTION: &'static str = "transfer";
}

pub fn ibc_transfer<T: Config, X: subxt::extrinsic::ExtrinsicParams<T>>(
	client: &Client<T>,
	call: Transfer,
) -> SubmittableExtrinsic<T, X, Transfer, polkadot::api::runtime_types::sp_runtime::DispatchError, ()>
{
	SubmittableExtrinsic::new(client, call)
}

#[derive(Encode, Decode, Clone)]
pub struct OpenTransferChannel {
	pub params: OpenChannelParams,
}

impl subxt::Call for OpenTransferChannel {
	const PALLET: &'static str = "Transfer";
	const FUNCTION: &'static str = "open_channel";
}

#[derive(Encode, Decode, Clone)]
pub struct SetPalletParams {
	pub params: PalletParams,
}

impl subxt::Call for SetPalletParams {
	const PALLET: &'static str = "Transfer";
	const FUNCTION: &'static str = "set_pallet_params";
}

#[derive(Encode, Decode, Clone)]
pub struct OpenPingChannel {
	pub params: OpenChannelParams,
}

impl subxt::Call for OpenPingChannel {
	const PALLET: &'static str = "IbcPing";
	const FUNCTION: &'static str = "open_channel";
}

#[derive(Encode, Decode, Clone)]
pub struct SendPing {
	pub params: SendPingParams,
}

impl subxt::Call for SendPing {
	const PALLET: &'static str = "IbcPing";
	const FUNCTION: &'static str = "send_ping";
}
