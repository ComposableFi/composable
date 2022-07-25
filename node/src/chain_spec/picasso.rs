use common::{AccountId, AuraId, Balance};
use picasso_runtime::GenesisConfig;

use super::{Extensions, ParaId};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn picasso_session_keys(keys: AuraId) -> picasso_runtime::opaque::SessionKeys {
	picasso_runtime::opaque::SessionKeys { aura: keys }
}
/// Generates the genesis config for picasso
pub fn genesis_config(
	root: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	accounts: Vec<AccountId>,
	id: ParaId,
	existential_deposit: Balance,
	treasury: AccountId,
) -> picasso_runtime::GenesisConfig {
	picasso_runtime::GenesisConfig {
		system: picasso_runtime::SystemConfig {
			code: picasso_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: picasso_runtime::BalancesConfig {
			// Configure endowed accounts with initial balance.
			balances: vec![
				vec![(treasury, existential_deposit)],
				accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
			]
			.concat(),
		},
		aura: Default::default(),
		sudo: picasso_runtime::SudoConfig {
			// Assign network admin rights.
			key: Some(root),
		},
		indices: picasso_runtime::IndicesConfig { indices: vec![] },
		parachain_info: picasso_runtime::ParachainInfoConfig { parachain_id: id },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		session: picasso_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),                // account id
						acc,                        // validator id
						picasso_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		collator_selection: picasso_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: existential_deposit * 16,
			..Default::default()
		},
		council_membership: picasso_runtime::CouncilMembershipConfig {
			members: vec![],
			..Default::default()
		},
		// council will get its members from council_membership
		council: Default::default(),
		democracy: Default::default(),
		treasury: Default::default(),
	}
}
