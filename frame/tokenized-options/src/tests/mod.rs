#![allow(clippy::disallowed_methods, clippy::identity_op)]

use crate::{
	mocks::{
		accounts::*,
		assets::*,
		runtime::{
			get_oracle_price, set_oracle_price, Assets, Balance, Moment, OptionId, Origin, System,
			Timestamp, TokenizedOptions, Vault, VaultId,
		},
	},
	pallet::{AssetToVault, Error, OptionIdToOption},
	types::*,
};
use composable_traits::{
	tokenized_options::TokenizedOptions as TokenizedOptionsTrait,
	vault::{Vault as VaultTrait, VaultConfig},
};
use frame_system::ensure_signed;

use frame_support::{
	assert_ok,
	traits::{fungibles::Mutate, Hooks},
};
use itertools::Itertools;
use proptest::{prelude::*, strategy::Strategy};
use sp_runtime::{DispatchError, Perquintill};
use std::{collections::BTreeMap, ops::Range};

mod block_producer;
mod buy_option;
mod create_option;
mod create_vault;
mod delete_sell_option;
mod epoch_tests;
mod exercise_option;
mod sell_option;
mod settle_options;
mod time_management;
mod total_test;
mod withdraw_collateral;

pub const UNIT: u128 = 10u128.pow(12);

// ----------------------------------------------------------------------------------------------------
//		VaultConfigBuilder
// ----------------------------------------------------------------------------------------------------
struct VaultConfigBuilder {
	pub asset_id: AssetId,
	pub manager: AccountId,
	pub reserved: Perquintill,
	pub strategies: BTreeMap<AccountId, Perquintill>,
}

impl Default for VaultConfigBuilder {
	fn default() -> Self {
		VaultConfigBuilder {
			asset_id: BTC,
			manager: ADMIN,
			reserved: Perquintill::one(),
			strategies: BTreeMap::new(),
		}
	}
}

impl VaultConfigBuilder {
	fn build(self) -> VaultConfig<AccountId, AssetId> {
		VaultConfig {
			asset_id: self.asset_id,
			manager: self.manager,
			reserved: self.reserved,
			strategies: self.strategies,
		}
	}

	fn asset_id(mut self, asset: AssetId) -> Self {
		self.asset_id = asset;
		self
	}
}

// ----------------------------------------------------------------------------------------------------
//		VaultInitializer
// ----------------------------------------------------------------------------------------------------
pub trait VaultInitializer {
	fn initialize_vaults(self, configs: Vec<VaultConfig<AccountId, AssetId>>) -> Self;

	fn initialize_all_vaults(self) -> Self;

	fn initialize_oracle_prices(self) -> Self;

	fn initialize_deposits(self, deposits: Vec<(AssetId, Balance)>) -> Self;

	fn initialize_vaults_with_deposits(
		self,
		vault_configs: Vec<VaultConfig<AccountId, AssetId>>,
		deposits: Vec<(AssetId, Balance)>,
	) -> Self;
}

impl VaultInitializer for sp_io::TestExternalities {
	fn initialize_oracle_prices(mut self) -> Self {
		let assets_prices: Vec<(AssetId, Balance)> = Vec::from([
			(USDC, 1 * UNIT),
			(BTC, 50_000 * UNIT),
			(DOT, 100 * UNIT),
			(PICA, 1 * 10u128.pow(9)),  // 0.001$ to test decimals interactions
			(LAYR, 1 * 10u128.pow(11)), // 0.1$ to test decimals interactions
		]);

		self.execute_with(|| {
			assets_prices.iter().for_each(|&(asset, price)| {
				set_oracle_price(asset, price);
			});
		});

		self
	}

