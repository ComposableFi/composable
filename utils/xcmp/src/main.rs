mod generated;

use base58::ToBase58;
use clap::{clap_derive::ArgEnum, Parser, Subcommand};
// TODO: allow to pass name as key and use ALICE, BOB, etc
//use sp_keyring::AccountKeyring;
use scale_codec::{Decode, Encode};
use sp_core::crypto::*;
use subxt::{
	sp_core::{crypto::AccountId32, sr25519, Pair},
	sp_runtime::MultiAddress,
	Call, ClientBuilder, DefaultConfig, PolkadotExtrinsicParams, SubmittableExtrinsic,
	SubstrateExtrinsicParams, TransactionInBlock, TransactionStatus,
};

// TODO: use latest version of sp_core after upgrade (because subxt does not generates these
// anymore) use sp_core::{
// 	crypto::Pair,
// 	sr25519,
// };

pub type PairSigner = subxt::PairSigner<DefaultConfig, sr25519::Pair>;

use crate::generated::rococo_relay_chain::{
	self,
	api::{
		self,
		runtime_types::{
			polkadot_parachain::primitives,
			xcm::{
				v0::junction::NetworkId,
				v1::{
					junction::Junction,
					multiasset::{AssetId, Fungibility, MultiAsset, MultiAssets},
				},
				*,
			},
		},
		xcm_pallet::calls::*,
	},
};

#[derive(Parser, Debug)]
#[clap(about ="XCMP tools", long_about = None)]
struct Args {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Subcommand, Debug)]
#[clap()]
enum Command {
	// https://substrate.stackexchange.com/questions/1200/how-to-calculate-sovereignaccount-for-parachain/1210#1210
	Parachain(Address),
	// TODO: unify transfer under single command
	TransferNative(TransferNative),
	ReserveTransferNative(ReserveTransferNative),
}

#[derive(Parser, Debug)]
struct TransferNative {
	pub from_account_id: String,
	pub to_account_id: String,
	pub amount: u128,
	pub rpc: String,
}

#[derive(Parser, Debug)]
struct AcceptChannelOpen {
	pub para_id: u32,
	pub root: String,
	pub rpc: String,
}

#[derive(Parser, Debug)]
struct Address {
	pub para_id: u32,
	#[clap(arg_enum, value_parser, default_value_t = AddressFormat::Base58)]
	pub format: AddressFormat,
}

#[derive(Parser, Debug)]
struct ReserveTransferNative {
	pub from_account_id: String,
	pub to_para_id: u32,
	pub to_account_id: String,
	pub amount: u128,
	pub rpc: String,
}

#[derive(Parser, Debug, Clone, ArgEnum)]
pub enum AddressFormat {
	Hex,
	Base58,
}

pub fn pair_signer(pair: sr25519::Pair) -> PairSigner {
	PairSigner::new(pair)
}

#[tokio::main]
pub async fn main() {
	let args = Args::parse();
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
	}
}

async fn transfer_native_asset(command: TransferNative) {
	println!("{:?}", &command);
	let api = ClientBuilder::new()
		.set_url(command.rpc)
		.build()
		.await
		.unwrap()
		.to_runtime_api::<rococo_relay_chain::api::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<_>>>(
	);
	let signer = pair_signer(
		sr25519::Pair::from_string(&command.from_account_id, None)
			.expect("provided key is not valid"),
	);
	let beneficiary = MultiAddress::Id(AccountId32::new(
		sp_keyring::sr25519::sr25519::Public::from_string(command.to_account_id.as_str())
			.unwrap()
			.into(),
	));

	let mut balance_transfer_progress = api
		.tx()
		.balances()
		.transfer(beneficiary, command.amount)
		.unwrap()
		.sign_and_submit_then_watch_default(&signer)
		.await
		.unwrap();

	while let Some(ev) = balance_transfer_progress.next_item().await {
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
	let signer = pair_signer(
		sr25519::Pair::from_string(&command.from_account_id, None)
			.expect("provided key is not valid"),
	);

	let api = ClientBuilder::new()
		.set_url(command.rpc)
		.build()
		.await
		.unwrap()
		.to_runtime_api::<rococo_relay_chain::api::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<_>>>(
	);

	let beneficiary =
		sp_keyring::sr25519::sr25519::Public::from_string(command.to_account_id.as_str()).unwrap();

	let beneficiary = VersionedMultiLocation::V1(v1::multilocation::MultiLocation {
		parents: 0,
		interior: v1::multilocation::Junctions::X1(Junction::AccountId32 {
			network: NetworkId::Any,
			id: beneficiary.into(),
		}),
	});

	let mut tx = api
		.tx()
		.xcm_pallet()
		.reserve_transfer_assets(destination, beneficiary, assets, 0)
		.unwrap()
		.sign_and_submit_then_watch_default(&signer)
		.await
		.unwrap();

	while let Some(ev) = tx.next_item().await {
		println!("{:?}", ev);
		if let Ok(TransactionStatus::Finalized(block)) = ev {
			println!("https://rococo.subscan.io/extrinsic/{:?}", block.extrinsic_hash());
		}
	}
}

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
			let account = sp_core::crypto::AccountId32::from_slice(&hex[0..32]).unwrap();
			account.to_ss58check()
		},
	};
	println!("{:?}", result);
}
