use anyhow::anyhow;
use codec::{Decode, Encode};
use std::{
	collections::BTreeMap,
	fmt::Display,
	future::Future,
	pin::Pin,
	time::{Duration, Instant},
};

use beefy_gadget_rpc::BeefyApiClient;
use finality_grandpa::BlockNumberOps;
use futures::{future, pending, ready, FutureExt, Stream, StreamExt, TryFutureExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc_proto::google::protobuf::Any;
use polkadot::api::runtime_types::{
	pallet_grandpa, rococo_runtime, sp_finality_grandpa as polkadot_grandpa,
};
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use subxt::{
	rpc_params,
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams, SubstrateExtrinsicParamsBuilder},
	Config,
};
use transaction_payment_rpc::TransactionPaymentApiClient;
use transaction_payment_runtime_api::RuntimeDispatchInfo;

use primitives::{Chain, IbcProvider, MisbehaviourHandler};

use super::{error::Error, signer::ExtrinsicSigner, ParachainClient};
use crate::{
	parachain,
	parachain::{api, api::runtime_types::pallet_ibc::Any as RawAny, UncheckedExtrinsic},
	polkadot, FinalityProtocol, H256,
};
use finality_grandpa_rpc::GrandpaApiClient;
use futures::future::{pending, ready};
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
use sp_core::ByteArray;

use sp_finality_grandpa::{Equivocation, OpaqueKeyOwnershipProof};
use subxt::rpc::ChainBlock;
use tokio::time::{sleep, timeout};

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
		use tendermint_proto::Protobuf;
		log::info!("counterparty: {}", counterparty.client_id());
		log::info!("client_msg: {}", hex::encode(client_message.encode_vec()));
		match client_message {
			AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
				let justification = GrandpaJustification::decode(
					&mut header.finality_proof.justification.as_slice(),
				)?;

				let encoded =
					GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
						&*self.relay_ws_client,
						justification.commit.target_number,
					)
					.await?
					.ok_or_else(|| {
						anyhow!(
							"No justification found for block: {:?}",
							header.finality_proof.block
						)
					})?
					.0;
				log::info!("encoded: {}", hex::encode(&encoded.0));

				let trusted_finality_proof =
					FinalityProof::<RelayChainHeader>::decode(&mut &encoded[..])?;

				// dbg!(&header);
				// dbg!(&trusted_finality_proof);
				if header.finality_proof.block != trusted_finality_proof.block {
					log::info!("block mismatch");
					let trusted_justification = GrandpaJustification::decode(
						&mut trusted_finality_proof.justification.as_slice(),
					)?;

					let api = self.relay_client.storage();
					let current_set_id_addr = polkadot::api::storage().grandpa().current_set_id();
					let current_set_id = api
						.fetch(
							&current_set_id_addr,
							Some(T::Hash::from(trusted_finality_proof.block.clone())),
						)
						.await?
						.expect("Failed to fetch current set id");

					log::info!("current_set_id: {}", current_set_id);

					let mut fraud_precommits = Vec::new();
					for first_precommit in &justification.commit.precommits {
						for second_precommit in &trusted_justification.commit.precommits {
							if first_precommit.id == second_precommit.id &&
								first_precommit.precommit != second_precommit.precommit
							{
								log::info!("found misbehaviour");
								fraud_precommits
									.push((first_precommit.clone(), second_precommit.clone()));
							}
						}
					}

					let mut equivocations = Vec::new();
					let mut equivocation_calls = Vec::new();

					for (first, second) in fraud_precommits {
						let key_ownership_proof: OpaqueKeyOwnershipProof = {
							let bytes = self
								.relay_client
								.rpc()
								.request::<String>(
									"state_call",
									rpc_params!(
										"GrandpaApi_generate_key_ownership_proof",
										format!(
											"0x{}",
											hex::encode((&current_set_id, &first.id).encode())
										)
									),
								)
								.await
								.map(|res| hex::decode(&res[2..]))
								.expect("Failed to fetch key ownership proof")?;

							log::info!("data: {}", hex::encode(&bytes));
							Option::decode(&mut &bytes[..])
								.expect("Failed to scale decode key ownership proof")
								.expect("Failed to fetch key ownership proof")
						};

						let equivocation = Equivocation::Precommit(grandpa::Equivocation {
							round_number: justification.round,
							identity: first.id.clone(),
							first: (first.precommit.clone(), first.signature.clone()),
							second: (second.precommit.clone(), second.signature.clone()),
						});

						let polkadot_equivocation =
							construct_polkadot_grandpa_equivocation(&equivocation);
						let equivocation_proof =
							polkadot::api::runtime_types::sp_finality_grandpa::EquivocationProof {
								set_id: current_set_id,
								equivocation: polkadot_equivocation,
							};

						let call = polkadot_runtime_common::Call::Grandpa(
							pallet_grandpa::pallet::Call::report_equivocation {
								equivocation_proof: Box::new(equivocation_proof),
								key_owner_proof: key_ownership_proof.decode().unwrap(),
							},
						);
						equivocation_calls.push(call);
						equivocations.push(equivocation);
					}

					let misbehaviour = ClientMessage::Misbehaviour(Misbehaviour {
						set_id: current_set_id,
						equivocations,
					});

					let batch_call = polkadot::api::tx().utility().batch(equivocation_calls);
					let equivocation_report_future = self
						.submit_call(batch_call, &self.relay_client)
						.map_err(|e| log::error!("Failed to submit equivocation report: {:?}", e))
						.map(|res| {
							log::info!("equivocation report submitted: {:?}", res,);
						});
					let misbehaviour_report_future = counterparty
						.submit(vec![MsgUpdateAnyClient::<LocalClientTypes>::new(
							self.client_id(),
							AnyClientMessage::Grandpa(misbehaviour.clone()),
							counterparty.account_id(),
						)
						.to_any()])
						.map_err(|e| log::error!("Failed to submit misbehaviour report: {:?}", e))
						.map(|res| {
							log::info!("misbehaviour report submitted: {:?}", res,);
						});
					future::join(
						Box::pin(equivocation_report_future)
							as Pin<Box<dyn Future<Output = ()> + Send>>,
						Box::pin(misbehaviour_report_future)
							as Pin<Box<dyn Future<Output = ()> + Send>>,
					)
					.await;
					log::info!("submitted misbehaviour");
				}
			},
			_ => {},
		}
		Ok(())
	}
}

