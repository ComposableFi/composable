use cumulus_primitives_core::ParaId;
use runtime_common::{AccountId, AuraId, Balance};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::MultiSigner;
pub mod picasso;

const PICASSO_ED: Balance = picasso_runtime::EXISTENTIAL_DEPOSIT;
/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
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

//TODO create prod config
pub fn picasso_dev(id: ParaId) -> picasso::ChainSpec {
	//TODO check properties
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "PIC".into());
	properties.insert("tokenDecimals".into(), 12.into());

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
					)
				],
                dev_accounts(),
                id,
				PICASSO_ED,
            )
        },
        vec![],
        None,
        None,
		Some(properties),
        Extensions {
			//TODO set to Ksm for picasso
            relay_chain: "picasso-local".into(), // You MUST set this to the correct network!
            para_id: id.into(),
        },
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
