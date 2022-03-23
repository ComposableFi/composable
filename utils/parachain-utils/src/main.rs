use codec::Encode;
use jsonrpc_core_client::RpcError;
use sp_core::{crypto::AccountId32, sr25519, Pair, H256};
use sp_runtime::{generic::Era, MultiSignature, MultiSigner};
use structopt::StructOpt;
use substrate_xt::{AdrressFor, Client, ConstructExt, ExtrinsicError, SubstrateXtError};

#[macro_use]
mod events;

use events::AllRuntimeEvents;

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
	<T::Runtime as frame_system::Config>::AccountId: From<AccountId32>,
	<T::Runtime as frame_system::Config>::Event: Into<AllRuntimeEvents>,
	<T::Runtime as frame_system::Config>::Call:
		Encode + Send + Sync + From<sudo::Call<T::Runtime>> + From<frame_system::Call<T::Runtime>>,
	T::Runtime: frame_system::Config + sudo::Config,
	<T::Runtime as frame_system::Config>::Hash: From<H256>,
	MultiSigner: From<<T::Pair as sp_core::Pair>::Public>,
	MultiSignature: From<<T::Pair as Pair>::Signature>,
	AdrressFor<T>: From<AccountId32>,
	<T::Runtime as sudo::Config>::Call: From<<T::Runtime as frame_system::Config>::Call>,
{
	let xt = state.api.construct_extrinsic(
		sudo::Call::sudo_unchecked_weight {
			call: Box::new(
				<T::Runtime as frame_system::Config>::Call::from(frame_system::Call::set_code {
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
		return Err(ExtrinsicError::Custom("Failed to propose upgrade".into()).into());
	}

	log::info!("Runtime upgrade proposed");

	Ok(())
}