	fn initialize_all_vaults(mut self) -> Self {
		let assets = Vec::from(ASSETS_WITH_USDC);

		self.execute_with(|| {
			assets.iter().for_each(|&asset| {
				let config = VaultConfigBuilder::default().asset_id(asset).build();
				TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), config).ok();
			});
		});

		self
	}

	fn initialize_vaults(mut self, vault_configs: Vec<VaultConfig<AccountId, AssetId>>) -> Self {
		self.execute_with(|| {
			vault_configs.iter().for_each(|config| {
				TokenizedOptions::create_asset_vault(Origin::signed(ADMIN), config.clone()).ok();
			});
		});

		self
	}

	fn initialize_deposits(mut self, deposits: Vec<(AssetId, Balance)>) -> Self {
		self.execute_with(|| {
			deposits.iter().for_each(|&(asset, balance)| {
				assert_ok!(<Assets as Mutate<AccountId>>::mint_into(asset, &ADMIN, balance));

				let vault_id: VaultId = Vault::token_vault(asset).unwrap();

				assert_ok!(Vault::deposit(Origin::signed(ADMIN), vault_id, balance));
			});
		});

		self
	}

	fn initialize_vaults_with_deposits(
		self,
		vault_configs: Vec<VaultConfig<AccountId, AssetId>>,
		deposits: Vec<(AssetId, Balance)>,
	) -> Self {
		self.initialize_vaults(vault_configs).initialize_deposits(deposits)
	}
}

// ----------------------------------------------------------------------------------------------------
//		OptionsConfigBuilder
// ----------------------------------------------------------------------------------------------------
struct OptionsConfigBuilder {
	pub base_asset_id: AssetId,
	pub quote_asset_id: AssetId,
	pub base_asset_strike_price: Balance,
	pub quote_asset_strike_price: Balance,
	pub option_type: OptionType,
	pub exercise_type: ExerciseType,
	pub expiring_date: Moment,
	pub epoch: Epoch<Moment>,
	pub status: Status,
	pub base_asset_amount_per_option: Balance,
	pub quote_asset_amount_per_option: Balance,
	pub total_issuance_seller: Balance,
	pub total_premium_paid: Balance,
	pub exercise_amount: Balance,
	pub base_asset_spot_price: Balance,
	pub total_issuance_buyer: Balance,
	pub total_shares_amount: Balance,
}

impl Default for OptionsConfigBuilder {
	fn default() -> Self {
		OptionsConfigBuilder {
			base_asset_id: BTC,
			quote_asset_id: USDC,
			base_asset_strike_price: 50000u128 * UNIT,
			quote_asset_strike_price: 1u128 * UNIT,
			option_type: OptionType::Call,
			exercise_type: ExerciseType::European,
			expiring_date: 6000u64,
			// Use this when https://github.com/paritytech/substrate/pull/10128 is merged
			// epoch: Epoch { deposit: 0u64, purchase: 3000u64, exercise: 6000u64, end: 9000u64 },
			epoch: Epoch { deposit: 0u64, purchase: 2000u64, exercise: 5000u64, end: 9000u64 },
			status: Status::NotStarted,
			base_asset_amount_per_option: 1u128 * UNIT,
			quote_asset_amount_per_option: 1u128 * UNIT,
			total_issuance_seller: 0u128,
			total_premium_paid: 0u128,
			exercise_amount: 0u128,
			base_asset_spot_price: 0u128,
			total_issuance_buyer: 0u128,
			total_shares_amount: 0u128,
		}
	}
}

impl OptionsConfigBuilder {
	fn build(self) -> OptionConfig<AssetId, Balance, Moment> {
		OptionConfig {
			base_asset_id: self.base_asset_id,
			quote_asset_id: self.quote_asset_id,
			base_asset_strike_price: self.base_asset_strike_price,
			quote_asset_strike_price: self.quote_asset_strike_price,
			option_type: self.option_type,
			exercise_type: self.exercise_type,
			expiring_date: self.expiring_date,
			epoch: self.epoch,
			status: self.status,
			base_asset_amount_per_option: self.base_asset_amount_per_option,
			quote_asset_amount_per_option: self.quote_asset_amount_per_option,
			total_issuance_seller: self.total_issuance_seller,
			total_premium_paid: self.total_premium_paid,
			exercise_amount: self.exercise_amount,
			base_asset_spot_price: self.base_asset_spot_price,
			total_issuance_buyer: self.total_issuance_buyer,
			total_shares_amount: self.total_shares_amount,
		}
	}

	fn base_asset_id(mut self, base_asset_id: AssetId) -> Self {
		self.base_asset_id = base_asset_id;
		self
	}

	fn quote_asset_id(mut self, quote_asset_id: AssetId) -> Self {
		self.quote_asset_id = quote_asset_id;
		self
	}

