use super::{
	block_producer::{BlockProducer, BlocksConfig, BlocksData, Run},
	random_initial_balances_simpl, random_option_config, OptionConfig, VaultInitializer, UNIT,
};
use crate::mock::{
	accounts::{account_id_from_u64, AccountId},
	assets::AssetId,
	runtime::{
		Balance, Event, ExtBuilder, MockRuntime, Moment, OptionId, Origin, System, Timestamp,
		TokenizedOptions,
	},
};
use composable_traits::tokenized_options::TokenizedOptions as TokenizedOptionsTrait;
use proptest::prelude::{
	any, prop, prop::sample::Index, prop_compose, proptest, Just, ProptestConfig, Strategy,
};
use sp_runtime::DispatchResult;
use std::{cell::RefCell, ops::Range, rc::Rc};

fn balance_strategy(max: Balance) -> impl Strategy<Value = Balance> + Clone {
	(0..=max).prop_map(|b| b * UNIT)
}

proptest! {
	#![proptest_config(ProptestConfig {
		cases: 1, .. ProptestConfig::default()
	})]
	#[test]
	fn total_proptest(
		(balances, option_configs, blocks_data, option_id_pool) in random_values(
			10..15,                       // account_count_rng
			balance_strategy(1_000_000),  // account_balance_strategy
			2..5,                         // option_count_rng
			balance_strategy(1_000),      // option_balance_strategy
			0..1000,                      // option_start_rng
			100..110,                     // option_duration_rng
			100..500,                     // block_count_rng
			10..15,                       // interval_rng
			0..150,                       // extrinsic_count_rng
			1..3,                         // option_amount_rng
			OptionIdPool::default(),      // option_id_pool
		),
	) {
		ExtBuilder::default()
			.initialize_balances_simpl(&balances)
			.build()
			.initialize_oracle_prices()
			.initialize_all_vaults()
			.execute_with(|| do_total_proptest(option_configs, blocks_data, option_id_pool));
	}
}

prop_compose! {
	#[allow(clippy::too_many_arguments)]
	fn random_values(
		account_count_rng: Range<u64>,
		account_balance_strategy: impl Strategy<Value = Balance> + Clone,
		option_count_rng: Range<usize>,
		option_balance_strategy: impl Strategy<Value = Balance> + Clone,
		option_start_rng: Range<Moment>,
		option_duration_rng: Range<Moment>,
		block_count_rng: Range<usize>,
		interval_rng: Range<u32>,
		extrinsic_count_rng: Range<usize>,
		option_amount_rng: Range<Balance>,
		option_id_pool: OptionIdPool,
	)(
		account_count in account_count_rng,
		option_count in option_count_rng,
		option_id_pool2 in Just(option_id_pool.clone()),
	)(
		balances in random_initial_balances_simpl(account_count, account_balance_strategy.clone()),
		option_configs in prop::collection::vec(random_option_config(option_balance_strategy.clone(), option_start_rng.clone(), option_duration_rng.clone()), option_count),
		blocks_data in BlocksData::<TokenizedOptionsBlocksConfig>::generate(
			block_count_rng.clone(),
			interval_rng.clone(),
			extrinsic_count_rng.clone(),
			TokenizedOptionsExtrinsic::generate(1..=account_count, option_amount_rng.clone(), option_id_pool2)
		),
	) -> (Vec<Balance>, Vec<OptionConfig<AssetId, Balance, Moment>>, BlocksData::<TokenizedOptionsBlocksConfig>, OptionIdPool) {
		(balances, option_configs, blocks_data, option_id_pool.clone())
	}
}

