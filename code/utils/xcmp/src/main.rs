use std::io::Read;

use clap::Parser;
use composable_subxt::generated::{self, composable_dali_on_parity_rococo, dali, picasso};
use sc_cli::utils::*;
use scale_codec::{Decode, Encode};
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	sr25519, Pair,
};

use sp_runtime::MultiAddress;
use subxt::{config::*, tx::*, *};

pub type ComposableConfig =
	WithExtrinsicParams<SubstrateConfig, crate::tx::SubstrateExtrinsicParams<SubstrateConfig>>;

pub type RelayPairSigner = subxt::tx::PairSigner<PolkadotConfig, sr25519::Pair>;
pub type ComposablePairSigner = subxt::tx::PairSigner<ComposableConfig, sr25519::Pair>;

use crate::generated::rococo::{
	self,
	api::runtime_types::xcm::{
		v0::junction::NetworkId,
		v1::{
			junction::Junction,
			multiasset::{AssetId, Fungibility, MultiAsset},
		},
		*,
	},
};

mod config;
use config::*;

pub fn pair_signer(pair: sr25519::Pair) -> ComposablePairSigner {
	ComposablePairSigner::new(pair)
}

pub fn parity_pair_signer(pair: sr25519::Pair) -> RelayPairSigner {
	RelayPairSigner::new(pair)
}

#[tokio::main]
pub async fn main() {
	let args = Args::parse();
	println!("Executing {:?}", args);
	match args.command {
		Command::Parachain(address) => {
			parachain_id_into_address(address);
		},
		Command::ReserveTransferNative(command) => {
			reserve_transfer_native_asset(command).await;
		},
		Command::TransferNative(command) => {
			transfer_native_asset(command).await;
		},
		Command::Sudo(command) => match command.command {
			SudoCommand::Execute(execute) =>
				execute_sudo(execute.ask, execute.call, execute.network, execute.suri, execute.rpc)
					.await,
			_ => todo!("implement"),
		},
	}
}

macro_rules! decode_call {
	($network:ident, $network_runtime: ident, $encoded:ident) => {{
		use $network::api::runtime_types::*;
		// NOTE: tried various ways to compose types into mod name, failed
		// or check this https://github.com/paritytech/subxt/issues/669
		// 			error[E0573]: expected type, found module `dali_runtime`
		//   --> utils/xcmp/src/main.rs:63:16
		//    |
		// 63 |             let call =  concat_idents!($network, _runtime)::Call::decode(&mut
		// &$encoded[..])    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a type
		let call = $network_runtime::Call::decode(&mut &$encoded[..]).expect("invalid call");
		println!("{:?}", &call);
		call
	}};
}

macro_rules! encode_sudo {
	($network:ident, $call:ident) => {
		$network::api::tx().sudo().sudo($call)
	};
}

macro_rules! sudo_call {
	($network:ident, $network_runtime: ident, $call:ident) => {{
		let call = decode_call!($network, $network_runtime, $call);
		let extrinsic = encode_sudo!($network, call);
		extrinsic
	}};
}

async fn execute_sudo(ask: bool, call: String, network: String, suri: String, rpc: String) {
	println!("https://polkadot.js.org/apps/?rpc={:#}#/extrinsics/decode/{:}", &rpc, &call);
	let call = sc_cli::utils::decode_hex(&call).expect("call is not hex encoded");
	let from_file = |path| {
		std::fs::read(path)
			.map(String::from_utf8)
			.unwrap()
			.map(|suri| pair_from_suri(&suri.trim(), None))
			.unwrap()
			.unwrap()
	};

	let key: sr25519::Pair =
		sc_cli::utils::pair_from_suri(&suri, None).unwrap_or_else(|_| from_file(&suri));
	let signer = pair_signer(key);

	// https://github.com/paritytech/subxt/issues/668
	let api = OnlineClient::<ComposableConfig>::from_url(&rpc).await.unwrap();
	match network.as_str() {
		"dali" => {
			let extrinsic = sudo_call!(dali, dali_runtime, call);
			may_be_do_call(ask, api, extrinsic, signer).await;
		},
		"composable_dali_on_parity_rococo" => {
			let extrinsic = sudo_call!(composable_dali_on_parity_rococo, dali_runtime, call);
			may_be_do_call(ask, api, extrinsic, signer).await;
		},
		"picasso" => {
			let extrinsic = sudo_call!(picasso, picasso_runtime, call);
			may_be_do_call(ask, api, extrinsic, signer).await;
		},
		"composable_picasso_on_parity_kusama" => {
			let extrinsic = sudo_call!(composable_picasso_on_parity_kusama, picasso_runtime, call);
			may_be_do_call(ask, api, extrinsic, signer).await;
		},		
		
		_ => panic!("unknown network"),
	}
}