	fn base_asset_strike_price(mut self, base_asset_strike_price: Balance) -> Self {
		self.base_asset_strike_price = base_asset_strike_price;
		self
	}

	fn quote_asset_strike_price(mut self, quote_asset_strike_price: Balance) -> Self {
		self.quote_asset_strike_price = quote_asset_strike_price;
		self
	}

	fn option_type(mut self, option_type: OptionType) -> Self {
		self.option_type = option_type;
		self
	}

	fn expiring_date(mut self, expiring_date: Moment) -> Self {
		self.expiring_date = expiring_date;
		self
	}

	fn exercise_type(mut self, exercise_type: ExerciseType) -> Self {
		self.exercise_type = exercise_type;
		self
	}

	fn total_issuance_seller(mut self, total_issuance_seller: Balance) -> Self {
		self.total_issuance_seller = total_issuance_seller;
		self
	}

	fn total_premium_paid(mut self, total_premium_paid: Balance) -> Self {
		self.total_premium_paid = total_premium_paid;
		self
	}

	fn epoch(mut self, epoch: Epoch<Moment>) -> Self {
		self.epoch = epoch;
		self
	}
}

// ----------------------------------------------------------------------------------------------------
//		OptionInitializer
// ----------------------------------------------------------------------------------------------------

pub trait OptionInitializer {
	fn initialize_options(
		self,
		option_configs: Vec<OptionConfig<AssetId, Balance, Moment>>,
	) -> Self;

	fn initialize_all_options(self) -> Self;
}

impl OptionInitializer for sp_io::TestExternalities {
	fn initialize_options(
		mut self,
		option_configs: Vec<OptionConfig<AssetId, Balance, Moment>>,
	) -> Self {
		self.execute_with(|| {
			option_configs.iter().for_each(|config| {
				TokenizedOptions::create_option(Origin::signed(ADMIN), config.clone()).ok();
			});
		});

		self
	}

	fn initialize_all_options(mut self) -> Self {
		let assets: Vec<AssetId> = Vec::from(ASSETS);

		assets.iter().for_each(|&asset| {
			self.execute_with(|| {
				let price = get_oracle_price(asset, UNIT);

				let config = OptionsConfigBuilder::default()
					.option_type(OptionType::Call)
					.base_asset_id(asset)
					.base_asset_strike_price(price)
					.build();

				let price2 = price.checked_add(price / 10).unwrap();
				let config2 = OptionsConfigBuilder::default()
					.option_type(OptionType::Call)
					.base_asset_id(asset)
					.base_asset_strike_price(price2)
					.build();

				let price3 = price.checked_sub(price / 10).unwrap();
				let config3 = OptionsConfigBuilder::default()
					.option_type(OptionType::Call)
					.base_asset_id(asset)
					.base_asset_strike_price(price3)
					.build();

				let config4 = OptionsConfigBuilder::default()
					.option_type(OptionType::Put)
					.base_asset_id(asset)
					.base_asset_strike_price(price)
					.build();

				let config5 = OptionsConfigBuilder::default()
					.option_type(OptionType::Put)
					.base_asset_id(asset)
					.base_asset_strike_price(price2)
					.build();

				let config6 = OptionsConfigBuilder::default()
					.option_type(OptionType::Put)
					.base_asset_id(asset)
					.base_asset_strike_price(price3)
					.build();

				TokenizedOptions::create_option(Origin::signed(ADMIN), config).ok();
				TokenizedOptions::create_option(Origin::signed(ADMIN), config2).ok();
				TokenizedOptions::create_option(Origin::signed(ADMIN), config3).ok();
				TokenizedOptions::create_option(Origin::signed(ADMIN), config4).ok();
				TokenizedOptions::create_option(Origin::signed(ADMIN), config5).ok();
				TokenizedOptions::create_option(Origin::signed(ADMIN), config6).ok();

				// Make the options go from NotStarted to Deposit phase
				run_to_block(2);
			});
		});

		self
	}
}

// ----------------------------------------------------------------------------------------------------
//		Prop Strategies
// ----------------------------------------------------------------------------------------------------

pub fn random_account() -> impl Strategy<Value = AccountId> {
	prop::sample::select(&[ALICE, BOB, CHARLIE, DAVE, EVEN] as &[_])
}

