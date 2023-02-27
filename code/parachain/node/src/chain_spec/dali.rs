use common::{AccountId, AuraId, Balance};
use dali_runtime::GenesisConfig;

use super::{Extensions, ParaId};
use pallet_ibc::pallet::AssetConfig;
use primitives::currency::CurrencyId;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn dali_session_keys(keys: AuraId) -> dali_runtime::opaque::SessionKeys {
	dali_runtime::opaque::SessionKeys { aura: keys }
}
/// Generates the genesis config for dali
pub fn genesis_config(
	root: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	accounts: Vec<AccountId>,
	id: ParaId,
	existential_deposit: Balance,
	treasury: AccountId,
) -> dali_runtime::GenesisConfig {
	dali_runtime::GenesisConfig {
		system: dali_runtime::SystemConfig {
			code: dali_runtime::WASM_BINARY_V2
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: dali_runtime::BalancesConfig {
			// Configure endowed accounts with initial balance.
			balances: vec![
				vec![(treasury, existential_deposit)],
				accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
			]
			.concat(),
		},
		aura: Default::default(),
		sudo: dali_runtime::SudoConfig {
			// Assign network admin rights.
			key: Some(root),
		},
		indices: dali_runtime::IndicesConfig { indices: vec![] },
		parachain_info: dali_runtime::ParachainInfoConfig { parachain_id: id },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		session: dali_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),             // account id
						acc,                     // validator id
						dali_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		collator_selection: dali_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: existential_deposit * 16,
			..Default::default()
		},
		council_membership: Default::default(),
		// council will get its members from council_membership
		council: Default::default(),
		democracy: Default::default(),
		treasury: Default::default(),
		technical_committee: Default::default(),
		technical_committee_membership: Default::default(),
		relayer_xcm: Default::default(),
		assets_registry: Default::default(),
		tokens: Default::default(),
		transaction_payment: Default::default(),
		vesting: Default::default(),
		lending: Default::default(),
		liquidations: Default::default(),
		ibc: dali_runtime::IbcConfig {
			assets: vec![AssetConfig { id: CurrencyId::from(1), denom: b"1".to_vec() }],
		},
	}
}
