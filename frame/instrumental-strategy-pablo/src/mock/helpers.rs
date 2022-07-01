use composable_traits::{
	defi::CurrencyPair,
	dex::Amm,
	instrumental::{Instrumental as InstrumentalTrait, InstrumentalVaultConfig},
};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use pallet_pablo::PoolInitConfiguration;
use primitives::currency::CurrencyId;
use sp_runtime::{Permill, Perquintill};

use super::runtime::{Instrumental, VaultId};
use crate::mock::{
	account_id::{AccountId, ALICE, BOB},
	runtime::{Balance, BlockNumber, Pablo, PoolId, Tokens},
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
	let fee = fee.into().unwrap_or_else(|| Permill::zero());
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
	assert_ok!(<Pablo as Amm>::add_liquidity(
		&ALICE,
		pool_id,
		base_amount,
		quote_amount,
		0_u128,
		true
	));
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
	let percent_deployable = percent_deployable.into().unwrap_or(Perquintill::zero());
	let config = InstrumentalVaultConfig { asset_id, percent_deployable };
	let vault_id = <Instrumental as InstrumentalTrait>::create(config);
	assert_ok!(vault_id);
	vault_id.unwrap()
}
