use codec::Encode;
use jsonrpc_core_client::RpcError;
use sp_core::{crypto::AccountId32, sr25519, Pair, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use structopt::StructOpt;
use substrate_xt::{AdrressFor, Client, ConstructExt, ExtrinsicError, SubstrateXtError};

use utils_common::*;

const DALI: &'static str = "dali";
const PICASSO: &'static str = "picasso";

/// The command options
#[derive(Debug, StructOpt, Clone)]
pub enum Subcommand {
	// RotateKeys,
	UpgradeRuntime {
		/// path to wasm file
		#[structopt(long)]
		path: String,
	},
}

/// The chain option
#[derive(Debug, StructOpt, Clone)]
pub struct Command {
	/// Chain id [`picasso`, `composable` or `dali`]
	#[structopt(long)]
	chain_id: String,

	/// ws url of the node to query and send extrinsics to
	/// eg wss://rpc.composablefinance.ninja (for dali-rocococ)
	#[structopt(long)]
	rpc_ws_url: String,

	/// Root key used to sign transactions
	#[structopt(long)]
	root_key: String,

	#[structopt(subcommand)]
	main: Subcommand,
}

struct State<T: ConstructExt> {
	/// substrate-xt client
	api: Client<T>,
	/// Pair signer
	signer: sr25519::Pair,
}

impl<T: ConstructExt + Send + Sync> State<T> {
	async fn new(args: &Command) -> Self {
		// create the signer
		let signer = sr25519::Pair::from_string(&args.root_key, None).unwrap();

		let api = Client::new(&args.rpc_ws_url).await.unwrap();

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
	let chain = Command::from_args();
	match &*chain.chain_id {
		id if id.contains(PICASSO) => {
			let state = State::<PicassoXtConstructor>::new(&chain).await;
			match chain.main {
				// Main::RotateKeys => rotate_keys(&state).await?,
				Subcommand::UpgradeRuntime { path } => {
					let wasm = std::fs::read(path).unwrap();
					upgrade_runtime_with_sudo(wasm, &state).await?;
				},
			};
		},

		id if id.contains(DALI) => {
			let state = State::<DaliXtConstructor>::new(&chain).await;
			match chain.main {
				// Main::RotateKeys => rotate_keys(&state).await?,
				Subcommand::UpgradeRuntime { path } => {
					let wasm = std::fs::read(path).unwrap();
					upgrade_runtime_with_sudo(wasm, &state).await?;
				},
			};
		},
		_ => panic!("Unsupported chain_id: {}", chain.chain_id),
	};

	Ok(())
}

/// Generic function to upgrade runtime.
async fn upgrade_runtime_with_sudo<T: ConstructExt<Pair = sr25519::Pair> + Send + Sync>(
	code: Vec<u8>,
	state: &State<T>,
) -> Result<(), SubstrateXtError>
where
	<T::Runtime as system::Config>::AccountId: From<AccountId32>,
	<T::Runtime as system::Config>::Event: Into<AllRuntimeEvents>,
	<T::Runtime as system::Config>::Call:
		Encode + Send + Sync + From<sudo::Call<T::Runtime>> + From<system::Call<T::Runtime>>,
	T::Runtime: system::Config + sudo::Config,
	<T::Runtime as system::Config>::Hash: From<H256>,
	MultiSigner: From<<T::Pair as sp_core::Pair>::Public>,
	MultiSignature: From<<T::Pair as Pair>::Signature>,
	AdrressFor<T>: From<AccountId32>,
	<T::Runtime as sudo::Config>::Call: From<<T::Runtime as system::Config>::Call>,
{
	let xt = state.api.construct_extrinsic(
		sudo::Call::sudo_unchecked_weight {
			call: Box::new(
				<T::Runtime as system::Config>::Call::from(system::Call::set_code {
					code: code.clone(),
				})
				.into(),
			),
			weight: 0,
		}
		.into(),
		state.signer.clone(),
	)?;
	let progress = state.api.submit_and_watch(xt).await?;

	log::info!("Runtime upgrade proposed, waiting for finalization");

	let events = state
		.api
		.with_rpc_externalities(Some(progress.wait_for_finalized().await?), || {
			system::Pallet::<<T as ConstructExt>::Runtime>::events()
		});
	let has_event = events.into_iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::ValidationFunctionStored
		)
	});
	if !has_event {
		return Err(ExtrinsicError::Custom("Failed to propose upgrade".into()).into())
	}

	log::info!("Runtime upgrade proposed");

	Ok(())
}
