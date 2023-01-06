use super::{
    cosmwasm,
    subxt_api::api::{
        self,
        cosmwasm::events,
        runtime_types::{
            pallet_cosmwasm::pallet::CodeIdentifier,
            primitives::currency::CurrencyId,
            sp_runtime::bounded::{bounded_btree_map::BoundedBTreeMap, bounded_vec::BoundedVec},
        },
    },
    types::{
        cosmwasm::{
            AdminUpdated, Emitted, Extrinsic, ExtrinsicExecuted, Instantiated, Migrated, Uploaded,
        },
        PrettyDisplay,
    },
    OutputType,
};
use clap::{Args, Subcommand};
use serde::Serialize;
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
    pub async fn run<P: Pair>(
        self,
        pair: P,
        chain_endpoint: String,
        output_type: OutputType,
    ) -> anyhow::Result<()>
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
                print_events::<events::Uploaded, Uploaded>(&events, output_type)?;
                Ok(())
            }
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
                print_events::<events::Instantiated, Instantiated>(&events, output_type)?;
                Ok(())
            }
            Subcommands::Execute(cosmwasm::Execute {
                contract,
                funds,
                gas,
                message,
            }) => {
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
                print_events::<events::Executed, ()>(&events, output_type)?;
                Ok(())
            }
            Subcommands::Migrate(cosmwasm::Migrate {
                contract,
                new_code_id,
                gas,
                message,
            }) => {
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
                print_events::<events::Migrated, Migrated>(&events, output_type)?;
                Ok(())
            }
            Subcommands::UpdateAdmin(cosmwasm::UpdateAdmin {
                contract,
                new_admin,
                gas,
                ..
            }) => {
                let events = do_signed_transaction(
                    chain_endpoint,
                    pair,
                    api::tx().cosmwasm().update_admin(contract, new_admin, gas),
                )
                .await?;
                print_events::<events::AdminUpdated, AdminUpdated>(&events, output_type)?;
                Ok(())
            }
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

fn print_events<E, CE>(
    events: &ExtrinsicEvents<SubstrateConfig>,
    output_type: OutputType,
) -> anyhow::Result<()>
where
    E: subxt::events::StaticEvent,
    CE: PrettyDisplay + Serialize + From<E>,
{
    let mut details = None;
    let mut cosmwasm_events = Vec::new();
    let mut data = None;
    for event in events.iter() {
        let event = event?;

        if let Some(event) = event.as_event::<events::Executed>()? {
            data = event.data;
        } else if let Some(event) = event.as_event::<E>()? {
            details = Some(Into::<CE>::into(event));
        } else if let Some(event) = event.as_event::<events::Emitted>()? {
            cosmwasm_events.push(Into::<Emitted>::into(event));
        }
    }

    let executed = ExtrinsicExecuted {
        extrinsic: Extrinsic {
            name: E::EVENT.into(),
            details,
            data,
        },
        cosmwasm_events,
    };

    match output_type {
        OutputType::Text => executed.pretty_display(0),
        OutputType::Json => println!("{}", serde_json::to_string_pretty(&executed)?),
    }
    Ok(())
}
