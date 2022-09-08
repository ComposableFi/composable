use composable_traits::{
	defi::CurrencyPair,
	dex::Amm,
	instrumental::{Instrumental as InstrumentalTrait, InstrumentalVaultConfig},
	vault::Vault as VaultTrait,
};
use frame_support::{assert_noop, assert_ok, traits::fungibles::Mutate, weights::GetDispatchInfo};
use frame_system::EventRecord;
use pallet_collective::{Error as CollectiveError, Instance1, MemberCount, ProposalIndex};
use pallet_pablo::PoolInitConfiguration;
use primitives::currency::CurrencyId;
use sp_core::{Encode, H256};
use sp_runtime::{
	traits::{BlakeTwo256, Hash},
	Permill, Perquintill,
};

use super::runtime::{
	Call, CollectiveInstrumental, Event, Instrumental, MockRuntime, Origin, System, Vault, VaultId,
};
use crate::{
	mock::{
		account_id::{AccountId, ALICE, BOB},
		runtime::{Balance, BlockNumber, Pablo, PoolId, Tokens},
	},
	Config,
};

pub fn create_pool<BAS, BAM, QAS, QAM, F, BW>(
	base_asset: BAS,
	base_amount: BAM,
	quote_asset: QAS,
	quote_amount: QAM,
	fee: F,
	base_weight: BW,
) -> PoolId
where
	BAS: Into<Option<CurrencyId>>,
	QAS: Into<Option<CurrencyId>>,
	BAM: Into<Option<Balance>>,
	QAM: Into<Option<Balance>>,
	F: Into<Option<Permill>>,
	BW: Into<Option<Permill>>,
{
	let default_amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
	let base_asset = base_asset.into().unwrap_or(CurrencyId::LAYR);
	let base_amount = base_amount.into().unwrap_or(default_amount);
	let quote_asset = quote_asset.into().unwrap_or(CurrencyId::CROWD_LOAN);
	let quote_amount = quote_amount.into().unwrap_or(default_amount);
	let fee = fee.into().unwrap_or_else(Permill::zero);
	let base_weight = base_weight.into().unwrap_or_else(|| Permill::from_percent(50));

	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));
	assert_ok!(Tokens::mint_into(base_asset, &BOB, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &BOB, quote_amount));

	let config = PoolInitConfiguration::<AccountId, CurrencyId, BlockNumber>::ConstantProduct {
		owner: ALICE,
		pair: CurrencyPair { base: base_asset, quote: quote_asset },
		fee,
		base_weight,
	};
	let pool_id = Pablo::do_create_pool(config);
	assert_ok!(pool_id);
	let pool_id = pool_id.unwrap();
	// assert_ok!(<Pablo as Amm>::add_liquidity(
	// 	&ALICE,
	// 	pool_id,
	// 	base_amount,
	// 	quote_amount,
	// 	0_u128,
	// 	true
	// ));
	assert_ok!(<Pablo as Amm>::add_liquidity(
		&BOB,
		pool_id,
		base_amount,
		quote_amount,
		0_u128,
		true
	));
	pool_id
}

pub fn create_vault<A, P>(asset_id: A, percent_deployable: P) -> VaultId
where
	A: Into<Option<CurrencyId>>,
	P: Into<Option<Perquintill>>,
{
	let asset_id = asset_id.into().unwrap_or(CurrencyId::LAYR);
	let percent_deployable = percent_deployable.into().unwrap_or_else(Perquintill::zero);
	let config = InstrumentalVaultConfig { asset_id, percent_deployable };
	let vault_id = <Instrumental as InstrumentalTrait>::create(config);
	assert_ok!(vault_id);
	vault_id.unwrap()
}

pub fn associate_vault_and_deposit_in_it<AMT>(vault_id: VaultId, asset_id: CurrencyId, amount: AMT)
where
	AMT: Into<Option<Balance>>,
{
	let default_amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
	let amount_to_mint = amount.into().unwrap_or(default_amount);
	let associate_vault_proposal = Call::PabloStrategy(crate::Call::associate_vault { vault_id });
	make_proposal(associate_vault_proposal, ALICE, 1, 0, None);
	let vault_account = Vault::account_id(&vault_id);
	assert_ok!(Tokens::mint_into(asset_id, &vault_account, amount_to_mint));
}

pub fn set_admin_members(members: Vec<AccountId>, members_count: MemberCount) {
	assert_ok!(CollectiveInstrumental::set_members(Origin::root(), members, None, members_count,));
}

pub fn make_proposal(
	proposal: Call,
	account_id: AccountId,
	threshold: u32,
	index: ProposalIndex,
	yes_votes: Option<&[AccountId]>,
) {
	let proposal_len: u32 = proposal.using_encoded(|p| p.len() as u32);
	let proposal_weight = proposal.get_dispatch_info().weight;
	let hash: H256 = BlakeTwo256::hash_of(&proposal);
	assert_ok!(CollectiveInstrumental::propose(
		Origin::signed(account_id),
		threshold,
		Box::new(proposal),
		proposal_len
	));
	if threshold > 1 {
		if let Some(votes) = yes_votes {
			votes.iter().for_each(|account| {
				assert_ok!(CollectiveInstrumental::vote(
					Origin::signed(*account),
					hash,
					index,
					true
				));
			});
			if (votes.len() as u32) < threshold {
				assert_noop!(
					CollectiveInstrumental::close(
						Origin::signed(account_id),
						hash,
						index,
						proposal_weight,
						proposal_len
					),
					CollectiveError::<MockRuntime, Instance1>::TooEarly
				);
			} else {
				assert_ok!(CollectiveInstrumental::close(
					Origin::signed(account_id),
					hash,
					index,
					proposal_weight,
					proposal_len
				));
			}
		};
	}
}

pub fn set_pool_id_for_asset(asset_id: CurrencyId, pool_id: PoolId) {
	let set_pool_id_for_asset_proposal =
		Call::PabloStrategy(crate::Call::set_pool_id_for_asset { asset_id, pool_id });
	make_proposal(set_pool_id_for_asset_proposal, ALICE, 1, 0, None);
}

pub fn assert_has_event<T, F>(matcher: F)
where
	T: Config,
	F: Fn(&EventRecord<Event, H256>) -> bool,
{
	assert!(System::events().iter().any(matcher));
}