pub fn random_asset() -> impl Strategy<Value = AssetId> {
	prop::sample::select(&ASSETS as &[_])
}

pub fn random_initial_balances_simpl(
	account_count: u64,
	balance_strategy: impl Strategy<Value = Balance>,
) -> impl Strategy<Value = Vec<Balance>> {
	prop::collection::vec(balance_strategy, account_count as usize * ASSETS_WITH_USDC.len())
}

pub fn random_epoch(
	start_rng: Range<Moment>,
	duration_rng: Range<Moment>,
) -> impl Strategy<Value = Epoch<Moment>> {
	(start_rng, prop::array::uniform4(duration_rng)).prop_map(|(start, duration)| {
		let deposit = start;
		let purchase = deposit + duration[0];
		let exercise = purchase + duration[1];
		let end = exercise + duration[2];
		Epoch { deposit, purchase, exercise, end }
	})
}

pub fn random_option_config(
	balance_strategy: impl Strategy<Value = Balance>,
	start_rng: Range<Moment>,
	duration_rng: Range<Moment>,
) -> impl Strategy<Value = OptionConfig<AssetId, Balance, Moment>> {
	(random_asset(), balance_strategy, random_epoch(start_rng, duration_rng)).prop_map(
		|(base_asset_id, base_asset_strike_price, epoch)| {
			OptionsConfigBuilder::default()
				.base_asset_id(base_asset_id)
				.base_asset_strike_price(base_asset_strike_price)
				.epoch(epoch)
				.build()
		},
	)
}

pub fn random_option_configs(
	option_count_rng: Range<usize>,
	balance_strategy: impl Strategy<Value = Balance>,
	start_rng: Range<Moment>,
	duration_rng: Range<Moment>,
) -> impl Strategy<Value = Vec<OptionConfig<AssetId, Balance, Moment>>> {
	prop::collection::vec(
		random_option_config(balance_strategy, start_rng, duration_rng),
		option_count_rng,
	)
}

// ----------------------------------------------------------------------------------------------------
//		Helper functions
// ----------------------------------------------------------------------------------------------------
// Move the block number to `n` calling the desired hooks
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			Timestamp::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		// Assuming millisecond timestamps, one second for each block
		System::on_initialize(System::block_number());
		Timestamp::on_initialize(System::block_number());
		TokenizedOptions::on_initialize(System::block_number());
		Timestamp::set(Origin::none(), System::block_number() * 1000).unwrap();

		// let max_weight = <<MockRuntime as frame_system::pallet::Config>::BlockWeights as Get<
		// 	frame_system::limits::BlockWeights,
		// >>::get()
		// .max_block;
		// TokenizedOptions::on_idle(System::block_number(), max_weight);
	}
}

// Move the block number by 1 and the timestamp by `n` seconds
pub fn run_for_seconds(n: u64) {
	if System::block_number() > 0 {
		Timestamp::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
	}
	System::set_block_number(System::block_number() + 1);
	System::on_initialize(System::block_number());
	Timestamp::on_initialize(System::block_number());
	TokenizedOptions::on_initialize(System::block_number());
	Timestamp::set(Origin::none(), n * 1000).unwrap();

	// let max_weight = <<MockRuntime as frame_system::pallet::Config>::BlockWeights as Get<
	// 	frame_system::limits::BlockWeights,
	// >>::get()
	// .max_block;
	// TokenizedOptions::on_idle(System::block_number(), max_weight);
}

// Simulate extrinsic call `create_asset_vault`, but returning values
pub fn trait_create_asset_vault(
	_origin: Origin,
	vault_config: VaultConfig<AccountId, AssetId>,
) -> Result<VaultId, DispatchError> {
	let _account_id = ensure_signed(_origin).unwrap();

	let vault_id = <TokenizedOptions as TokenizedOptionsTrait>::create_asset_vault(vault_config)?;

	Ok(vault_id)
}

// Simulate extrinsic call `create_option`, but returning values
pub fn trait_create_option(
	origin: Origin,
	option_config: OptionConfig<AssetId, Balance, Moment>,
) -> Result<OptionId, DispatchError> {
	let _account_id = ensure_signed(origin).unwrap();

	let option_id = <TokenizedOptions as TokenizedOptionsTrait>::create_option(option_config)?;

	Ok(option_id)
}
