use codec::{Decode, Encode};
use dali_runtime::{Runtime as DaliRuntime, SignedExtra as DaliSignedExtra};
use jsonrpc_core_client::{futures::StreamExt, transports::ws, RpcChannel, RpcError};
use parachain_system::Call as ParachainSystemCall;
use picasso_runtime::{Runtime as PicaRuntime, SignedExtra as PicaSignedExtra};
use sc_rpc::author::AuthorClient;
use serde::Deserialize;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use substrate_xt::{Client, ConstructExt, ExtrinsicError, SubstrateXtError};
mod events;
use events::{match_event, AllRuntimeEvents};

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
	/// Chain id [`picasso` or `dali`]
	#[structopt(long)]
	chain_id: String,
}

#[derive(Deserialize, Debug)]
struct Env {
	/// Root key used to sign transactions
	root_key: String,
	/// Url to dali rpc node
	rpc_node: String,
}

struct State<T: ConstructExt> {
	/// Subxt api
	api: Client<T>,
	/// Pair signer
	signer: T::Pair,
	/// Env variables
	env: Env,
}

fn build_client(chain_id: &str, ws_url: &str) -> impl ConstructExt {
	struct XtConstructor;
	match chain_id {
		"picasso" => {
			impl ConstructExt for XtConstructor {
				type Runtime = PicaRuntime;
				type Pair = sp_core::sr25519::Pair;
				type SignedExtra = PicaSignedExtra;

				fn signed_extras(
					account_id: <Self::Runtime as frame_system::Config>::AccountId,
				) -> Self::SignedExtra {
					let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(account_id);
					(
						frame_system::CheckSpecVersion::<Runtime>::new(),
						frame_system::CheckTxVersion::<Runtime>::new(),
						frame_system::CheckGenesis::<Runtime>::new(),
						frame_system::CheckEra::<Runtime>::from(Era::Immortal),
						frame_system::CheckNonce::<Runtime>::from(nonce),
						frame_system::CheckWeight::<Runtime>::new(),
						transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
					)
				}
			}

			Client::<XtConstructor>::new(ws_url).await.unwrap()
		},

		"dali" => {
			impl ConstructExt for XtConstructor {
				type Runtime = DaliRuntime;
				type Pair = sp_core::sr25519::Pair;
				type SignedExtra = DaliSignedExtra;

				fn signed_extras(
					account_id: <Self::Runtime as frame_system::Config>::AccountId,
				) -> Self::SignedExtra {
					let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(account_id);
					(
						frame_system::CheckSpecVersion::<Runtime>::new(),
						frame_system::CheckTxVersion::<Runtime>::new(),
						frame_system::CheckGenesis::<Runtime>::new(),
						frame_system::CheckEra::<Runtime>::from(Era::Immortal),
						frame_system::CheckNonce::<Runtime>::from(nonce),
						frame_system::CheckWeight::<Runtime>::new(),
						transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
					)
				}
			}

			Client::<XtConstructor>::new(ws_url).await.unwrap()
		},
		_ => panic!("Unsupported chain_id: {}", chain_id),
	}
}

impl<T: ConstructExt> State<T> {
	async fn new(chain_id: &str) -> Self {
		let env = envy::from_env::<Env>().unwrap();
		// create the signer
		let signer = sr25519::Pair::from_string(&env.root_key, None).unwrap();

		let api = build_client(chain_id, &env.rpc_node);

		State { api, signer, env }
	}
}

#[derive(derive_more::From, Debug)]
enum Error {
	SubXt(SubstrateXtError),
	Rpc(RpcError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	env_logger::init();
	let chain = Chain::from_args();
	let main = Main::from_args();
	let state = State::new(chain.chain_id).await;

	match main {
		// Main::RotateKeys => rotate_keys(&state).await?,
		Main::UpgradeRuntime { path } => {
			let wasm = std::fs::read(path).unwrap();
			upgrade_runtime(wasm, &state).await?
		},
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

async fn upgrade_runtime<T: ConstructExt>(
	code: Vec<u8>,
	state: &State<T>,
) -> Result<(), SubstrateXtError> {
	let code_hash: H256 = sp_io::hashing::blake2_256(&code).into();

	let call = ParachainSystemCall::authorize_upgrade { code_hash };
	let xt = state.api.construct_extrinsic(call.into(), state.signer.clone())?;
	let progress = state.api.submit_and_watch(xt).await?;

	let events =
		state.api.with_rpc_externalities(Some(progress.wait_for_in_block().await?), || {
			frame_system::Pallet::<<T as ConstructExt>::Runtime>::events()
		});
	let has_event = events.iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::UpgradeAuthorized
		)
	});

	if !has_event {
		return Err(ExtrinsicError::Custom("Failed to authorize upgrade".into()).into())
	}

	let call = <<T as ConstructExt>::Runtime as frame_system::Config>::Call::ParachainSystem(
		ParachainSystemCall::enact_authorized_upgrade { code },
	);
	let xt = state.api.construct_extrinsic(call.into(), state.signer.clone())?;
	let progress = state.api.submit_and_watch(xt).await?;

	let events =
		state.api.with_rpc_externalities(Some(progress.wait_for_in_block().await?), || {
			frame_system::Pallet::<<T as ConstructExt>::Runtime>::events()
		});

	let has_event = events.iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::ValidationFunctionStored
		)
	});
	if !has_event {
		return Err(ExtrinsicError::Custom("Failed to enact upgrade".into()).into())
	}

	log::info!("Runtime upgrade proposed, extrinsic hash: {}", result.extrinsic_hash());

	Ok(())
}
