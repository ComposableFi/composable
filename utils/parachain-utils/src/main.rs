#![allow(unknown_lints, panics)]

use codec::Encode;
use jsonrpc_core_client::RpcError;
use parachain_system::Call as ParachainSystemCall;
use serde::Deserialize;
use sp_core::{crypto::AccountId32, sr25519, Pair, H256};
use sp_runtime::{generic::Era, MultiSignature, MultiSigner};
use structopt::StructOpt;
use substrate_xt::{AdrressFor, Client, ConstructExt, ExtrinsicError, SubstrateXtError};

#[macro_use]
mod events;

use events::AllRuntimeEvents;

const DALI: &str = "dali";
const PICASSO: &str = "picasso";

/// The command options
#[derive(Debug, StructOpt, Clone)]
pub enum Main {
	// RotateKeys,
	UpgradeRuntime {
		/// path to wasm file
		#[structopt(long)]
		path: String,
	},
}

/// The chain option
#[derive(Debug, StructOpt, Clone)]
pub struct Chain {
	/// Chain id [`picasso`, `composable` or `dali`]
	#[structopt(long)]
	chain_id: String,

	#[structopt(subcommand)]
	main: Main,
}

#[derive(Deserialize, Debug)]
struct Env {
	/// Root key used to sign transactions
	root_key: String,
	/// Url to dali rpc node
	rpc_node: String,
}

struct State<T: ConstructExt> {
	/// substrate-xt client
	api: Client<T>,
	/// Pair signer
	signer: sr25519::Pair,
}

struct DaliXtConstructor;

impl ConstructExt for DaliXtConstructor {
	type Runtime = dali_runtime::Runtime;
	type Pair = sp_core::sr25519::Pair;
	type SignedExtra = dali_runtime::SignedExtra;

