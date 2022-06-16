use super::{block_producer::BlockProducer, OptionsConfigBuilder, VaultInitializer};
use crate::{
	mocks::runtime::{Event, ExtBuilder, Moment, OptionId, System, Timestamp, TokenizedOptions},
	types::Epoch,
};
use composable_traits::tokenized_options::TokenizedOptions as TokenizedOptionsTrait;
use frame_support::assert_ok;
use proptest::prelude::*;
use std::{collections::HashMap, ops::Range};

prop_compose! {
	fn random_epoch(start_rng: Range<Moment>, duration_rng: Range<Moment>)
	(start in start_rng, duration in prop::array::uniform4(duration_rng)) -> Epoch<Moment> {
		let deposit = start;
		let purchase = deposit + duration[0];
		let exercise = purchase + duration[1];
		let end = exercise + duration[2];
		Epoch { deposit, purchase, exercise, end }
	}
}

fn random_epochs(
	n_rng: Range<usize>,
	start_rng: Range<Moment>,
	duration_rng: Range<Moment>,
) -> impl Strategy<Value = Vec<Epoch<Moment>>> {
	prop::collection::vec(random_epoch(start_rng, duration_rng), n_rng)
}

fn random_durations(
	m_rng: Range<usize>,
	duration_rng: Range<Moment>,
) -> impl Strategy<Value = Vec<Moment>> {
	prop::collection::vec(duration_rng, m_rng)
}

proptest! {
	#![proptest_config(ProptestConfig {
		cases: 10, .. ProptestConfig::default()
	})]
	#[test]
	fn test_time_management(epochs in random_epochs(50..200, 0..1000, 10..100), durations in random_durations(100..500, 10..50)) {
		ExtBuilder::default()
			.build()
			.initialize_oracle_prices()
			.initialize_all_vaults()
			.execute_with(|| do_test_time_management(epochs, durations));
	}
}

fn do_test_time_management(mut epochs: Vec<Epoch<Moment>>, durations: Vec<Moment>) {
	let mut tester = Tester::default();
	for block in BlockProducer::new(durations) {
		if block.is_initial() {
			let options = initialize_options(std::mem::take(&mut epochs));
			tester.set_options(options);
		}
		drop(block);
		tester.block_test();
	}
	tester.final_test();
}

fn initialize_options(epochs: Vec<Epoch<Moment>>) -> HashMap<OptionId, Epoch<Moment>> {
	let mut hash_map = HashMap::with_capacity(epochs.len());
	for (i, epoch) in epochs.into_iter().enumerate() {
		let option_config = OptionsConfigBuilder::default()
			.base_asset_strike_price(i as _)
			.epoch(epoch)
			.build();
		let option_id = <TokenizedOptions as TokenizedOptionsTrait>::create_option(option_config);
		if let Ok(option_id) = option_id {
			hash_map.insert(option_id, epoch);
		}
		assert_ok!(option_id);
	}
	hash_map
}

#[derive(Debug, Default)]
struct Tester {
	options: HashMap<OptionId, Epoch<Moment>>,
	counters: [usize; 4],
	event_moment: Moment,
}

impl Tester {
	fn set_options(&mut self, options: HashMap<OptionId, Epoch<Moment>>) {
		self.options = options;
	}
	fn block_test(&mut self) {
		for event in System::events() {
			let event = event.event;
			let event_moment = match event {
				Event::TokenizedOptions(crate::Event::OptionDepositStart { option_id }) => {
					self.counters[0] += 1;
					self.options[&option_id].deposit
				},
				Event::TokenizedOptions(crate::Event::OptionPurchaseStart { option_id }) => {
					self.counters[1] += 1;
					self.options[&option_id].purchase
				},
				Event::TokenizedOptions(crate::Event::OptionExerciseStart { option_id }) => {
					self.counters[2] += 1;
					self.options[&option_id].exercise
				},
				Event::TokenizedOptions(crate::Event::OptionEnd { option_id }) => {
					self.counters[3] += 1;
					self.options[&option_id].end
				},
				_ => continue,
			};
			assert!(event_moment <= Timestamp::get());
			assert!(self.event_moment <= event_moment);
			self.event_moment = event_moment;
		}
	}
	fn final_test(&mut self) {
		for counter in self.counters {
			assert_eq!(counter, self.options.len());
		}
	}
}
