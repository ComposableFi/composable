use super::subxt_api;
use serde::Serialize;
use std::{collections::BTreeMap, string::ToString};
use subxt::ext::sp_core::crypto::Ss58Codec;

pub trait PrettyDisplay {
    fn pretty_display(&self, indentation_level: usize);
}

pub mod cosmwasm {
    use super::subxt_api::api::cosmwasm::events;
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Extrinsic<T: Serialize> {
        pub name: String,
        pub details: Option<T>,
        pub data: Option<Vec<u8>>,
    }

    #[derive(Debug, Serialize)]
    pub struct ExtrinsicExecuted<T: Serialize + PrettyDisplay> {
        pub extrinsic: Extrinsic<T>,
        pub cosmwasm_events: Vec<Emitted>,
    }

    impl<T: Serialize + PrettyDisplay> PrettyDisplay for ExtrinsicExecuted<T> {
        fn pretty_display(&self, indentation_level: usize) {
            let indent = "\t".repeat(indentation_level);
            println!("{indent}[ + ] {}", self.extrinsic.name);
            if let Some(details) = &self.extrinsic.details {
                details.pretty_display(indentation_level + 1);
            }
            self.cosmwasm_events
                .iter()
                .for_each(|e| e.pretty_display(indentation_level + 1));
            if let Some(data) = &self.extrinsic.data {
                println!("{indent}\t- Data: 0x{}", hex::encode(data));
            }
        }
    }

    impl PrettyDisplay for () {
        fn pretty_display(&self, _: usize) {}
    }

    impl From<events::Executed> for () {
        fn from(_: events::Executed) -> Self {}
    }

    #[derive(Debug, Serialize)]
    pub struct Uploaded {
        pub code_hash: String,
        pub code_id: u64,
    }

    impl From<events::Uploaded> for Uploaded {
        fn from(uploaded: events::Uploaded) -> Self {
            Self {
                code_hash: format!("{}", uploaded.code_hash),
                code_id: uploaded.code_id,
            }
        }
    }

    impl PrettyDisplay for Uploaded {
        fn pretty_display(&self, indentation_level: usize) {
            let indent = "\t".repeat(indentation_level);
            println!("{indent}- Code Hash: {}", self.code_hash);
            println!("{indent}- Code ID: {}", self.code_id);
        }
    }

    #[derive(Debug, Serialize)]
    pub struct Instantiated {
        pub contract_addr: String,
    }

    impl From<events::Instantiated> for Instantiated {
        fn from(instantiated: events::Instantiated) -> Self {
            Self {
                contract_addr: instantiated.contract.to_ss58check(),
            }
        }
    }

    impl PrettyDisplay for Instantiated {
        fn pretty_display(&self, indentation_level: usize) {
            let indent = "\t".repeat(indentation_level);
            println!("{indent}- Contract address: {}", self.contract_addr);
        }
    }

    #[derive(Debug, Serialize)]
    pub struct Migrated {
        pub contract: String,
        pub to: u64,
    }

    impl From<events::Migrated> for Migrated {
        fn from(migrated: events::Migrated) -> Self {
            Self {
                contract: migrated.contract.to_string(),
                to: migrated.to,
            }
        }
    }

    impl PrettyDisplay for Migrated {
        fn pretty_display(&self, indentation_level: usize) {
            let indent = "\t".repeat(indentation_level);
            println!(
                "{indent}- Contract \"{}\" is migrated to Code ID \"{}\"",
                self.contract, self.to
            );
        }
    }

    #[derive(Debug, Serialize)]
    pub struct AdminUpdated {
        pub contract: String,
        pub new_admin: Option<String>,
    }

    impl From<events::AdminUpdated> for AdminUpdated {
        fn from(updated: events::AdminUpdated) -> Self {
            Self {
                contract: updated.contract.to_string(),
                new_admin: updated.new_admin.map(|a| a.to_string()),
            }
        }
    }

    impl PrettyDisplay for AdminUpdated {
        fn pretty_display(&self, indentation_level: usize) {
            println!(
                "{}- Contract \"{}\"'s admin is updated to \"{:?}\"",
                "\t".repeat(indentation_level),
                self.contract,
                self.new_admin
            );
        }
    }

    #[derive(Debug, Serialize)]
    pub struct Emitted {
        pub contract: String,
        pub ty: String,
        pub attributes: BTreeMap<String, String>,
    }

    impl From<events::Emitted> for Emitted {
        fn from(emitted: events::Emitted) -> Self {
            Self {
                contract: emitted.contract.to_ss58check(),
                ty: String::from_utf8_lossy(&emitted.ty).to_string(),
                attributes: emitted
                    .attributes
                    .iter()
                    .map(|(k, v)| {
                        (
                            String::from_utf8_lossy(k).to_string(),
                            String::from_utf8_lossy(v).to_string(),
                        )
                    })
                    .collect(),
            }
        }
    }

    impl PrettyDisplay for Emitted {
        fn pretty_display(&self, indentation_level: usize) {
            let indent = "\t".repeat(indentation_level - 1);
            println!("{indent}- Event: {}", self.ty);
            println!("{indent}\t- Contract: {}", self.contract);
            println!("{indent}\t- Attributes:");
            self.attributes
                .iter()
                .for_each(|(k, v)| println!("{indent}\t\t- {}: {}", k, v));
        }
    }
}
