use std::pin::Pin;

use futures::Stream;
use ibc_proto::google::protobuf::Any;
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, Verify},
	MultiSigner,
};
use subxt::{
	extrinsic::PlainTip,
	rpc::{rpc_params, SubscriptionClientT},
	Config, PolkadotExtrinsicParams, PolkadotExtrinsicParamsBuilder,
};

use ibc::core::ics03_connection::msgs::{conn_open_ack, conn_open_init};

use primitives::{Chain, IbcProvider, KeyProvider};

use super::{
	calls::{deliver, Deliver, DeliverPermissioned, RawAny},
	error::Error,
	signer::ExtrinsicSigner,
	ParachainClient,
};

#[async_trait::async_trait]
impl<T: Config + Send + Sync> Chain for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	<T::Signature as Verify>::Signer:
		From<Self::Public> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<Self::Public>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<<Self as KeyProvider>::Signature>,
{
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		let subscription = self
			.relay_client
			.rpc()
			.client
			.subscribe::<String>(
				"beefy_subscribeJustifications",
				rpc_params![],
				"beefy_unsubscribeJustifications",
			)
			.await
			.expect("Expect subscription to open");
		Box::pin(Box::new(subscription))
	}

	async fn submit_ibc_messages(&self, mut messages: Vec<Any>) -> Result<(), Error> {
		let public_key = self.public_key();
		let signer =
			ExtrinsicSigner::<T, Self>::new(self.key_store(), self.key_type_id(), public_key);

		let update_client_message = {
			let update_client_message = messages.remove(0);
			RawAny {
				type_url: update_client_message.type_url.as_bytes().to_vec(),
				value: update_client_message.value,
			}
		};
		let mut permissioned_messages = vec![];
		let mut non_permissioned_messages = vec![];

		for msg in messages {
			if matches!(msg.type_url.as_str(), conn_open_init::TYPE_URL | conn_open_ack::TYPE_URL) {
				permissioned_messages.push(msg)
			} else {
				non_permissioned_messages.push(msg)
			}
		}

		let permissioned_messages = permissioned_messages
			.into_iter()
			.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
			.collect::<Vec<_>>();
		let non_permissioned_messages = non_permissioned_messages
			.into_iter()
			.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
			.collect::<Vec<_>>();

		let metadata = self.para_client.rpc().metadata().await?;
		// Check for pallet and call index existence in latest chain metadata to ensure our static
		// definitions are up to date
		let pallet = metadata
			.pallet(<Deliver as subxt::Call>::PALLET)
			.map_err(|_| Error::PalletNotFound(<Deliver as subxt::Call>::PALLET))?;
		pallet
			.call_index::<Deliver>()
			.map_err(|_| Error::CallNotFound(<Deliver as subxt::Call>::FUNCTION))?;
		pallet
			.call_index::<DeliverPermissioned>()
			.map_err(|_| Error::CallNotFound(<Deliver as subxt::Call>::FUNCTION))?;
		// Update the metadata held by the client
		let _ = self.para_client.metadata().try_write().and_then(|mut writer| {
			*writer = metadata;
			Some(writer)
		});

		let tx_params = PolkadotExtrinsicParamsBuilder::new()
			.tip(PlainTip::new(100_000))
			.era(Era::Immortal, *self.para_client.genesis());

		let mut messages = vec![update_client_message];
		if !non_permissioned_messages.is_empty() {
			messages.extend(non_permissioned_messages.into_iter())
		}
		let update_client_ext =
			deliver::<T, PolkadotExtrinsicParams<T>>(&self.para_client, Deliver { messages });
		let _ = update_client_ext.sign_and_submit(&signer, tx_params).await?;

		if !permissioned_messages.is_empty() {
			self.submit_sudo_call(DeliverPermissioned { messages: permissioned_messages })
				.await?;
		}

		Ok(())
	}
}
