use composable_support::types::EthereumAddress;
use pallet_crowdloan_rewards::models::RemoteAccount;
use primitives::currency::CurrencyId;
use serde::{Deserialize, Serialize};
use sp_core::crypto::Ss58Codec;
use sp_runtime::AccountId32;
use ss58_registry::Ss58AddressFormatRegistry;
use std::collections::BTreeMap;
use substrate_api_client::{
	compose_call, compose_extrinsic, rpc::WsRpcClient, Api, AssetTipExtrinsicParams,
	GenericAddress, XtStatus,
};

use crate::common::api_wrap;

const REWARDS_PERMALINK: &str = "https://raw.githubusercontent.com/ComposableFi/composable/f887b58833f8a1ea82cdf0ffd93e3848174a5185/composablejs/packages/bootstrap_pallets/src/constants/rewards.json";
const REWARDS_DIGEST_SHA256: &str =
	"69e2fb1b8fbbeb0028c6c841c744a7c5744c06c4947a8673d8c9f6198ab458bb";
const REWARDS_VESTING_PERIOD_48_WEEKS: u64 = 29030400000;
// Submit 1000 rewards
const REWARDS_BATCH_POPULATE_SIZE: usize = 2000;
const REWARDS_TOTAL_EXPECTED: u128 = 3000000000000000009314;
const REWARDS_TOTAL_CONTRIBUTOR_EXPECTED: usize = 10685;
// Kusama
const RELAYCHAIN_ADDRESS_FORMAT: Ss58AddressFormatRegistry =
	Ss58AddressFormatRegistry::KusamaAccount;

type CanonicalContributorAccount = RemoteAccount<AccountId32>;
type CanonicalContributorAmount = u128;

type RawContributorAccount = String;
type RawContributorReward = String;

#[derive(Debug, Serialize, Deserialize)]
struct Rewards<K: Ord, V> {
	#[serde(flatten)]
	contributors: BTreeMap<K, V>,
}

#[derive(Debug)]
pub enum CrowdloanError {
	UnableToDownloadContributorFile,
	UnableToDecodeContributorFile,
	CannotDeserializeContributorFile,
	InvalidContributorAddress,
	InvalidContributorReward,
	InvalidContributorAddressPrefix,
	ApiFailure(substrate_api_client::ApiClientError),
}

impl From<substrate_api_client::ApiClientError> for CrowdloanError {
	fn from(x: substrate_api_client::ApiClientError) -> Self {
		Self::ApiFailure(x)
	}
}

fn reward_canonicalize(
	(account, reward): (RawContributorAccount, RawContributorReward),
) -> Result<(CanonicalContributorAccount, CanonicalContributorAmount), CrowdloanError> {
	let reward = (str::parse::<f64>(&reward)
		.map_err(|_| CrowdloanError::InvalidContributorReward)? *
		10f64.powi(CurrencyId::decimals() as i32)) as u128;
	match account.strip_prefix("0x") {
		Some(ethereum_address_src) => {
			let ethereum_address = TryInto::<[u8; 20]>::try_into(
				hex::decode(ethereum_address_src)
					.map_err(|_| CrowdloanError::InvalidContributorAddress)?,
			)
			.map_err(|_| CrowdloanError::InvalidContributorAddress)?;
			Ok((RemoteAccount::Ethereum(EthereumAddress(ethereum_address)), reward))
		},
		None => {
			let (relaychain_account, ss58_address_format) =
				Ss58Codec::from_ss58check_with_version(&account)
					.map_err(|_| CrowdloanError::InvalidContributorAddress)?;
			// Contributor are either eth or from ksm
			if ss58_address_format != RELAYCHAIN_ADDRESS_FORMAT.into() {
				Err(CrowdloanError::InvalidContributorAddressPrefix)
			} else {
				Ok((RemoteAccount::RelayChain(relaychain_account), reward))
			}
		},
	}
}

