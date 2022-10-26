use anyhow::anyhow;
use codec::{Decode, Encode};
use std::{
	collections::BTreeMap,
	fmt::Display,
	pin::Pin,
	time::{Duration, Instant},
};

use beefy_gadget_rpc::BeefyApiClient;
use futures::{Stream, StreamExt, TryFutureExt};
use grandpa::BlockNumberOps;
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc_proto::google::protobuf::Any;
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use subxt::{
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams, SubstrateExtrinsicParamsBuilder},
	Config,
};
use transaction_payment_rpc::TransactionPaymentApiClient;
use transaction_payment_runtime_api::RuntimeDispatchInfo;

use primitives::{Chain, IbcProvider, MisbehaviourHandler};

use super::{error::Error, signer::ExtrinsicSigner, ParachainClient};
use crate::{
	parachain::{api, api::runtime_types::pallet_ibc::Any as RawAny, UncheckedExtrinsic},
	FinalityProtocol, H256,
};
use finality_grandpa_rpc::GrandpaApiClient;
use ibc::{
	core::{
		ics02_client::msgs::{update_client::MsgUpdateAnyClient, ClientMsg},
		ics26_routing::msgs::Ics26Envelope,
	},
	tx_msg::Msg,
};
use ics10_grandpa::client_message::{ClientMessage, Misbehaviour, RelayChainHeader};
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::mock::LocalClientTypes;

use tokio::time::sleep;

type GrandpaJustification = grandpa_light_client_primitives::justification::GrandpaJustification<
	polkadot_core_primitives::Header,
>;

type BeefyJustification =
	beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct JustificationNotification(sp_core::Bytes);

#[async_trait::async_trait]
impl<T: Config + Send + Sync> MisbehaviourHandler for ParachainClient<T>
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
	sp_core::H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
{
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		match client_message {
			AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
				// todo: verify block num. This block number may not exist on relaychain
				// (see `verify_parachain_headers_with_grandpa_finality_proof` implementation)
				let base_block_number =
					header.finality_proof.unknown_headers[0].number.saturating_sub(1);
				let encoded =
					GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
						&*self.relay_ws_client,
						base_block_number,
					)
					.await?
					.ok_or_else(|| {
						anyhow!(
							"No justification found for block: {:?}",
							header.finality_proof.block
						)
					})?
					.0;

				let trusted_finality_proof =
					FinalityProof::<RelayChainHeader>::decode(&mut &encoded[..])?;

				if header.finality_proof.block != trusted_finality_proof.block {
					log::warn!(
						"Found misbehaviour on client {}: {:?} != {:?}",
						self.client_id
							.as_ref()
							.map(|x| x.as_str().to_owned())
							.unwrap_or_else(|| "{unknown}".to_owned()),
						header.finality_proof.block,
						trusted_finality_proof.block
					);

					let misbehaviour = ClientMessage::Misbehaviour(Misbehaviour {
						set_id: 0,
						first_finality_proof: header.finality_proof,
						second_finality_proof: trusted_finality_proof,
					});

					counterparty
						.submit(vec![MsgUpdateAnyClient::<LocalClientTypes>::new(
							self.client_id(),
							AnyClientMessage::Grandpa(misbehaviour.clone()),
							counterparty.account_id(),
						)
						.to_any()])
						.map_err(|e| anyhow!("Failed to submit misbehaviour report: {:?}", e))
						.await?;
				}
			},
			_ => {},
		}
		Ok(())
	}
}

