use crate::ParachainClient;
use futures::{Stream, StreamExt};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	events::IbcEvent,
};
use pallet_ibc::{MultiAddress, Timeout, TransferParams};
use primitives::{KeyProvider, TestProvider};
use sp_core_git::crypto::{AccountId32, Ss58Codec};
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
	<T as Config>::Address: From<<T as Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero,
{
	async fn send_transfer(&self, transfer: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		let account_id = AccountId32::from_ss58check(transfer.receiver.as_ref()).unwrap();
		let params = TransferParams {
			to: MultiAddress::Id(account_id),
			source_channel: transfer.source_channel.sequence(),
			timeout: Timeout::Absolute {
				timestamp: Some(transfer.timeout_timestamp.nanoseconds()),
				height: Some(transfer.timeout_height.revision_height),
			},
		};
		let amount = str::parse::<u128>(&transfer.token.amount.to_string()).expect("Infallible!");
		dbg!(&amount);
		self.transfer_tokens(params, 1, amount).await?;

		Ok(())
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>> {
		let stream =
			BroadcastStream::new(self.sender.subscribe()).map(|result| result.unwrap_or_default());
		Box::pin(Box::new(stream))
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		let stream = self.para_client.rpc().subscribe_blocks().await.unwrap().map(|header| {
			let header = header.unwrap();
			let block_number: u64 = (*header.number()).into();
			block_number
		});

		Box::pin(Box::new(stream))
	}
}
