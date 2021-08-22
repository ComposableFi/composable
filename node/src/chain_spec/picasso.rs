use picasso_runtime::{self as parachain_runtime, AccountId, GenesisConfig, Balance};

use super::{AuraId, Extensions, ParaId};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
//TODO un hardcode pull proper ED from config
const PICASSO_ED: Balance = 500u128;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn picasso_session_keys(keys: AuraId) -> parachain_runtime::opaque::SessionKeys {
	parachain_runtime::opaque::SessionKeys { aura: keys }
}
/// Generates the genesis config for picasso
pub fn genesis_config(
    root: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
    accounts: Vec<AccountId>,
    id: ParaId,
) -> parachain_runtime::GenesisConfig {
    parachain_runtime::GenesisConfig {
        system: parachain_runtime::SystemConfig {
            code: parachain_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: parachain_runtime::BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
        },
        aura: Default::default(),
        sudo: parachain_runtime::SudoConfig {
            // Assign network admin rights.
            key: root,
        },
		indices: parachain_runtime::IndicesConfig {
			indices: vec![],
		},
        parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
        aura_ext: Default::default(),
        parachain_system: Default::default(),
		session: picasso_runtime::SessionConfig {
			keys: invulnerables.iter().cloned().map(|(acc, aura)| (
				acc.clone(), // account id
				acc.clone(), // validator id
				picasso_session_keys(aura), // session keys
			)).collect()
		},
		collator_selection: parachain_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			//TODO split for each chain due to different ED
			candidacy_bond: PICASSO_ED * 16,
			..Default::default()
		},
    }
}