async fn may_be_do_call<CallData: Encode>(
	ask: bool,
	api: OnlineClient<
		subxt::config::WithExtrinsicParams<
			SubstrateConfig,
			BaseExtrinsicParams<SubstrateConfig, AssetTip>,
		>,
	>,
	extrinsic: StaticTxPayload<CallData>,
	signer: PairSigner<
		subxt::config::WithExtrinsicParams<
			SubstrateConfig,
			BaseExtrinsicParams<SubstrateConfig, AssetTip>,
		>,
		sr25519::Pair,
	>,
) {
	if ask {
		println!("type `Yes` or `yes` to sign and submit sudo transaction");
		let mut message = String::new();
		std::io::stdin().read_line(&mut message).expect("console always work");
		message = message.trim().to_lowercase();
		if !(message == "yes") {
			panic!("rejected")
		}
	}
	println!("executing... ");
	let mut result =
		api.tx().sign_and_submit_then_watch_default(&extrinsic, &signer).await.unwrap();
	while let Some(ev) = result.next_item().await {
		println!("{:?}", ev);
		ev.unwrap();
	}

	println!("executed: {:?}", result);
}

async fn transfer_native_asset(command: TransferNative) {
	let api = OnlineClient::<PolkadotConfig>::from_url(&command.rpc).await.unwrap();
	let signer = parity_pair_signer(
		sr25519::Pair::from_string(&command.from_account_id, None)
			.expect("provided key is not valid"),
	);
	let beneficiary = MultiAddress::Id(AccountId32::new(
		sp_keyring::sr25519::sr25519::Public::from_string(command.to_account_id.as_str())
			.unwrap()
			.into(),
	));

	let extrinsic = rococo::api::tx().balances().transfer(beneficiary, command.amount);
	let mut result =
		api.tx().sign_and_submit_then_watch_default(&extrinsic, &signer).await.unwrap();

	while let Some(ev) = result.next_item().await {
		println!("{:?}", ev);
	}
}

async fn reserve_transfer_native_asset(command: ReserveTransferNative) {
	let asset = v1::multilocation::MultiLocation {
		parents: 0,
		interior: v1::multilocation::Junctions::Here,
	};
	let asset =
		MultiAsset { id: AssetId::Concrete(asset), fun: Fungibility::Fungible(command.amount) };
	let assets = VersionedMultiAssets::V1(v1::multiasset::MultiAssets { 0: vec![asset] });
	let destination = VersionedMultiLocation::V1(v1::multilocation::MultiLocation {
		parents: 0,
		interior: v1::multilocation::Junctions::X1(Junction::Parachain(command.to_para_id)),
	});
	let signer = parity_pair_signer(
		sr25519::Pair::from_string(&command.from_account_id, None)
			.expect("provided key is not valid"),
	);

	let api = OnlineClient::<PolkadotConfig>::from_url(&command.rpc).await.unwrap();

	let beneficiary =
		sp_keyring::sr25519::sr25519::Public::from_string(command.to_account_id.as_str()).unwrap();

	let beneficiary = VersionedMultiLocation::V1(v1::multilocation::MultiLocation {
		parents: 0,
		interior: v1::multilocation::Junctions::X1(Junction::AccountId32 {
			network: NetworkId::Any,
			id: beneficiary.into(),
		}),
	});

	let extrinsic =
		rococo::api::tx()
			.xcm_pallet()
			.reserve_transfer_assets(destination, beneficiary, assets, 0);
	let mut result =
		api.tx().sign_and_submit_then_watch_default(&extrinsic, &signer).await.unwrap();

	while let Some(ev) = result.next_item().await {
		println!("{:?}", ev);
		if let Ok(TxStatus::Finalized(block)) = ev {
			println!("https://rococo.subscan.io/extrinsic/{:?}", block.extrinsic_hash());
		}
	}
}

// TODO: PR this to `subkey`
fn parachain_id_into_address(address: Address) {
	//  https://substrate.stackexchange.com/questions/1200/how-to-calculate-sovereignaccount-for-parachain/1210#1210
	let mut hex = Vec::new();
	let mut para = b"para".to_vec();
	let mut number = address.para_id.encode();
	let mut suffix = [0_u8; 24].to_vec();
	hex.append(&mut para);
	hex.append(&mut number);
	hex.append(&mut suffix);
	let result = match address.format {
		AddressFormat::Hex => hex::encode(hex),
		_ => {
			let account = sp_core::crypto::AccountId32::try_from(&hex[0..32]).unwrap();
			account.to_ss58check()
		},
	};
	println!("{:?}", result);
}