pub fn crowdloan_seed(
	api: Api<sp_core::sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>,
) -> Result<(), CrowdloanError> {
	// Download, verify and deserialize.
	let rewards_src = reqwest::blocking::get(REWARDS_PERMALINK)
		.map_err(|_| CrowdloanError::UnableToDownloadContributorFile)?
		.bytes()
		.map_err(|_| CrowdloanError::UnableToDecodeContributorFile)?;
	let rewards_digest = sha256::digest(rewards_src.as_ref());
	assert_eq!(REWARDS_DIGEST_SHA256, rewards_digest, "Invalid reward digest");
	let rewards = serde_json::from_slice::<Rewards<RawContributorAccount, RawContributorReward>>(
		&rewards_src,
	)
	.map_err(|_| CrowdloanError::CannotDeserializeContributorFile)?;

	// Canonicalize from string to strongly typed values.
	let mut canonicalized_rewards = rewards
		.contributors
		.into_iter()
		.map(reward_canonicalize)
		.collect::<Result<Vec<_>, _>>()?;

	// Verify
	let total_rewards =
		canonicalized_rewards.iter().map(|(_, r)| r).sum::<CanonicalContributorAmount>();
	assert_eq!(
		REWARDS_TOTAL_EXPECTED, total_rewards,
		"Unexpected total reward, should be approx 3B"
	);
	let total_contributors = canonicalized_rewards.len();
	assert_eq!(
		REWARDS_TOTAL_CONTRIBUTOR_EXPECTED, total_contributors,
		"Unexpected total contributors"
	);

	// Populate crowdloan, batching to avoid exceeding block weight.
	while canonicalized_rewards.len() > 0 {
		let batch_rewards = canonicalized_rewards
			.drain(0..usize::min(REWARDS_BATCH_POPULATE_SIZE, canonicalized_rewards.len()))
			.map(|(remote_account, reward)| {
				(remote_account, reward, REWARDS_VESTING_PERIOD_48_WEEKS)
			})
			.collect::<Vec<_>>();

		let batch_len = batch_rewards.len();

		let tx_hash = api_wrap::<_, CrowdloanError>(
			api.send_extrinsic(
				compose_extrinsic!(
					api.clone(),
					"Sudo",
					"sudo",
					compose_call!(
						api.metadata.clone(),
						"CrowdloanRewards",
						"populate",
						batch_rewards
					)
				)
				.hex_encode(),
				XtStatus::InBlock,
			),
		)?;

		log::info!(
			"Crowdloan populated with a batch of {} contribs, hash: {:?}",
			batch_len,
			tx_hash
		);
	}

	// Verify on chain data
	let onchain_total_contributors = api_wrap::<_, CrowdloanError>(api.get_storage_value::<u32>(
		"CrowdloanRewards",
		"TotalContributors",
		None,
	))?
	.unwrap_or(0);
	assert_eq!(
		REWARDS_TOTAL_CONTRIBUTOR_EXPECTED, onchain_total_contributors as usize,
		"Unexpected ONCHAIN total contributors"
	);

	let onchain_total_rewards = api_wrap::<_, CrowdloanError>(api.get_storage_value::<u128>(
		"CrowdloanRewards",
		"TotalRewards",
		None,
	))?
	.unwrap_or(0);
	assert_eq!(
		REWARDS_TOTAL_EXPECTED, onchain_total_rewards,
		"Unexpected ONCHAIN total reward, should be approx 3B"
	);

	// Now that crowdloan is populated, let's fund the pallet.
	let crowdloan_pallet_account = api_wrap::<_, CrowdloanError>(
		api.get_constant::<AccountId32>("CrowdloanRewards", "account_id"),
	)?;

	log::info!("Crowdloan pallet account: {}", crowdloan_pallet_account);

	let fund_account = |account, currency: CurrencyId, amount: u128| {
		api_wrap::<_, CrowdloanError>(
			api.send_extrinsic(
				compose_extrinsic!(
					api.clone(),
					"Sudo",
					"sudo",
					compose_call!(
						api.metadata.clone(),
						"Assets",
						"mint_into",
						currency,
						GenericAddress::Id(account),
						Compact(amount)
					)
				)
				.hex_encode(),
				XtStatus::InBlock,
			),
		)
	};

	let tx_hash = fund_account(crowdloan_pallet_account, CurrencyId::PICA, REWARDS_TOTAL_EXPECTED)?;

	log::info!("Crowdloan funded, tx hash: {:?}", tx_hash);

	Ok(())
}
