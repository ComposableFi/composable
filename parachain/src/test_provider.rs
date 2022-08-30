use crate::ParachainClient;
use futures::{Stream, StreamExt};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	events::IbcEvent,
};
use primitives::{KeyProvider, TestProvider};
use sp_runtime::{
	traits::{Header as HeaderT, IdentifyAccount, Verify},
	MultiSignature, MultiSigner,
};
use std::{fmt::Display, pin::Pin};
use subxt::Config;
use tokio_stream::wrappers::BroadcastStream;

#[async_trait::async_trait]
impl<T> TestProvider for ParachainClient<T>
where
	T: Config + Send + Sync + Clone,
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as Config>::Address: From<<T as Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero,
{
	async fn send_transfer(&self, transfer: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		use pallet_ibc::{MultiAddress, TransferParams};

		let params = TransferParams {
			to: MultiAddress::Raw(transfer.receiver.as_ref().as_bytes().to_vec()),
			source_channel: transfer.source_channel.sequence(),
			timeout_timestamp_offset: 0,
			timeout_height_offset: 0,
		};
		let amount = str::parse::<u128>(&transfer.token.amount.to_string()).expect("Infallible!");
		self.transfer_tokens(params, 1, amount).await?;

		Ok(())
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>> {
		let stream =
			BroadcastStream::new(self.sender.subscribe()).map(|result| result.unwrap_or_default());
		Box::pin(Box::new(stream))
	}
}
