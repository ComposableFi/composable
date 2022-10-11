#![allow(clippy::derive_partial_eq_without_eq)] // for ChainSpecGroup
use common::{AccountId, AuraId};
use cumulus_primitives_core::ParaId;
use once_cell::sync::Lazy;
use primitives::currency::CurrencyId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
#[cfg(feature = "composable")]
pub mod composable;

#[cfg(feature = "dali")]
pub mod dali;

pub mod picasso;

const DEFAULT_PARA_ID: u32 = 2087;

// Parachain ID.
static PARA_ID: Lazy<ParaId> = Lazy::new(|| {
	ParaId::new(
		std::env::var("COMPOSABLE_PARA_ID")
			.unwrap_or_else(|_| DEFAULT_PARA_ID.to_string())
			.parse::<u32>()
			.unwrap_or(DEFAULT_PARA_ID),
	)
});

/// The extensions for the [`ChainSpec`].
#[derive(
	Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension,
)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Generate a crypto pair from seed.
pub fn from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	from_seed::<AuraId>(seed)
}

/// Generate an account ID from seed.
pub fn account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	MultiSigner: From<<TPublic::Pair as Pair>::Public>,
{
	MultiSigner::from(from_seed::<TPublic>(seed)).into_account()
}

#[cfg(feature = "composable")]
/// Composable (Westend parachain)
pub fn composable_westend() -> composable::ChainSpec {
	composable::ChainSpec::from_json_bytes(include_bytes!("res/composable-westend.json").to_vec())
		.expect("composable-westend chain spec not found!")
}

#[cfg(feature = "dali")]
/// Dali (Rococo parachain)
pub fn dali_rococo() -> dali::ChainSpec {
	dali::ChainSpec::from_json_bytes(include_bytes!("./res/dali-rococo.json").to_vec())
		.expect("Dali chain spec not found!")
}

/// Picasso (Kusama parachain)
pub fn picasso() -> picasso::ChainSpec {
	picasso::ChainSpec::from_json_bytes(include_bytes!("./res/picasso.json").to_vec())
		.expect("Picasso chain spec not found!")
}

#[cfg(feature = "composable")]
/// Composable (Polkadot parachain)
pub fn composable() -> composable::ChainSpec {
	composable::ChainSpec::from_json_bytes(include_bytes!("./res/composable.json").to_vec())
		.expect("Picasso chain spec not found!")
}

// chain spec for single node environments
pub fn picasso_dev() -> picasso::ChainSpec {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "PICA".into());
	properties.insert("tokenDecimals".into(), CurrencyId::decimals().into());
	properties.insert("ss58Format".into(), picasso_runtime::SS58Prefix::get().into());

	picasso::ChainSpec::from_genesis(
		"Local Picasso Testnet",
		"picasso",
		ChainType::Development,
		move || {
			picasso::genesis_config(
				account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
					(
						account_id_from_seed::<sr25519::Public>("Charlie"),
						get_collator_keys_from_seed("Charlie"),
					),
				],
				dev_accounts(),
				*PARA_ID,
				common::NativeExistentialDeposit::get(),
				picasso_runtime::governance::TreasuryAccount::get(),
			)
		},
		vec![],
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "rococo_local_testnet".into(), para_id: u32::from(*PARA_ID) },
	)
}

#[cfg(feature = "dali")]
// chain spec for local testnet environments
pub fn dali_dev(id: &str) -> dali::ChainSpec {
	let para_id = match id.split('-').nth(1).expect("Dali chainspec 'dali-dev' has a dash; qed") {
		"dev" => None,
		number => number.parse::<u32>().ok().map(ParaId::from),
	};
	let para_id = para_id.unwrap_or(*PARA_ID);
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "DALI".into());
	properties.insert("tokenDecimals".into(), CurrencyId::decimals().into());
	properties.insert("ss58Format".into(), dali_runtime::SS58Prefix::get().into());

	dali::ChainSpec::from_genesis(
		"Local Dali Testnet",
		"dali",
		ChainType::Development,
		move || {
			dali::genesis_config(
				account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
					(
						account_id_from_seed::<sr25519::Public>("Charlie"),
						get_collator_keys_from_seed("Charlie"),
					),
				],
				dev_accounts(),
				para_id,
				common::NativeExistentialDeposit::get(),
				dali_runtime::TreasuryAccount::get(),
			)
		},
		vec![],
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "rococo_local_testnet".into(), para_id: u32::from(para_id) },
	)
}

#[cfg(feature = "composable")]
// chain spec for single node environments
pub fn composable_dev() -> composable::ChainSpec {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "LAYR".into());
	properties.insert("tokenDecimals".into(), CurrencyId::decimals().into());
	properties.insert("ss58Format".into(), composable_runtime::SS58Prefix::get().into());

	composable::ChainSpec::from_genesis(
		"Local Composable Testnet",
		"composable",
		ChainType::Development,
		move || {
			composable::genesis_config(
				account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
					(
						account_id_from_seed::<sr25519::Public>("Charlie"),
						get_collator_keys_from_seed("Charlie"),
					),
				],
				dev_accounts(),
				*PARA_ID,
				composable_runtime::ExistentialDeposit::get(),
				composable_runtime::TreasuryAccount::get(),
			)
		},
		vec![],
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "westend_local_testnet".into(), para_id: u32::from(*PARA_ID) },
	)
}

/// Common dev accounts
pub fn dev_accounts() -> Vec<AccountId> {
	vec![
		account_id_from_seed::<sr25519::Public>("Alice"),
		account_id_from_seed::<sr25519::Public>("Bob"),
		account_id_from_seed::<sr25519::Public>("Charlie"),
		account_id_from_seed::<sr25519::Public>("Dave"),
		account_id_from_seed::<sr25519::Public>("Eve"),
		account_id_from_seed::<sr25519::Public>("Ferdie"),
		account_id_from_seed::<sr25519::Public>("Alice//stash"),
		account_id_from_seed::<sr25519::Public>("Bob//stash"),
		account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		account_id_from_seed::<sr25519::Public>("Dave//stash"),
		account_id_from_seed::<sr25519::Public>("Eve//stash"),
		account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	]
}