#[async_trait::async_trait]
impl<T: Config + Send + Sync> Chain for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	sp_core::H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
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

			let tx_params = SubstrateExtrinsicParamsBuilder::new()
				.tip(AssetTip::new(100_000))
				.era(Era::Immortal, self.para_client.genesis_hash());
			let call = api::tx().ibc().deliver(messages);
			self.para_client.tx().create_signed(&call, &signer, tx_params.into()).await?
		};
		let dispatch_info =
			TransactionPaymentApiClient::<sp_core::H256, RuntimeDispatchInfo<u128>>::query_info(
				&*self.para_ws_client,
				extrinsic.encoded().to_vec().into(),
				None,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(dispatch_info.weight)
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let subscription =
					GrandpaApiClient::<JustificationNotification, sp_core::H256, u32>::subscribe_justifications(
						&*self.relay_ws_client,
					)
						.await
						.expect("Failed to subscribe to grandpa justifications")
						.chunks(6)
						.map(|mut notifs| notifs.remove(notifs.len() - 1)); // skip every 4 finality notifications

				let stream = subscription.filter_map(|justification_notif| {
					let encoded_justification = match justification_notif {
						Ok(JustificationNotification(sp_core::Bytes(justification))) =>
							justification,
						Err(err) => {
							log::error!("Failed to fetch Justification: {}", err);
							return futures::future::ready(None)
						},
					};

					let justification =
						match GrandpaJustification::decode(&mut &*encoded_justification) {
							Ok(j) => j,
							Err(err) => {
								log::error!("Grandpa Justification scale decode error: {}", err);
								return futures::future::ready(None)
							},
						};
					futures::future::ready(Some(Self::FinalityEvent::Grandpa(justification)))
				});

				Box::pin(Box::new(stream))
			},
			FinalityProtocol::Beefy => {
				let subscription =
					BeefyApiClient::<JustificationNotification, sp_core::H256>::subscribe_justifications(
						&*self.relay_ws_client,
					)
						.await
						.expect("Failed to subscribe to beefy justifications");

				let stream = subscription.filter_map(|commitment_notification| {
					let encoded_commitment = match commitment_notification {
						Ok(JustificationNotification(sp_core::Bytes(commitment))) => commitment,
						Err(err) => {
							log::error!("Failed to fetch Commitment: {}", err);
							return futures::future::ready(None)
						},
					};

					let signed_commitment =
						match BeefyJustification::decode(&mut &*encoded_commitment) {
							Ok(c) => c,
							Err(err) => {
								log::error!("SignedCommitment scale decode error: {}", err);
								return futures::future::ready(None)
							},
						};
					futures::future::ready(Some(Self::FinalityEvent::Beefy(signed_commitment)))
				});

				Box::pin(Box::new(stream))
			},
		}
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<(), Error> {
		let messages = messages
			.into_iter()
			.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
			.collect::<Vec<_>>();

		let call = api::tx().ibc().deliver(messages);
		log::info!("submitted call {:?}", self.submit_call(call).await?);

		Ok(())
	}

	async fn query_client_message(
		&self,
		host_block_hash: [u8; 32],
		transaction_id: u32,
		event_index: usize,
	) -> Result<AnyClientMessage, primitives::error::Error> {
		use api::runtime_types::{
			dali_runtime::Call as RuntimeCall, pallet_ibc::pallet::Call as IbcCall,
		};

		let h256 = H256(host_block_hash);
		log::info!("Querying extrinsic data at {:?} {}", h256, transaction_id);

		let now = Instant::now();
		let block = loop {
			let maybe_block = self.para_client.rpc().block(Some(h256.into())).await?;
			match maybe_block {
				Some(block) => {
					log::info!("block query took {}", now.elapsed().as_millis());
					break block
				},
				None => {
					if now.elapsed() > Duration::from_secs(20) {
						return Err(primitives::error::Error::from(
							"Timeout while waiting for block".to_owned(),
						))
					}
					sleep(Duration::from_millis(100)).await;
				},
			}
		};
		let extrinsic_opaque = block
			.block
			.extrinsics
			.get(transaction_id as usize)
			.expect("Extrinsic not found");

		let unchecked_extrinsic = UncheckedExtrinsic::<T>::decode(&mut &*extrinsic_opaque.encode())
			.map_err(|e| {
				primitives::error::Error::from(format!("Extrinsic decode error: {}", e))
			})?;

		match unchecked_extrinsic.function {
			RuntimeCall::Ibc(IbcCall::deliver { messages }) => {
				let message = messages.get(event_index).ok_or_else(|| {
					primitives::error::Error::from(format!(
						"Message index {} out of bounds",
						event_index
					))
				})?;
				let envelope = Ics26Envelope::<LocalClientTypes>::try_from(Any {
					type_url: String::from_utf8(message.type_url.clone())?,
					value: message.value.clone(),
				});
				match envelope {
					Ok(Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(update_msg))) =>
						return Ok(update_msg.client_message),
					_ => (),
				}
			},
			_ => (),
		}
		Err(primitives::error::Error::Custom("No ICS02 update message found".into()))
	}
}
