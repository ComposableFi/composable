//! Light client protocols for parachains.

use crate::{error::Error, ParachainClient};
use beefy_light_client_primitives::{ClientState as BeefyPrimitivesClientState, NodesUtils};
use codec::{Decode, Encode};
use grandpa_light_client::justification::find_scheduled_change;
use grandpa_light_client_primitives::{
	FinalityProof, ParachainHeaderProofs, ParachainHeadersWithFinalityProof,
};
use ibc::{
	core::ics02_client::{client_state::ClientState as _, msgs::update_client::MsgUpdateAnyClient},
	events::IbcEvent,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use ibc_rpc::{BlockNumberOrHash, IbcApiClient};
use ics10_grandpa::client_message::{ClientMessage, Header as GrandpaHeader};
use ics11_beefy::client_message::{
	BeefyHeader, ClientMessage as BeefyClientMessage, ParachainHeadersWithProof,
};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState};
use primitives::{
	find_maximum_height_for_timeout_proofs, mock::LocalClientTypes, Chain, IbcProvider,
	KeyProvider, UpdateMessage, UpdateType,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use std::{
	collections::{BTreeMap, BTreeSet, HashMap},
	fmt::Display,
};
use subxt::{
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams},
	Config,
};
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LightClientProtocol {
	Grandpa,
	Beefy,
}

/// Finality event for parachains
#[derive(Decode, Encode)]
pub enum FinalityEvent {
	Grandpa(
		grandpa_light_client::justification::GrandpaJustification<polkadot_core_primitives::Header>,
	),
	Beefy(beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>),
}

