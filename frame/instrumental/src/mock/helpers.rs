use composable_traits::{instrumental::InstrumentalVaultConfig, vault::VaultConfig};
use frame_support::{
	assert_ok, sp_std::collections::btree_map::BTreeMap, traits::fungibles::Mutate,
};
use primitives::currency::CurrencyId;
use sp_runtime::Perquintill;

use super::{
	account_id::{AccountId, ADMIN},
	runtime::{Assets, Balance, Instrumental, Origin},
};

// ----------------------------------------------------------------------------------------------------
//                                    InstrumentalVaultConfigBuilder
// ----------------------------------------------------------------------------------------------------

pub struct InstrumentalVaultConfigBuilder {
	pub asset_id: CurrencyId,
	pub percent_deployable: Perquintill,
}

impl Default for InstrumentalVaultConfigBuilder {
	fn default() -> Self {
		InstrumentalVaultConfigBuilder {
			asset_id: CurrencyId::USDC,
			percent_deployable: Perquintill::zero(),
		}
	}
}

#[allow(dead_code)]
impl InstrumentalVaultConfigBuilder {
	pub fn build(self) -> InstrumentalVaultConfig<CurrencyId, Perquintill> {
		InstrumentalVaultConfig {
			asset_id: self.asset_id,
			percent_deployable: self.percent_deployable,
		}
	}

	pub fn asset_id(mut self, asset: CurrencyId) -> Self {
		self.asset_id = asset;
		self
	}

	pub fn percent_deployable(mut self, percent_deployable: Perquintill) -> Self {
		self.percent_deployable = percent_deployable;
		self
	}
}

// ----------------------------------------------------------------------------------------------------
//                                          VaultConfigBuilder
// ----------------------------------------------------------------------------------------------------

pub struct VaultConfigBuilder {
	pub asset_id: CurrencyId,
	pub manager: AccountId,
	pub reserved: Perquintill,
	pub strategies: BTreeMap<AccountId, Perquintill>,
}

impl Default for VaultConfigBuilder {
	fn default() -> Self {
		VaultConfigBuilder {
			asset_id: CurrencyId::USDC,
			manager: ADMIN,
			reserved: Perquintill::one(),
			strategies: BTreeMap::new(),
		}
	}
}

#[allow(dead_code)]
impl VaultConfigBuilder {
	fn asset_id(mut self, asset: CurrencyId) -> Self {
		self.asset_id = asset;
		self
	}

	fn reserved(mut self, reserved: Perquintill) -> Self {
		self.reserved = reserved;
		self
	}

	fn manager(mut self, manager: AccountId) -> Self {
		self.manager = manager;
		self
	}

	fn strategy(mut self, account: AccountId, strategy: Perquintill) -> Self {
		self.strategies.insert(account, strategy);
		self
	}

	fn build(self) -> VaultConfig<AccountId, CurrencyId> {
		VaultConfig {
			asset_id: self.asset_id,
			reserved: self.reserved,
			manager: self.manager,
			strategies: self.strategies,
		}
	}
}

// ----------------------------------------------------------------------------------------------------
//                                       InstrumentalVaultBuilder
// ----------------------------------------------------------------------------------------------------

pub struct InstrumentalVaultBuilder {
	pub configs: Vec<InstrumentalVaultConfig<CurrencyId, Perquintill>>,
}

#[allow(dead_code)]
impl InstrumentalVaultBuilder {
	fn new() -> Self {
		InstrumentalVaultBuilder { configs: Vec::new() }
	}

	fn add(mut self, config: InstrumentalVaultConfig<CurrencyId, Perquintill>) -> Self {
		self.configs.push(config);
		self
	}

	fn group_add(mut self, configs: Vec<InstrumentalVaultConfig<CurrencyId, Perquintill>>) -> Self {
		configs.into_iter().for_each(|config| {
			self.configs.push(config);
		});
		self
	}

	fn build(self) {
		// TODO: (Nevin)
		//  - remove duplicate assets
		self.configs.iter().for_each(|&config| {
			Instrumental::create(Origin::signed(ADMIN), config).ok();
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                     InstrumentalVaultInitializer
// ----------------------------------------------------------------------------------------------------

pub trait InstrumentalVaultInitializer {
	fn initialize_vault(self, config: InstrumentalVaultConfig<CurrencyId, Perquintill>) -> Self;
	fn initialize_vaults(
		self,
		configs: Vec<InstrumentalVaultConfig<CurrencyId, Perquintill>>,
	) -> Self;

	fn initialize_reserve(self, asset: CurrencyId, balance: Balance) -> Self;
	fn initialize_reserves(self, reserves: Vec<(CurrencyId, Balance)>) -> Self;

	fn initialize_vaults_with_reserves(
		self,
		configs: Vec<InstrumentalVaultConfig<CurrencyId, Perquintill>>,
		reserves: Vec<(CurrencyId, Balance)>,
	) -> Self;
}

impl InstrumentalVaultInitializer for sp_io::TestExternalities {
	fn initialize_vault(
		mut self,
		config: InstrumentalVaultConfig<CurrencyId, Perquintill>,
	) -> Self {
		self.execute_with(|| Instrumental::create(Origin::signed(ADMIN), config).ok());

		self
	}

	fn initialize_vaults(
		mut self,
		configs: Vec<InstrumentalVaultConfig<CurrencyId, Perquintill>>,
	) -> Self {
		self.execute_with(|| {
			configs.iter().for_each(|&config| {
				Instrumental::create(Origin::signed(ADMIN), config).ok();
			});
		});

		self
	}

	fn initialize_reserve(mut self, asset: CurrencyId, balance: Balance) -> Self {
		self.execute_with(|| {
			assert_ok!(<Assets as Mutate<AccountId>>::mint_into(asset, &ADMIN, balance));

			assert_ok!(Instrumental::add_liquidity(Origin::signed(ADMIN), asset, balance));
		});

		self
	}

	fn initialize_reserves(mut self, reserves: Vec<(CurrencyId, Balance)>) -> Self {
		self.execute_with(|| {
			reserves.iter().for_each(|&(asset, balance)| {
				assert_ok!(<Assets as Mutate<AccountId>>::mint_into(asset, &ADMIN, balance));

				assert_ok!(Instrumental::add_liquidity(Origin::signed(ADMIN), asset, balance));
			});
		});

		self
	}

	fn initialize_vaults_with_reserves(
		self,
		configs: Vec<InstrumentalVaultConfig<CurrencyId, Perquintill>>,
		reserves: Vec<(CurrencyId, Balance)>,
	) -> Self {
		self.initialize_vaults(configs).initialize_reserves(reserves)
	}

	// TODO: (Nevin)
	//  - set_block_number
}