fn do_total_proptest(
	mut option_configs: Vec<OptionConfig<AssetId, Balance, Moment>>,
	blocks_data: BlocksData<TokenizedOptionsBlocksConfig>,
	option_id_pool: OptionIdPool,
) {
	let mut block_producer = BlockProducer::new(blocks_data);
	while let Some(mut block) = block_producer.next_block() {
		if block.is_initial() {
			initialize_options(std::mem::take(&mut option_configs), option_id_pool.clone());
		}
		while let Some((_extrinsic, _result)) = block.call_next_extrinsic() {
			// if _result.is_ok() {
			// 	dbg!(_extrinsic);
			// }
			// if let Err(err) = _result {
			// 	dbg!(_extrinsic);
			// 	dbg!(err);
			// 	return;
			// }
		}
		drop(block);
		for event in System::events() {
			let event = event.event;
			if matches!(
				event,
				Event::TokenizedOptions(crate::Event::SellOption { .. })
					| Event::TokenizedOptions(crate::Event::DeleteSellOption { .. })
					| Event::TokenizedOptions(crate::Event::BuyOption { .. })
			) {
				dbg!(event);
			}
		}
	}
}

fn initialize_options(
	option_configs: Vec<OptionConfig<AssetId, Balance, Moment>>,
	option_id_pool: OptionIdPool,
) {
	option_id_pool.add_capacity(option_configs.len());
	for option_config in option_configs {
		if let Ok(option_id) =
			<TokenizedOptions as TokenizedOptionsTrait>::create_option(option_config)
		{
			option_id_pool.push(option_id);
		}
	}
}

enum TokenizedOptionsBlocksConfig {}

impl BlocksConfig for TokenizedOptionsBlocksConfig {
	type Runtime = MockRuntime;
	type Hooked = TokenizedOptions;
	type Extrinsic = TokenizedOptionsExtrinsic;
}

#[derive(Clone, Debug)]
struct TokenizedOptionsExtrinsic {
	extrinsic_type: TokenizedOptionsExtrinsicType,
	account_id: AccountId,
	option_amount: Balance,
	option_id_index: Index,
	option_id_pool: OptionIdPool,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, proptest_derive::Arbitrary)]
enum TokenizedOptionsExtrinsicType {
	SellOption,
	DeleteSellOption,
	BuyOption,
}

impl Run for TokenizedOptionsExtrinsic {
	fn run(&self) -> DispatchResult {
		match self.extrinsic_type {
			TokenizedOptionsExtrinsicType::SellOption => TokenizedOptions::sell_option(
				Origin::signed(self.account_id),
				self.option_amount,
				self.option_id_pool.get(self.option_id_index),
			),
			TokenizedOptionsExtrinsicType::DeleteSellOption => {
				TokenizedOptions::delete_sell_option(
					Origin::signed(self.account_id),
					self.option_amount,
					self.option_id_pool.get(self.option_id_index),
				)
			},
			TokenizedOptionsExtrinsicType::BuyOption => TokenizedOptions::buy_option(
				Origin::signed(self.account_id),
				self.option_amount,
				self.option_id_pool.get(self.option_id_index),
			),
		}
	}
}

impl TokenizedOptionsExtrinsic {
	fn generate(
		account_id_strategy: impl Strategy<Value = u64> + Clone,
		option_amount_rng: Range<Balance>,
		option_id_pool: OptionIdPool,
	) -> impl Strategy<Value = Self> + Clone {
		(
			any::<TokenizedOptionsExtrinsicType>(),
			account_id_strategy,
			option_amount_rng,
			any::<Index>(),
		)
			.prop_map(move |(extrinsic_type, account_id, option_amount, option_id_index)| {
				let account_id = account_id_from_u64(account_id);
				let option_id_pool = option_id_pool.clone();
				TokenizedOptionsExtrinsic {
					extrinsic_type,
					account_id,
					option_amount,
					option_id_index,
					option_id_pool,
				}
			})
	}
}

#[derive(Clone, Debug, Default)]
struct OptionIdPool(Rc<RefCell<Vec<OptionId>>>);

impl OptionIdPool {
	fn len(&self) -> usize {
		self.0.borrow().len()
	}
	fn get(&self, index: Index) -> OptionId {
		*index.get(self.0.borrow().as_slice())
	}
	fn add_capacity(&self, capacity: usize) {
		self.0.borrow_mut().reserve(capacity);
	}
	fn push(&self, option_id: OptionId) {
		self.0.borrow_mut().push(option_id)
	}
}
