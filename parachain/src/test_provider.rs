use crate::ParachainClient;
use beefy_prover::helpers::unsafe_cast_to_jsonrpsee_client;
use futures::{Stream, StreamExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::ChannelId,
	events::IbcEvent,
};
use jsonrpsee::core::client::SubscriptionClientT;
use pallet_ibc::{MultiAddress, Timeout, TransferParams};
use primitives::{KeyProvider, TestProvider};
use sp_core::crypto::{AccountId32, Ss58Codec};
use sp_runtime::{
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use std::{collections::BTreeMap, fmt::Display, pin::Pin};

use subxt::{
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams},
	Config,
};
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
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256>,
	sp_core::H256: From<T::Hash>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
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

	async fn send_ping(&self, channel_id: ChannelId, timeout: Timeout) -> Result<(), Self::Error> {
		let (timeout_height, timestamp) = match timeout {
			Timeout::Offset { timestamp, height } => (height.unwrap(), timestamp.unwrap()),
			_ => panic!("Only offset timeouts allowed"),
		};

		let params = crate::parachain::api::runtime_types::pallet_ibc_ping::SendPingParams {
			data: "ping".as_bytes().to_vec(),
			timeout_height_offset: timeout_height,
			timeout_timestamp_offset: timestamp,
			channel_id: channel_id.sequence(),
		};

		let call = crate::parachain::api::tx().ibc_ping().send_ping(params);

		self.submit_call(call, true).await.map(|_| ())
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>> {
		let stream =
			BroadcastStream::new(self.sender.subscribe()).map(|result| result.unwrap_or_default());
		Box::pin(Box::new(stream))
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		let para_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.para_client) };
		let stream = para_client
			.subscribe::<T::Header>("chain_subscribeNewHeads", None, "chain_unsubscribeNewHeads")
			.await
			.unwrap()
			.map(|header| {
				let header = header.unwrap();
				let block_number: u64 = (*header.number()).into();
				block_number
			});

		Box::pin(Box::new(stream))
	}
}
