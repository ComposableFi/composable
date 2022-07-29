use frame_support::{
	traits::{Get, OnFinalize, OnIdle, OnInitialize},
	weights::Weight,
};
use frame_system::{Config as SystemConfig, Pallet as SystemPallet};
use pallet_timestamp::{Config as TimestampConfig, Pallet as TimestampPallet};
use proptest::prelude::{prop, Strategy};
use sp_runtime::traits::{Bounded, CheckedAdd, One, Saturating};
use std::{fmt::Debug, marker::PhantomData, ops::Range};

/// Block producer configuration trait.
pub trait BlocksConfig {
	/// Runtime type.
	type Runtime: pallet_timestamp::Config;

	/// Pallet type for which hooks are called.
	type Hooked: OnInitialize<BlockNumberOf<Self>>
		+ OnIdle<BlockNumberOf<Self>>
		+ OnFinalize<BlockNumberOf<Self>>;
}

pub type MomentOf<C> = <<C as BlocksConfig>::Runtime as TimestampConfig>::Moment;
type BlockNumberOf<C> = <<C as BlocksConfig>::Runtime as SystemConfig>::BlockNumber;

pub struct Block<'t, C: BlocksConfig> {
	weight: Weight,
	is_initial: bool,
	is_final: bool,
	block_number: BlockNumberOf<C>,
	_marker: PhantomData<&'t mut ()>,
}

impl<C: BlocksConfig> Debug for Block<'_, C> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("Block")
			.field("weight", &self.weight)
			.field("is_initial", &self.is_initial)
			.field("is_final", &self.is_final)
			.field("block_number", &self.block_number)
			.finish()
	}
}

impl<C: BlocksConfig> Drop for Block<'_, C> {
	fn drop(&mut self) {
		let max_weight = <C::Runtime as SystemConfig>::BlockWeights::get().max_block;
		let remaining_weight = if !self.is_final {
			max_weight.saturating_sub(self.weight)
		} else {
			Weight::max_value()
		};
		let idle_weight = C::Hooked::on_idle(self.block_number, remaining_weight);
		self.inc_weight(idle_weight);
		C::Hooked::on_finalize(self.block_number);
	}
}

impl<C: BlocksConfig> Block<'_, C> {
	pub fn weight(&self) -> Weight {
		self.weight
	}
	pub fn inc_weight(&mut self, weight: Weight) {
		self.weight = self.weight.saturating_add(weight);
	}
	pub fn is_initial(&self) -> bool {
		self.is_initial
	}
	pub fn is_final(&self) -> bool {
		self.is_final
	}
}

pub struct BlockProducer<C: BlocksConfig> {
	moments: std::vec::IntoIter<MomentOf<C>>,
	is_initial: bool,
	block_number: BlockNumberOf<C>,
}

impl<C: BlocksConfig> Debug for BlockProducer<C> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("BlockProducer")
			.field("moments", &self.moments)
			.field("is_initial", &self.is_initial)
			.field("block_number", &self.block_number)
			.finish()
	}
}

impl<C: BlocksConfig> BlockProducer<C> {
	pub fn new(moments: Vec<MomentOf<C>>) -> Self {
		BlockProducer {
			moments: moments.into_iter(),
			is_initial: true,
			block_number: SystemPallet::<C::Runtime>::block_number(),
		}
	}
	/// Generates random block producer from block count and
	/// time interval ranges. First block is produced on initial moment;
	/// last block will have timestamp on infinity when `finalize` is `true`.
	pub fn generate(
		block_count_rng: Range<usize>,
		interval_rng: Range<u32>,
		finalize: bool,
	) -> impl Strategy<Value = Self> {
		prop::collection::vec(interval_rng, block_count_rng).prop_map(move |intervals| {
			let mut moment = TimestampPallet::<C::Runtime>::get();
			let mut moments = Vec::with_capacity(intervals.len());
			for interval in intervals.into_iter() {
				moments.push(moment);
				moment = moment.saturating_add(interval.into());
			}
			if let (true, Some(moment)) = (finalize, moments.last_mut()) {
				*moment = MomentOf::<C>::max_value();
			}
			BlockProducer::new(moments)
		})
	}
	pub fn next_block(&mut self) -> Option<Block<'_, C>> {
		match self.moments.next() {
			Some(moment) => {
				if !self.is_initial {
					SystemPallet::<C::Runtime>::reset_events();
					SystemPallet::<C::Runtime>::set_block_number(self.block_number);
				}
				// TODO: move the next line after on_initialize and call on_post_inherent
				// when https://github.com/paritytech/substrate/pull/10128 is merged.
				TimestampPallet::<C::Runtime>::set_timestamp(moment);
				let weight = C::Hooked::on_initialize(self.block_number);

				let block = Block {
					weight,
					is_initial: self.is_initial,
					is_final: self.moments.len() == 0,
					block_number: self.block_number,
					_marker: Default::default(),
				};

				self.is_initial = false;
				self.block_number = self
					.block_number
					.checked_add(&BlockNumberOf::<C>::one())
					.expect("Hit the limit for block number!");
				Some(block)
			},
			None => None,
		}
	}
}

pub fn random_intervals(
	block_count_rng: Range<usize>,
	interval_rng: Range<u32>,
) -> impl Strategy<Value = Vec<u32>> {
	prop::collection::vec(interval_rng, block_count_rng)
}