	fn signed_extras(
		account_id: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtra {
		let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(account_id);
		(
			frame_system::CheckNonZeroSender::<Self::Runtime>::new(),
			frame_system::CheckSpecVersion::<Self::Runtime>::new(),
			frame_system::CheckTxVersion::<Self::Runtime>::new(),
			frame_system::CheckGenesis::<Self::Runtime>::new(),
			frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Self::Runtime>::from(nonce),
			frame_system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}

struct PicassoXtConstructor;

impl ConstructExt for PicassoXtConstructor {
	type Runtime = picasso_runtime::Runtime;
	type Pair = sp_core::sr25519::Pair;
	type SignedExtra = picasso_runtime::SignedExtra;

	fn signed_extras(
		account_id: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtra {
		let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(account_id);
		(
			frame_system::CheckNonZeroSender::<Self::Runtime>::new(),
			frame_system::CheckSpecVersion::<Self::Runtime>::new(),
			frame_system::CheckTxVersion::<Self::Runtime>::new(),
			frame_system::CheckGenesis::<Self::Runtime>::new(),
			frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Self::Runtime>::from(nonce),
			frame_system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}

impl<T: ConstructExt + Send + Sync> State<T> {
	async fn new() -> Self {
		let env = envy::from_env::<Env>().unwrap();
		// create the signer
		let signer = sr25519::Pair::from_string(&env.root_key, None).unwrap();

		let api = Client::new(&env.rpc_node).await.unwrap();

		State { api, signer }
	}
}

#[derive(derive_more::From, Debug)]
enum Error {
	SubXt(SubstrateXtError),
	Rpc(RpcError),
	Io(std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	env_logger::init();
	let chain = Chain::from_args();
	match &*chain.chain_id {
		PICASSO => {
			let state = State::<PicassoXtConstructor>::new().await;
			match chain.main {
				// Main::RotateKeys => rotate_keys(&state).await?,
				Main::UpgradeRuntime { path } => {
					let wasm = std::fs::read(path).unwrap();
					upgrade_runtime(wasm, &state).await?;
				},
			};
		},

		DALI => {
			let state = State::<DaliXtConstructor>::new().await;
			match chain.main {
				// Main::RotateKeys => rotate_keys(&state).await?,
				Main::UpgradeRuntime { path } => {
					let wasm = std::fs::read(path).unwrap();
					upgrade_runtime(wasm, &state).await?;
				},
			};
		},
		_ => panic!("Unsupported chain_id: {}", chain.chain_id),
	};

	Ok(())
}

// async fn rotate_keys(state: &State) -> Result<(), Error> {
// 	let url = url::Url::from_str(&state.env.rpc_node).unwrap();
// 	let rpc_channel = ws::connect::<RpcChannel>(&url).await?;
// 	let dali_author: AuthorClient<common::Hash, common::Hash> = rpc_channel.clone().into();
//
// 	// first rotate, our keys.
// 	let bytes = dali_author.rotate_keys().await?.to_vec();
// 	use chachacha::api::runtime_types::rococo_runtime::SessionKeys;
// 	// assert that our keys have been rotated.
// 	assert!(dali_author.has_session_keys(bytes.clone().into()).await?);
//
// 	// now to set our session keys on cha cha cha
// 	let api = ClientBuilder::new()
// 		.set_url("wss://fullnode-relay.chachacha.centrifuge.io")
// 		.build()
// 		.await?
// 		.to_runtime_api::<chachacha::api::RuntimeApi<chachacha::api::DefaultConfig>>();
//
// 	let signer = PairSigner::new(state.signer.clone());
// 	let account = MultiSigner::from(state.signer.public()).into_account();
//
// 	let _ = api
// 		.tx()
// 		.session()
// 		.set_keys(SessionKeys::decode(&mut &bytes[..]).unwrap(), vec![])
// 		.sign_and_submit_then_watch(&signer)
// 		.await?;
//
// 	// check storage for the new keys
// 	let key_bytes = api
// 		.storage()
// 		.session()
// 		.next_keys(account, None)
// 		.await?
// 		.ok_or_else(|| subxt::Error::Other("Failed to set keys!".into()))?
// 		.encode();
//
// 	// should match
// 	assert_eq!(bytes, key_bytes);
//
// 	Ok(())
// }

/// Generic function to upgrade runtime.
async fn upgrade_runtime<T: ConstructExt<Pair = sr25519::Pair> + Send + Sync>(
	code: Vec<u8>,
	state: &State<T>,
) -> Result<(), SubstrateXtError>
where
	<T::Runtime as frame_system::Config>::AccountId: From<AccountId32>,
	<T::Runtime as frame_system::Config>::Event: Into<AllRuntimeEvents>,
	<T::Runtime as frame_system::Config>::Call: Encode + Send + Sync,
	T::Runtime: parachain_system::Config + frame_system::Config,
	<T::Runtime as frame_system::Config>::Hash: From<H256>,
	MultiSigner: From<<T::Pair as sp_core::Pair>::Public>,
	MultiSignature: From<<T::Pair as Pair>::Signature>,
	AdrressFor<T>: From<AccountId32>,
	<T::Runtime as frame_system::Config>::Call: From<parachain_system::Call<T::Runtime>>,
{
	let code_hash = H256::from(sp_io::hashing::blake2_256(&code));

	let call = ParachainSystemCall::authorize_upgrade { code_hash: code_hash.into() };
	let xt = state.api.construct_extrinsic(call.into(), state.signer.clone())?;
	let progress = state.api.submit_and_watch(xt).await?;

	let events = state
		.api
		.with_rpc_externalities(Some(progress.wait_for_finalized().await?), || {
			frame_system::Pallet::<<T as ConstructExt>::Runtime>::events()
		});
	let has_event = events.into_iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::UpgradeAuthorized(_)
		)
	});

	if !has_event {
		return Err(ExtrinsicError::Custom("Failed to authorize upgrade".into()).into())
	}

	let call = ParachainSystemCall::enact_authorized_upgrade { code };
	let xt = state.api.construct_extrinsic(call.into(), state.signer.clone())?;
	let progress = state.api.submit_and_watch(xt).await?;

	let events = state
		.api
		.with_rpc_externalities(Some(progress.wait_for_finalized().await?), || {
			frame_system::Pallet::<<T as ConstructExt>::Runtime>::events()
		});

	let has_event = events.into_iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::ValidationFunctionStored
		)
	});
	if !has_event {
		return Err(ExtrinsicError::Custom("Failed to enact upgrade".into()).into())
	}

	log::info!("Runtime upgrade proposed");

	Ok(())
}
