use crate::mocks::runtime::{
	BlockNumber, MockRuntime, Moment, System, Timestamp, TokenizedOptions,
};
use frame_support::{
	traits::{Get, OnFinalize, OnIdle, OnInitialize},
	weights::Weight,
};

#[derive(Debug)]
pub(super) struct Block {
	is_initial: bool,
	is_final: bool,
	block_number: BlockNumber,
}

impl Drop for Block {
	fn drop(&mut self) {
		let max_weight = <<MockRuntime as frame_system::pallet::Config>::BlockWeights as Get<
			frame_system::limits::BlockWeights,
		>>::get()
		.max_block;
		let max_weight = if !self.is_final { max_weight } else { Weight::max_value() };
		TokenizedOptions::on_idle(self.block_number, max_weight);
		TokenizedOptions::on_finalize(self.block_number);
	}
}

impl Block {
	pub(super) fn new(
		is_initial: bool,
		is_final: bool,
		block_number: BlockNumber,
		moment: Moment,
	) -> Self {
		if !is_initial {
			System::reset_events();
			System::set_block_number(block_number);
		}
		TokenizedOptions::on_initialize(block_number);
		let moment = if !is_final { moment } else { Moment::max_value() };
		Timestamp::set_timestamp(moment);
		Block { is_initial, is_final, block_number }
	}
	pub(super) fn is_initial(&self) -> bool {
		self.is_initial
	}
	pub(super) fn is_final(&self) -> bool {
		self.is_final
	}
}

#[derive(Debug)]
pub(super) struct BlockProducer {
	durations: std::vec::IntoIter<Moment>,
	is_initial: bool,
	block_number: BlockNumber,
	moment: Moment,
}

impl BlockProducer {
	pub(super) fn new(durations: Vec<Moment>) -> Self {
		BlockProducer {
			durations: durations.into_iter(),
			is_initial: true,
			block_number: System::block_number(),
			moment: Timestamp::get(),
		}
	}
}

impl Iterator for BlockProducer {
	type Item = Block;
	fn next(&mut self) -> Option<Self::Item> {
		match self.durations.next() {
			Some(duration) => {
				let is_final = self.durations.len() == 0;
				let block = Block::new(self.is_initial, is_final, self.block_number, self.moment);
				self.is_initial = false;
				self.block_number += 1;
				self.moment += duration;
				Some(block)
			},
			None => None,
		}
	}
}
