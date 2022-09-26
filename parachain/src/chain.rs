use codec::Decode;
use std::{collections::BTreeMap, fmt::Display, pin::Pin};

use futures::{Stream, StreamExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc_proto::google::protobuf::Any;
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use subxt::{extrinsic::PlainTip, Config, PolkadotExtrinsicParams, PolkadotExtrinsicParamsBuilder};
use transaction_payment_rpc::TransactionPaymentApiClient;
use transaction_payment_runtime_api::RuntimeDispatchInfo;

use primitives::{Chain, IbcProvider};

use super::{
	calls::{deliver, Deliver, RawAny},
	error::Error,
	signer::ExtrinsicSigner,
	ParachainClient,
};

#[async_trait::async_trait]
impl<T: Config + Send + Sync> Chain for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
{
	fn name(&self) -> &str {
		&*self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_extrinsic_weight
	}

	async fn estimate_weight(&self, messages: Vec<Any>) -> Result<u64, Self::Error> {
		let extrinsic = {
			// todo: put this in utils
			let signer = ExtrinsicSigner::<T, Self>::new(
				self.key_store.clone(),
				self.key_type_id.clone(),
				self.public_key.clone(),
			);

			let messages = messages
				.into_iter()
				.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
				.collect::<Vec<_>>();

			let metadata = self.para_client.rpc().metadata().await?;
			// Check for pallet and call index existence in latest chain metadata to ensure our
			// static definitions are up to date
			let pallet = metadata
				.pallet(<Deliver as subxt::Call>::PALLET)
				.map_err(|_| Error::PalletNotFound(<Deliver as subxt::Call>::PALLET))?;
			pallet
				.call_index::<Deliver>()
				.map_err(|_| Error::CallNotFound(<Deliver as subxt::Call>::FUNCTION))?;
			// Update the metadata held by the client
			let _ = self.para_client.metadata().try_write().and_then(|mut writer| {
				*writer = metadata;
				Some(writer)
			});

			let tx_params = PolkadotExtrinsicParamsBuilder::new()
				.tip(PlainTip::new(100_000))
				.era(Era::Immortal, *self.para_client.genesis());

			let submitabble_ext =
				deliver::<T, PolkadotExtrinsicParams<T>>(&self.para_client, Deliver { messages })
					.create_signed(&signer, tx_params)
					.await?;
			submitabble_ext.encoded().to_vec()
		};
		let dispatch_info =
			TransactionPaymentApiClient::<sp_core::H256, RuntimeDispatchInfo<u128>>::query_info(
				&*self.para_client.rpc().client,
				extrinsic.into(),
				None,
			)
			.await?;
		Ok(dispatch_info.weight)
	}

	#[cfg(not(feature = "beefy"))]
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		use finality_grandpa_rpc::GrandpaApiClient;
		/// An encoded justification proving that the given header has been finalized
		#[derive(Clone, serde::Serialize, serde::Deserialize)]
		struct JustificationNotification(sp_core::Bytes);
		let subscription =
			GrandpaApiClient::<JustificationNotification, sp_core::H256, u32>::subscribe_justifications(
				&*self.relay_client.rpc().client,
			)
				.await
				.expect("Failed to subscribe to grandpa justifications");

		let stream = subscription.filter_map(|justification_notif| {
			let encoded_justification = match justification_notif {
				Ok(JustificationNotification(sp_core::Bytes(justification))) => justification,
				Err(err) => {
					log::error!("Failed to fetch Justification: {}", err);
					return futures::future::ready(None)
				},
			};

			let justification = match Decode::decode(&mut &*encoded_justification) {
				Ok(j) => j,
				Err(err) => {
					log::error!("Grandpa Justification scale decode error: {}", err);
					return futures::future::ready(None)
				},
			};
			futures::future::ready(Some(justification))
		});

		Box::pin(Box::new(stream))
	}

	#[cfg(feature = "beefy")]
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		use subxt::rpc::{rpc_params, SubscriptionClientT};
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

		let stream = subscription.filter_map(|commitment| {
			let commitment = match commitment {
				Ok(c) => c,
				Err(err) => {
					log::error!("Failed to fetch SignedCommitment: {}", err);
					return futures::future::ready(None)
				},
			};
			let recv_commitment = match hex::decode(&commitment[2..]) {
				Ok(c) => c,
				Err(err) => {
					log::error!("SignedCommitment hex decode error: {}", err);
					return futures::future::ready(None)
				},
			};
			let signed_commitment = match Decode::decode(&mut &*recv_commitment) {
				Ok(c) => c,
				Err(err) => {
					log::error!("SignedCommitment scale decode error: {}", err);
					return futures::future::ready(None)
				},
			};
			futures::future::ready(Some(signed_commitment))
		});

		Box::pin(Box::new(stream))
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<(), Error> {
		let messages = messages
			.into_iter()
			.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
			.collect::<Vec<_>>();

		self.submit_call(Deliver { messages }, false).await?;

		Ok(())
	}
}
