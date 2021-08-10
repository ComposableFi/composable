use sp_core::{sr25519, Pair, Public};

use picasso_runtime::AccountId;
use sc_service::ChainType;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::MultiSigner;

pub mod picasso;

/// Generate a crypto pair from seed.
pub fn from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate an account ID from seed.
pub fn account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    MultiSigner: From<<TPublic::Pair as Pair>::Public>,
{
    MultiSigner::from(from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (from_seed::<AuraId>(s), from_seed::<GrandpaId>(s))
}

pub fn picasso_dev() -> picasso::ChainSpec {
    picasso::ChainSpec::from_genesis(
        // Name
        "Local Picasso Testnet",
        // ID
        "picasso",
        ChainType::Development,
        move || {
            picasso::genesis_config(
	            // Sudo account
	            account_id_from_seed::<sr25519::Public>("Alice"),
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Pre-funded accounts
                dev_accounts(),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
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
