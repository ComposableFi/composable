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

pub fn create_pool_with(assets: CurrencyPair<CurrencyId>, amounts: Vec<Balance>) -> PoolId {
	create_pool(assets, amounts, Permill::zero(), Permill::from_percent(50))
}

fn create_pool(
	assets: CurrencyPair<CurrencyId>,
	amounts: Vec<Balance>,
	fee: Permill,
	base_weight: Permill,
) -> PoolId {
	let base = assets.base;
	let quote = assets.quote;
	assert_ok!(Tokens::mint_into(base, &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(base, &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &BOB, amounts[1]));

	let config = PoolInitConfiguration::<AccountId, CurrencyId, BlockNumber>::ConstantProduct {
		owner: ALICE,
		pair: assets,
		fee,
		base_weight,
	};
	let pool_id = Pablo::do_create_pool(config);
	assert_ok!(pool_id);
	let pool_id = pool_id.unwrap();
	assert_ok!(<Pablo as Amm>::add_liquidity(
		&ALICE, pool_id, amounts[0], amounts[1], 0_u128, true
	));
	assert_ok!(<Pablo as Amm>::add_liquidity(&BOB, pool_id, amounts[0], amounts[1], 0_u128, true));
	pool_id
}

pub fn create_vault<A, P>(asset_id: A, percent_deployable: P) -> VaultId
where
	A: Into<Option<CurrencyId>>,
	P: Into<Option<Perquintill>>,
{
	let asset_id = asset_id.into().unwrap_or_default();
	let percent_deployable = percent_deployable.into().unwrap_or_default();
	let config = InstrumentalVaultConfig { asset_id, percent_deployable };
	let vault_id = <Instrumental as InstrumentalTrait>::create(config);
	assert_ok!(vault_id);
	vault_id.unwrap()
}
