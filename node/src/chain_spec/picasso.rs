use picasso_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, IndicesConfig,
	SudoConfig, SystemConfig, WASM_BINARY,
};
use super::{AuraId, GrandpaId};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generates the genesis config for picasso
pub fn genesis_config(
	root: AccountId,
	authorities: Vec<(AuraId, GrandpaId)>,
	accounts: Vec<AccountId>,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: WASM_BINARY.unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		indices: IndicesConfig {
			indices: vec![],
		},
		grandpa: GrandpaConfig {
			authorities: authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root,
		},
	}
}
