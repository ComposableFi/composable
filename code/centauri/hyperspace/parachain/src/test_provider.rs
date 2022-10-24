use crate::{
	parachain::api, polkadot, signer::ExtrinsicSigner, utils::unsafe_cast_to_jsonrpsee_client,
	Error, GrandpaClientState, ParachainClient,
};
use beefy_prover::Prover;
use codec::Decode;
use common::AccountId;
use finality_grandpa::BlockNumberOps;
use futures::{Stream, StreamExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use grandpa_prover::GrandpaProver;
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::{ChannelId, ClientId, PortId},
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
use primitives::{KeyProvider, TestProvider};
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	H256,
};
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
{
	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
	}

	pub async fn submit_create_client_msg(&self, msg: pallet_ibc::Any) -> Result<ClientId, Error> {
		let call = api::tx().ibc().deliver(vec![api::runtime_types::pallet_ibc::Any {
			type_url: msg.type_url,
			value: msg.value,
		}]);
		let (ext_hash, block_hash) = self.submit_call(call).await?;

		// Query newly created client Id
		let identified_client_state = IbcApiClient::<u32, H256>::query_newly_created_client(
			&*self.para_ws_client,
			block_hash.into(),
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

		self.submit_call(call).await?;

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
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
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

		self.submit_call(call).await.map(|_| ())
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

	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		self.channel_whitelist = channel_whitelist;
	}
}
