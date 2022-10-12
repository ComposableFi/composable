use crate::{
	parachain, parachain::api, polkadot, signer::ExtrinsicSigner,
	utils::unsafe_cast_to_jsonrpsee_client, Error, GrandpaClientState, ParachainClient,
};
use beefy_prover::ClientWrapper;
use codec::Decode;
use common::AccountId;
use futures::{Stream, StreamExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use grandpa_prover::GrandpaProver;
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::{ChannelId, ClientId, PortId},
	events::IbcEvent,
	timestamp::Timestamp,
};
use ibc_rpc::IbcApiClient;
use ics10_grandpa::consensus_state::ConsensusState as GrandpaConsensusState;
use ics11_beefy::{
	client_state::ClientState as BeefyClientState,
	consensus_state::ConsensusState as BeefyConsensusState,
};
use jsonrpsee::core::client::SubscriptionClientT;
use pallet_ibc::{
	light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager},
	MultiAddress, Timeout, TransferParams,
};
use primitives::{BalancesAccountData, KeyProvider, TestProvider};
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	H256,
};
use sp_finality_grandpa::SetId;
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use std::{collections::BTreeMap, fmt::Display, pin::Pin, str::FromStr, time::Duration};
use subxt::{
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams, SubstrateExtrinsicParamsBuilder},
	Config,
};
use tokio_stream::wrappers::BroadcastStream;