fn construct_polkadot_grandpa_equivocation<H: Copy, N: Copy>(
	equivocation: &Equivocation<H, N>,
) -> polkadot_grandpa::Equivocation<H, N> {
	use polkadot::api::runtime_types::{
		finality_grandpa as polkadot_finality_grandpa,
		finality_grandpa::Precommit,
		sp_core::ed25519::{Public, Signature},
	};

	match equivocation {
		Equivocation::Precommit(equiv) =>
			polkadot_grandpa::Equivocation::Precommit(polkadot_finality_grandpa::Equivocation {
				round_number: equiv.round_number,
				identity: polkadot_grandpa::app::Public(Public(
					equiv.identity.to_raw_vec().try_into().unwrap(),
				)),
				first: (
					Precommit {
						target_number: equiv.first.0.target_number,
						target_hash: equiv.first.0.target_hash,
					},
					polkadot_grandpa::app::Signature(Signature(
						(&*equiv.first.1).try_into().unwrap(),
					)),
				),
				second: (
					Precommit {
						target_number: equiv.second.0.target_number,
						target_hash: equiv.second.0.target_hash,
					},
					polkadot_grandpa::app::Signature(Signature(
						(&*equiv.second.1).try_into().unwrap(),
					)),
				),
			}),
		_ => {
			unimplemented!()
		},
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
		log::info!("submitted call {:?}", self.submit_call(call, &self.para_client).await?);

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

		let extrinsic_data_addr =
			parachain::api::storage().system().extrinsic_data(&transaction_id);
		let h256 = H256(host_block_hash);
		log::info!("Querying extrinsic data at {:?} {}", h256, transaction_id);
		// let block = timeout(Duration::from_secs(20), || async {
		// 	let maybe_block = self.para_client.rpc().block(Some(h256.into())).await?;
		// 	match maybe_block {
		// 		Some(block) =>
		// 			Box::new(ready(block)) as Box<dyn Future<Output = _> + Send + 'static>,
		// 		None => Box::new(pending()) as Box<dyn Future<Output = _> + Send + 'static>,
		// 	}
		// })
		// .await?;

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

		// let extrinsic_opaque = self
		// 	.para_client
		// 	.storage()
		// 	.fetch(&extrinsic_data_addr, Some(h256.into()))
		// 	.await?
		// 	.expect("Extrinsic should exist");
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
