use super::{
	cosmwasm,
	types::api::{
		self,
		cosmwasm::events::{AdminUpdated, Emitted, Executed, Instantiated, Migrated, Uploaded},
		runtime_types::{
			pallet_cosmwasm::pallet::CodeIdentifier,
			primitives::currency::CurrencyId,
			sp_runtime::bounded::{bounded_btree_map::BoundedBTreeMap, bounded_vec::BoundedVec},
		},
	},
};
use crate::error::Error;
use anyhow::anyhow;
use clap::{Args, Subcommand};
use subxt::{
	blocks::ExtrinsicEvents,
	ext::{
		codec::Encode,
		sp_core::Pair,
		sp_runtime::{MultiSignature, MultiSigner},
	},
	tx::PairSigner,
	OnlineClient, SubstrateConfig,
};

#[derive(Args, Debug)]
pub struct Command {
	#[command(subcommand)]
	pub subcommands: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
	/// Upload a CosmWasm contract
	Upload(cosmwasm::Upload),

	/// Instantiate a CosmWasm contract
	Instantiate(cosmwasm::Instantiate),

	/// Execute a CosmWasm contract
	Execute(cosmwasm::Execute),

	/// Migrate a CosmWasm contract
	Migrate(cosmwasm::Migrate),

	/// Update admin of a CosmWasm contract
	UpdateAdmin(cosmwasm::UpdateAdmin),
}

impl Command {
	pub async fn run<P: Pair>(self, pair: P, chain_endpoint: String) -> anyhow::Result<()>
	where
		P::Seed: TryFrom<Vec<u8>>,
		MultiSignature: From<<P as Pair>::Signature>,
		MultiSigner: From<<P as Pair>::Public>,
	{
		match self.subcommands {
			Subcommands::Upload(upload) => {
				let code = upload.fetch_code().await?;
				let events = do_signed_transaction(
					chain_endpoint,
					pair,
					api::tx().cosmwasm().upload(BoundedVec(code)),
				)
				.await?;
				let uploaded = find_and_cast_events::<Uploaded>(&events, true)?;

				println!("[ + ] Contract uploaded.");
				println!("\t- Code Hash: {}", uploaded[0].code_hash);
				println!("\t- Code ID: {}", uploaded[0].code_id);

				print_cosmwasm_events(&events)?;
				Ok(())
			},
			Subcommands::Instantiate(cosmwasm::Instantiate {
				code_id,
				salt,
				admin,
				label,
				funds,
				gas,
				message,
			}) => {
				let events = do_signed_transaction(
					chain_endpoint,
					pair,
					api::tx().cosmwasm().instantiate(
						CodeIdentifier::CodeId(code_id),
						BoundedVec(salt.into()),
						admin,
						BoundedVec(label.into()),
						BoundedBTreeMap(
							funds
								.unwrap_or_default()
								.into_iter()
								.map(|(asset, amount)| (CurrencyId(asset), (amount, true)))
								.collect(),
						),
						gas,
						BoundedVec(message.into()),
					),
				)
				.await?;

				let event = find_and_cast_events::<Instantiated>(&events, true)?;

				println!("[ + ] Contract instantiated.");
				println!("\t- Contract address: {}", event[0].contract);
				print_cosmwasm_events(&events)?;

				Ok(())
			},
			Subcommands::Execute(cosmwasm::Execute { contract, funds, gas, message }) => {
				let events = do_signed_transaction(
					chain_endpoint,
					pair,
					api::tx().cosmwasm().execute(
						contract,
						BoundedBTreeMap(
							funds
								.unwrap_or_default()
								.into_iter()
								.map(|(asset, amount)| (CurrencyId(asset), (amount, true)))
								.collect(),
						),
						gas,
						BoundedVec(message.into()),
					),
				)
				.await?;

				let _ = find_and_cast_events::<Executed>(&events, true)?;

				println!("[ + ] Contract executed.");
				print_cosmwasm_events(&events)?;

				Ok(())
			},
			Subcommands::Migrate(cosmwasm::Migrate { contract, new_code_id, gas, message }) => {
				let events = do_signed_transaction(
					chain_endpoint,
					pair,
					api::tx().cosmwasm().migrate(
						contract,
						CodeIdentifier::CodeId(new_code_id),
						gas,
						BoundedVec(message.into()),
					),
				)
				.await?;
				let _ = find_and_cast_events::<Migrated>(&events, true)?;
				println!("[ + ] Contract migrated.");
				print_cosmwasm_events(&events)?;

				Ok(())
			},
			Subcommands::UpdateAdmin(cosmwasm::UpdateAdmin {
				contract, new_admin, gas, ..
			}) => {
				let events = do_signed_transaction(
					chain_endpoint,
					pair,
					api::tx().cosmwasm().update_admin(contract, new_admin, gas),
				)
				.await?;
				let _ = find_and_cast_events::<AdminUpdated>(&events, true)?;
				println!("[ + ] Contract's admin is updated.");

				Ok(())
			},
		}
	}
}

async fn do_signed_transaction<CallData: Encode, P: Pair>(
	endpoint: String,
	signer: P,
	tx: subxt::tx::StaticTxPayload<CallData>,
) -> anyhow::Result<ExtrinsicEvents<SubstrateConfig>>
where
	MultiSignature: From<<P as Pair>::Signature>,
	MultiSigner: From<<P as Pair>::Public>,
{
	let signer = PairSigner::new(signer);
	let api = OnlineClient::<SubstrateConfig>::from_url(endpoint).await?;
	let events = api
		.tx()
		.sign_and_submit_then_watch_default(&tx, &signer)
		.await?
		.wait_for_in_block()
		.await?
		.wait_for_success()
		.await?;
	Ok(events)
}

fn find_and_cast_events<E: subxt::events::StaticEvent>(
	events: &ExtrinsicEvents<SubstrateConfig>,
	is_mandatory: bool,
) -> anyhow::Result<Vec<E>> {
	let mut desired_events = Vec::new();
	for event in events.iter() {
		let event = event?;
		if let Some(event) = event.as_event::<E>()? {
			desired_events.push(event)
		}
	}

	if is_mandatory && desired_events.is_empty() {
		Err(anyhow!("{}", Error::ExpectedEventNotEmitted))
	} else {
		Ok(desired_events)
	}
}

fn print_cosmwasm_events(events: &ExtrinsicEvents<SubstrateConfig>) -> anyhow::Result<()> {
	let events = find_and_cast_events::<Emitted>(events, false)?;
	for event in events {
		println!("- Event: {}", String::from_utf8_lossy(&event.ty));
		println!("\t- Contract: {}", event.contract);
		println!("\t- Attributes:");
		for (key, value) in event.attributes {
			println!(
				"\t\t- {}: {}",
				String::from_utf8_lossy(&key),
				String::from_utf8_lossy(&value)
			);
		}
	}
	Ok(())
}