impl<T: Config + Send + Sync> ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as Config>::Address: From<<T as Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>>,
	T::BlockNumber: Ord + sp_runtime::traits::Zero + One,
	T::Header: HeaderT,
	<T::Header as HeaderT>::Hash: From<T::Hash>,
	T::BlockNumber: From<u32>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as Config>::Hash, ParachainHeaderProofs>>,
	T::BlockNumber: Ord + sp_runtime::traits::Zero,
{
	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
	}

	/// Construct a beefy client state to be submitted to the counterparty chain
	pub async fn construct_beefy_client_state(
		&self,
		beefy_activation_block: u32,
	) -> Result<(AnyClientState, AnyConsensusState), Error>
	where
		Self: KeyProvider,
		<T::Signature as Verify>::Signer:
			From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
		MultiSigner: From<MultiSigner>,
		<T as Config>::Address: From<<T as Config>::AccountId>,
		u32: From<<T as Config>::BlockNumber>,
	{
		use ibc::core::ics24_host::identifier::ChainId;
		let api = self.relay_client.storage();
		let para_client_api = self.para_client.storage();
		let client_wrapper = ClientWrapper {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block,
			para_id: self.para_id,
		};
		loop {
			let beefy_state = client_wrapper
				.construct_beefy_client_state(beefy_activation_block)
				.await
				.map_err(|e| {
					Error::from(format!("[construct_beefy_client_state] Failed due to {:?}", e))
				})?;

			let subxt_block_number: subxt::rpc::BlockNumber =
				beefy_state.latest_beefy_height.into();
			let block_hash = self.relay_client.rpc().block_hash(Some(subxt_block_number)).await?;
			let heads_addr = polkadot::api::storage().paras().heads(
				&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(self.para_id),
			);
			let head_data = api.fetch(&heads_addr, block_hash).await?.ok_or_else(|| {
				Error::Custom(format!(
					"Couldn't find header for ParaId({}) at relay block {:?}",
					self.para_id, block_hash
				))
			})?;
			let decoded_para_head = sp_runtime::generic::Header::<
				u32,
				sp_runtime::traits::BlakeTwo256,
			>::decode(&mut &*head_data.0)?;
			let block_number = decoded_para_head.number;
			let client_state = BeefyClientState::<HostFunctionsManager> {
				chain_id: ChainId::new("relay-chain".to_string(), 0),
				relay_chain: Default::default(),
				mmr_root_hash: beefy_state.mmr_root_hash,
				latest_beefy_height: beefy_state.latest_beefy_height,
				frozen_height: None,
				beefy_activation_block: beefy_state.beefy_activation_block,
				latest_para_height: block_number,
				para_id: self.para_id,
				authority: beefy_state.current_authorities,
				next_authority_set: beefy_state.next_authorities,
				_phantom: Default::default(),
			};
			// we can't use the genesis block to construct the initial state.
			if block_number == 0 {
				continue
			}
			let subxt_block_number: subxt::rpc::BlockNumber = block_number.into();
			let block_hash =
				self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
			let timestamp_addr = api::storage().timestamp().now();
			let unix_timestamp_millis = para_client_api
				.fetch(&timestamp_addr, block_hash)
				.await?
				.expect("Timestamp should exist");
			let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

			let consensus_state = AnyConsensusState::Beefy(BeefyConsensusState {
				timestamp: Timestamp::from_nanoseconds(timestamp_nanos)
					.unwrap()
					.into_tm_time()
					.unwrap(),
				root: decoded_para_head.state_root.as_bytes().to_vec().into(),
			});

			return Ok((AnyClientState::Beefy(client_state), consensus_state))
		}
	}

	pub async fn construct_grandpa_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Error>
	where
		Self: KeyProvider,
		<T::Signature as Verify>::Signer:
			From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
		MultiSigner: From<MultiSigner>,
		<T as Config>::Address: From<<T as Config>::AccountId>,
		u32: From<<T as Config>::BlockNumber>,
	{
		let relay_ws_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.relay_ws_client) };
		let para_ws_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.para_ws_client) };
		let prover = GrandpaProver {
			relay_client: self.relay_client.clone(),
			relay_ws_client,
			para_client: self.para_client.clone(),
			para_ws_client,
			para_id: self.para_id,
		};
		let api = self.relay_client.storage();
		let para_client_api = self.para_client.storage();
		loop {
			let light_client_state = prover
				.initialize_client_state()
				.await
				.map_err(|_| Error::from("Error constructing client state".to_string()))?;

			let heads_addr = polkadot::api::storage().paras().heads(
				&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(self.para_id),
			);
			let head_data = api
				.fetch(&heads_addr, Some(light_client_state.latest_relay_hash))
				.await?
				.ok_or_else(|| {
					Error::Custom(format!(
						"Couldn't find header for ParaId({}) at relay block {:?}",
						self.para_id, light_client_state.latest_relay_hash
					))
				})?;
			let decoded_para_head = sp_runtime::generic::Header::<
				u32,
				sp_runtime::traits::BlakeTwo256,
			>::decode(&mut &*head_data.0)?;
			let block_number = decoded_para_head.number;
			// we can't use the genesis block to construct the initial state.
			if block_number == 0 {
				continue
			}

			let mut client_state = GrandpaClientState::<HostFunctionsManager>::default();

			client_state.relay_chain = Default::default();
			client_state.current_authorities = light_client_state.current_authorities;
			client_state.current_set_id = light_client_state.current_set_id;
			client_state.latest_relay_hash = light_client_state.latest_relay_hash.into();
			client_state.frozen_height = None;
			client_state.latest_para_height = block_number;
			client_state.para_id = self.para_id;
			client_state.latest_relay_height = light_client_state.latest_relay_height;

			let subxt_block_number: subxt::rpc::BlockNumber = block_number.into();
			let block_hash =
				self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
			let timestamp_addr = api::storage().timestamp().now();
			let unix_timestamp_millis = para_client_api
				.fetch(&timestamp_addr, block_hash)
				.await?
				.expect("Timestamp should exist");
			let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

			let consensus_state = AnyConsensusState::Grandpa(GrandpaConsensusState {
				timestamp: Timestamp::from_nanoseconds(timestamp_nanos)
					.unwrap()
					.into_tm_time()
					.unwrap(),
				root: decoded_para_head.state_root.as_bytes().to_vec().into(),
			});

			return Ok((AnyClientState::Grandpa(client_state), consensus_state))
		}
	}

	pub async fn submit_create_client_msg(&self, msg: pallet_ibc::Any) -> Result<ClientId, Error> {
		let call = api::tx().ibc().deliver(vec![api::runtime_types::pallet_ibc::Any {
			type_url: msg.type_url,
			value: msg.value,
		}]);
		let (ext_hash, block_hash) = self.submit_call(call, &self.para_client).await?;

		// Query newly created client Id
		let identified_client_state = IbcApiClient::<u32, H256>::query_newly_created_client(
			&*self.para_ws_client,
			block_hash.unwrap().into(),
			ext_hash.into(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		let client_id = ClientId::from_str(&identified_client_state.client_id)
			.expect("Should have a valid client id");
		Ok(client_id)
	}

	pub async fn transfer_tokens(
		&self,
		params: TransferParams<AccountId>,
		asset_id: u128,
		amount: u128,
	) -> Result<(), Error> {
		let params = api::runtime_types::pallet_ibc::TransferParams {
			to: match params.to {
				MultiAddress::Id(id) => {
					let id: [u8; 32] = id.into();
					api::runtime_types::pallet_ibc::MultiAddress::Id(id.into())
				},
				MultiAddress::Raw(raw) => api::runtime_types::pallet_ibc::MultiAddress::Raw(raw),
			},

			source_channel: params.source_channel,
			timeout: match params.timeout {
				Timeout::Offset { timestamp, height } =>
					api::runtime_types::pallet_ibc::Timeout::Offset { timestamp, height },
				Timeout::Absolute { timestamp, height } =>
					api::runtime_types::pallet_ibc::Timeout::Absolute { timestamp, height },
			},
		};
		// Submit extrinsic to parachain node
		let call = api::tx().ibc().transfer(
			params,
			api::runtime_types::primitives::currency::CurrencyId(asset_id),
			amount.into(),
		);

		self.submit_call(call, &self.para_client).await?;

		Ok(())
	}

	pub async fn submit_sudo_call(
		&self,
		call: api::runtime_types::dali_runtime::Call,
	) -> Result<(), Error> {
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		let ext = api::tx().sudo().sudo(call);
		// Submit extrinsic to parachain node

		let tx_params = SubstrateExtrinsicParamsBuilder::new()
			.tip(AssetTip::new(100_000))
			.era(Era::Immortal, self.para_client.genesis_hash());

		let _progress = self
			.para_client
			.tx()
			.sign_and_submit_then_watch(&ext, &signer, tx_params.into())
			.await?
			.wait_for_in_block()
			.await?
			.wait_for_success()
			.await?;

		Ok(())
	}

	pub async fn set_pallet_params(
		&self,
		receive_enabled: bool,
		send_enabled: bool,
	) -> Result<(), Error> {
		let params = api::runtime_types::pallet_ibc::PalletParams { receive_enabled, send_enabled };

		let call = api::runtime_types::dali_runtime::Call::Ibc(
			api::runtime_types::pallet_ibc::pallet::Call::set_params { params },
		);

		self.submit_sudo_call(call).await?;

		Ok(())
	}
}

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
	T::Hash: From<H256>,
	H256: From<T::Hash>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as Config>::Hash, ParachainHeaderProofs>>,
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

		let params = api::runtime_types::pallet_ibc_ping::SendPingParams {
			data: "ping".as_bytes().to_vec(),
			timeout_height_offset: timeout_height,
			timeout_timestamp_offset: timestamp,
			channel_id: channel_id.sequence(),
		};

		let call = api::tx().ibc_ping().send_ping(params);

		self.submit_call(call, &self.para_client).await.map(|_| ())
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		let para_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.para_ws_client) };
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

	async fn subscribe_relaychain_blocks(&self) -> Pin<Box<dyn Stream<Item = u32>>> {
		let stream =
			self.relay_client.rpc().subscribe_finalized_blocks().await.unwrap().filter_map(
				|result| futures::future::ready(result.ok().map(|x| u32::from(*x.number()))),
			);
		Box::pin(Box::new(stream))
	}

	async fn current_set_id(&self) -> SetId {
		let api = self.relay_client.storage();
		let current_set_id_addr = polkadot::api::storage().grandpa().current_set_id();
		api.fetch(&current_set_id_addr, None)
			.await
			.ok()
			.flatten()
			.expect("Failed to fetch current set id")
	}

	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		self.channel_whitelist = channel_whitelist;
	}

	async fn query_relaychain_balance(&self) -> Result<BalancesAccountData, Self::Error> {
		let account = self.public_key.clone().into_account();
		log::info!("{:?}", account);
		let api = self.relay_client.storage();
		let addr = polkadot::api::storage().system().account(&account);
		let data = api
			.fetch(&addr, None)
			.await
			.ok()
			.flatten()
			.expect("Failed to fetch relaychain balance")
			.data;
		Ok(BalancesAccountData {
			free: data.free,
			reserved: data.reserved,
			misc_frozen: data.misc_frozen,
			fee_frozen: data.fee_frozen,
		})
	}
}
