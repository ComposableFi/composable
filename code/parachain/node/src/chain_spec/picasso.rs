use common::{AccountId, AuraId, Balance};
use picasso_runtime::GenesisConfig;

use super::{Extensions, ParaId};

// The block number until ed25519-dalek should be used for signature verification. Decided at
// 1_788_000
pub const DALEK_END_BLOCK: u32 = 2_076_000;

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
			code: picasso_runtime::WASM_BINARY_V2
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
			key: Some(root.clone()),
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
		council_membership: Default::default(),
		council: Default::default(),
		democracy: Default::default(),
		treasury: Default::default(),
		technical_committee: Default::default(),
		technical_committee_membership: picasso_runtime::TechnicalCommitteeMembershipConfig {
			members : vec![root.clone()].try_into().expect("const"),
			phantom : Default::default(),
		},
		relayer_xcm: Default::default(),
		assets_registry: Default::default(),
		tokens: Default::default(),
		transaction_payment: Default::default(),
		vesting: Default::default(),
		ibc: picasso_runtime::IbcConfig {
			assets: vec![pallet_ibc::pallet::AssetConfig {
				id: primitives::currency::CurrencyId::PICA,
				denom: b"1".to_vec(),
			}],
		},
		release_membership: picasso_runtime::ReleaseMembershipConfig {
			members : vec![root].try_into().expect("const"),
			phantom : Default::default(),
		},
		release_committee :  Default::default(),
	}
}
