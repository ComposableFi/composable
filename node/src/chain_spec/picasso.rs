use picasso_runtime::{self as parachain_runtime, AccountId, GenesisConfig};
use super::{AuraId, Extensions, ParaId};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generates the genesis config for picasso
pub fn genesis_config(
	root: AccountId,
	authorities: Vec<AuraId>,
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
			balances: accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		},
		aura: parachain_runtime::AuraConfig {
			authorities: authorities.clone(),
		},
		sudo: parachain_runtime::SudoConfig {
			// Assign network admin rights.
			key: root,
		},
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