impl LightClientProtocol {
	pub async fn query_latest_ibc_events<T, C>(
		&self,
		source: &mut ParachainClient<T>,
		finality_event: FinalityEvent,
		counterparty: &C,
	) -> Result<(UpdateMessage, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		T: Config + Send + Sync,
		C: Chain,
		u32: From<<<T as Config>::Header as HeaderT>::Number>,
		u32: From<<T as Config>::BlockNumber>,
		ParachainClient<T>: Chain,
		ParachainClient<T>: KeyProvider,
		<T::Signature as Verify>::Signer:
			From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
		MultiSigner: From<MultiSigner>,
		<T as Config>::Address: From<<T as Config>::AccountId>,
		T::Signature: From<MultiSignature>,
		T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
		T::Hash: From<sp_core::H256>,
		sp_core::H256: From<T::Hash>,
		FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
			From<FinalityProof<T::Header>>,
		BTreeMap<H256, ParachainHeaderProofs>:
			From<BTreeMap<<T as Config>::Hash, ParachainHeaderProofs>>,
		<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
			From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
	{
		match self {
			LightClientProtocol::Grandpa =>
				query_latest_ibc_events_with_grandpa::<T, C>(source, finality_event, counterparty)
					.await,
			LightClientProtocol::Beefy =>
				query_latest_ibc_events_with_beefy::<T, C>(source, finality_event, counterparty)
					.await,
		}
	}
}

/// Query the latest events that have been finalized by the BEEFY finality protocol.
pub async fn query_latest_ibc_events_with_beefy<T, C>(
	source: &mut ParachainClient<T>,
	finality_event: FinalityEvent,
	counterparty: &C,
) -> Result<(UpdateMessage, Vec<IbcEvent>, UpdateType), anyhow::Error>
where
	T: Config + Send + Sync,
	C: Chain,
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	ParachainClient<T>: Chain,
	ParachainClient<T>: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as Config>::Address: From<<T as Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
	T::Hash: From<sp_core::H256>,
	sp_core::H256: From<T::Hash>,
{
	let signed_commitment = match finality_event {
		FinalityEvent::Beefy(signed_commitment) => signed_commitment,
		_ => panic!("Expected beefy signed commitment"),
	};
	let client_id = source.client_id();
	let latest_height = counterparty.latest_height_and_timestamp().await?.0;
	let response = counterparty.query_client_state(latest_height, client_id).await?;
	let client_state = response.client_state.ok_or_else(|| {
		Error::Custom("Received an empty client state from counterparty".to_string())
	})?;
	let client_state = AnyClientState::try_from(client_state)
		.map_err(|_| Error::Custom("Failed to decode client state".to_string()))?;
	let beefy_client_state = match &client_state {
		AnyClientState::Beefy(client_state) => BeefyPrimitivesClientState {
			latest_beefy_height: client_state.latest_beefy_height,
			mmr_root_hash: client_state.mmr_root_hash,
			current_authorities: client_state.authority.clone(),
			next_authorities: client_state.next_authority_set.clone(),
			beefy_activation_block: client_state.beefy_activation_block,
		},
		c => Err(Error::ClientStateRehydration(format!(
			"Expected AnyClientState::Beefy found: {:?}",
			c
		)))?,
	};

	if signed_commitment.commitment.validator_set_id < beefy_client_state.current_authorities.id {
		log::info!(
			"Commitment: {:#?}\nClientState: {:#?}",
			signed_commitment.commitment,
			beefy_client_state
		);
		// If validator set id of signed commitment is less than current validator set
		// id we have Then commitment is outdated and we skip it.
		log::warn!(
				"Skipping outdated commitment \n Received signed commitmment with validator_set_id: {:?}\n Current authority set id: {:?}\n Next authority set id: {:?}\n",
				signed_commitment.commitment.validator_set_id, beefy_client_state.current_authorities.id, beefy_client_state.next_authorities.id
			);
		Err(Error::HeaderConstruction("Received an outdated beefy commitment".to_string()))?
	}

	// fetch the new parachain headers that have been finalized
	let headers = source
		.query_beefy_finalized_parachain_headers_between(
			signed_commitment.commitment.block_number,
			&beefy_client_state,
		)
		.await?;

	log::info!(
		"Fetching events from {} for blocks {}..{}",
		source.name(),
		headers[0].number(),
		headers.last().unwrap().number()
	);

	// Get finalized parachain block numbers, but only those higher than the latest para
	// height recorded in the on-chain client state, because in some cases a parachain
	// block that was already finalized in a former beefy block might still be part of
	// the parachain headers in a later beefy block, discovered this from previous logs
	let finalized_blocks =
		headers.iter().map(|header| u32::from(*header.number())).collect::<Vec<_>>();

	let finalized_block_numbers = finalized_blocks
		.iter()
		.filter_map(|block_number| {
			if (client_state.latest_height().revision_height as u32) < *block_number {
				Some(*block_number)
			} else {
				None
			}
		})
		.map(|h| BlockNumberOrHash::Number(h))
		.collect::<Vec<_>>();

	// 1. we should query the sink chain for any outgoing packets to the source chain
	// and return the maximum height at which we can construct non-existence proofs for
	// all these packets on the source chain
	let max_height_for_timeouts =
		find_maximum_height_for_timeout_proofs(counterparty, source).await;
	let timeout_update_required = if let Some(max_height) = max_height_for_timeouts {
		let max_height = max_height as u32;
		finalized_blocks.contains(&max_height)
	} else {
		false
	};

	let latest_finalized_block = finalized_blocks.into_iter().max().unwrap_or_default();

	let authority_set_changed =
		signed_commitment.commitment.validator_set_id == beefy_client_state.next_authorities.id;

	let is_update_required = source.is_update_required(
		latest_finalized_block.into(),
		client_state.latest_height().revision_height,
	);

	// if validator set has changed this is a mandatory update
	let update_type = match authority_set_changed || timeout_update_required || is_update_required {
		true => UpdateType::Mandatory,
		false => UpdateType::Optional,
	};

	// block_number => events
	let events: HashMap<String, Vec<IbcEvent>> =
		IbcApiClient::<u32, H256>::query_events(&*source.para_ws_client, finalized_block_numbers)
			.await?;

	// header number is serialized to string
	let mut headers_with_events = events
		.iter()
		.filter_map(|(num, events)| {
			if events.is_empty() {
				None
			} else {
				str::parse::<u32>(&*num).ok().map(T::BlockNumber::from)
			}
		})
		.collect::<BTreeSet<_>>();

	let events: Vec<IbcEvent> = events.into_values().flatten().collect();

	if timeout_update_required {
		let max_height_for_timeouts = max_height_for_timeouts.unwrap();
		if max_height_for_timeouts > client_state.latest_height().revision_height {
			let max_timeout_height = T::BlockNumber::from(max_height_for_timeouts as u32);
			headers_with_events.insert(max_timeout_height);
		}
	}

	if is_update_required {
		headers_with_events.insert(T::BlockNumber::from(latest_finalized_block));
	}

	// only query proofs for headers that actually have events or are mandatory
	let headers_with_proof = if !headers_with_events.is_empty() {
		let (headers, batch_proof) = source
			.query_beefy_finalized_parachain_headers_with_proof(
				signed_commitment.commitment.block_number,
				&beefy_client_state,
				headers_with_events.into_iter().collect(),
			)
			.await?;
		let mmr_size = NodesUtils::new(batch_proof.leaf_count).size();

		Some(ParachainHeadersWithProof {
			headers,
			mmr_size,
			mmr_proofs: batch_proof.items.into_iter().map(|item| item.encode()).collect(),
		})
	} else {
		None
	};

	let mmr_update = source
		.query_beefy_mmr_update_proof(signed_commitment, &beefy_client_state)
		.await?;

	for event in events.iter() {
		if source.sender.send(event.clone()).is_err() {
			log::trace!("Failed to push {event:?} to stream, no active receiver found");
			break
		}
	}

	let update_header = {
		let msg = MsgUpdateAnyClient::<LocalClientTypes> {
			client_id: source.client_id(),
			client_message: AnyClientMessage::Beefy(BeefyClientMessage::Header(BeefyHeader {
				headers_with_proof,
				mmr_update_proof: Some(mmr_update),
			})),
			signer: counterparty.account_id(),
		};
		let value = msg.encode_vec();
		Any { value, type_url: msg.type_url() }
	};

	Ok((UpdateMessage::Single(update_header), events, update_type))
}

/// Query the latest events that have been finalized by the GRANDPA finality protocol.
pub async fn query_latest_ibc_events_with_grandpa<T, C>(
	source: &mut ParachainClient<T>,
	finality_event: FinalityEvent,
	counterparty: &C,
) -> Result<(UpdateMessage, Vec<IbcEvent>, UpdateType), anyhow::Error>
where
	T: Config + Send + Sync,
	C: Chain,
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	ParachainClient<T>: Chain,
	ParachainClient<T>: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as Config>::Address: From<<T as Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256>,
	sp_core::H256: From<T::Hash>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
{
	let justification = match finality_event {
		FinalityEvent::Grandpa(justification) => justification,
		_ => panic!("Expected grandpa finality event"),
	};
	let client_id = source.client_id();
	let latest_height = counterparty.latest_height_and_timestamp().await?.0;
	let response = counterparty.query_client_state(latest_height, client_id).await?;
	let client_state = response.client_state.ok_or_else(|| {
		Error::Custom("Received an empty client state from counterparty".to_string())
	})?;
	let client_state = AnyClientState::try_from(client_state)
		.map_err(|_| Error::Custom("Failed to decode client state".to_string()))?;
	let grandpa_client_state = match &client_state {
		AnyClientState::Grandpa(client_state) => client_state,
		c => Err(Error::ClientStateRehydration(format!(
			"Expected AnyClientState::Grandpa found: {:?}",
			c
		)))?,
	};

	let missed_updates = source
		.find_missed_mandatory_update(
			counterparty,
			grandpa_client_state.latest_relay_height,
			justification.commit.target_number,
		)
		.await
		.ok()
		.flatten();

	// fetch the new parachain headers that have been finalized
	// If we find missed an updates we want to start querying the relay chain for parachain blocks
	// blocks from the missed update height instead of the previous light client height
	// Since we would have fetched parachain headers for the previous light client height while
	// fetching proofs for the missed update
	let previous_finalized_relay_height = missed_updates
		.as_ref()
		.map(|(.., height)| *height)
		.unwrap_or(grandpa_client_state.latest_relay_height);
	let headers = source
		.query_grandpa_finalized_parachain_headers_between(
			justification.commit.target_number,
			previous_finalized_relay_height,
		)
		.await?
		.ok_or_else(|| {
			Error::from(
				"[query_latest_ibc_events_with_grandpa] No parachain headers have been finalized"
					.to_string(),
			)
		})?;

	log::info!(
		"Fetching events from {} for blocks {}..{}",
		source.name(),
		headers[0].number(),
		headers.last().unwrap().number()
	);

	let finalized_blocks =
		headers.iter().map(|header| u32::from(*header.number())).collect::<Vec<_>>();

	let finalized_block_numbers = finalized_blocks
		.iter()
		.filter_map(|block_number| {
			if (client_state.latest_height().revision_height as u32) < *block_number {
				Some(*block_number)
			} else {
				None
			}
		})
		.map(|h| BlockNumberOrHash::Number(h))
		.collect::<Vec<_>>();

	// 1. we should query the sink chain for any outgoing packets to the source chain
	// and return the maximum height at which we can construct non-existence proofs for
	// all these packets on the source chain
	let max_height_for_timeouts =
		find_maximum_height_for_timeout_proofs(counterparty, source).await;
	let timeout_update_required = if let Some(max_height) = max_height_for_timeouts {
		let max_height = max_height as u32;
		finalized_blocks.contains(&max_height)
	} else {
		false
	};

	let latest_finalized_block = finalized_blocks.into_iter().max().unwrap_or_default();

	let is_update_required = source.is_update_required(
		latest_finalized_block.into(),
		client_state.latest_height().revision_height,
	);

	let target = source
		.relay_client
		.rpc()
		.header(Some(justification.commit.target_hash.into()))
		.await?
		.ok_or_else(|| {
			Error::from("Could not find relay chain header for justification target".to_string())
		})?;

	let authority_set_changed_scheduled = find_scheduled_change(&target).is_some();
	// if validator set has changed this is a mandatory update
	let update_type = match authority_set_changed_scheduled ||
		missed_updates.is_some() ||
		timeout_update_required ||
		is_update_required
	{
		true => UpdateType::Mandatory,
		false => UpdateType::Optional,
	};

	// block_number => events
	let events: HashMap<String, Vec<IbcEvent>> =
		IbcApiClient::<u32, H256>::query_events(&*source.para_ws_client, finalized_block_numbers)
			.await?;

	// header number is serialized to string
	let mut headers_with_events = events
		.iter()
		.filter_map(|(num, events)| {
			if events.is_empty() {
				None
			} else {
				str::parse::<u32>(&*num).ok().map(T::BlockNumber::from)
			}
		})
		.collect::<BTreeSet<_>>();

	let events: Vec<IbcEvent> = events.into_values().flatten().collect();

	if timeout_update_required {
		let max_height_for_timeouts = max_height_for_timeouts.unwrap();
		if max_height_for_timeouts > client_state.latest_height().revision_height {
			let max_timeout_height = T::BlockNumber::from(max_height_for_timeouts as u32);
			headers_with_events.insert(max_timeout_height);
		}
	}

	if is_update_required {
		headers_with_events.insert(T::BlockNumber::from(latest_finalized_block));
	}

	let ParachainHeadersWithFinalityProof { finality_proof, parachain_headers } = source
		.query_grandpa_finalized_parachain_headers_with_proof(
			justification.commit.target_number.into(),
			previous_finalized_relay_height,
			headers_with_events.into_iter().collect(),
		)
		.await?;
	let grandpa_header = GrandpaHeader {
		finality_proof: finality_proof.into(),
		parachain_headers: parachain_headers.into(),
	};

	for event in events.iter() {
		if source.sender.send(event.clone()).is_err() {
			log::trace!("Failed to push {event:?} to stream, no active receiver found");
			break
		}
	}

	let update_header = {
		let msg = MsgUpdateAnyClient::<LocalClientTypes> {
			client_id: source.client_id(),
			client_message: AnyClientMessage::Grandpa(ClientMessage::Header(grandpa_header)),
			signer: counterparty.account_id(),
		};
		let value = msg.encode_vec();
		Any { value, type_url: msg.type_url() }
	};

	if let Some((mut missed_updates, ..)) = missed_updates {
		missed_updates.push(update_header);
		Ok((UpdateMessage::Batch(missed_updates), events, update_type))
	} else {
		Ok((UpdateMessage::Single(update_header), events, update_type))
	}
}
