use frame_support::{
	traits::{Get, OnFinalize, OnIdle, OnInitialize},
	weights::Weight,
};
use frame_system::{Config as SystemConfig, Pallet as SystemPallet};
use pallet_timestamp::{Config as TimestampConfig, Pallet as TimestampPallet};
use proptest::prelude::{prop, Strategy};
use sp_runtime::{
	traits::{Bounded, CheckedAdd, One, Saturating},
	DispatchResult,
};
use std::{fmt::Debug, ops::Range};

pub trait Run {
	fn run(&self) -> DispatchResult;
	fn weight(&self) -> Weight {
		0
	}
}

/// Block producer configuration trait.
pub trait BlocksConfig {
	/// Runtime type.
	type Runtime: pallet_timestamp::Config;

	/// Pallet type for which hooks are called.
	type Hooked: OnInitialize<BlockNumberOf<Self>>
		+ OnIdle<BlockNumberOf<Self>>
		+ OnFinalize<BlockNumberOf<Self>>;

	/// Extrinsic type executed by the block producer.
	type Extrinsic: Run;
}

type BlockNumberOf<C> = <<C as BlocksConfig>::Runtime as SystemConfig>::BlockNumber;
type MomentOf<C> = <<C as BlocksConfig>::Runtime as TimestampConfig>::Moment;

pub struct Block<'t, C: BlocksConfig> {
	extrinsics: &'t mut std::vec::IntoIter<C::Extrinsic>,
	extrinsic_count: usize,
	weight: Weight,
	is_initial: bool,
	is_final: bool,
	block_number: BlockNumberOf<C>,
}

impl<C: BlocksConfig> Debug for Block<'_, C>
where
	C::Extrinsic: Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("Block")
			.field("extrinsics", &self.extrinsics)
			.field("extrinsic_count", &self.extrinsic_count)
			.field("weight", &self.weight)
			.field("is_initial", &self.is_initial)
			.field("is_final", &self.is_final)
			.field("block_number", &self.block_number)
			.finish()
	}
}

impl<C: BlocksConfig> Drop for Block<'_, C> {
	fn drop(&mut self) {
		while self.call_next_extrinsic().is_some() {}
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
	pub fn call_next_extrinsic(&mut self) -> Option<(C::Extrinsic, DispatchResult)> {
		if self.extrinsic_count == 0 {
			return None;
		}
		self.extrinsic_count -= 1;
		let extrinsic = self.extrinsics.next().expect("No extrinsic for the block!");
		let result = extrinsic.run();
		self.inc_weight(extrinsic.weight());
		Some((extrinsic, result))
	}
	pub fn is_initial(&self) -> bool {
		self.is_initial
	}
	pub fn is_final(&self) -> bool {
		self.is_final
	}
}

pub struct BlockProducer<C: BlocksConfig> {
	blocks_data: std::vec::IntoIter<BlockData<C>>,
	extrinsics: std::vec::IntoIter<C::Extrinsic>,
	is_initial: bool,
	block_number: BlockNumberOf<C>,
	moment: MomentOf<C>,
}

impl<C: BlocksConfig> Debug for BlockProducer<C>
where
	C::Extrinsic: Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("BlockProducer")
			.field("blocks_data", &self.blocks_data)
			.field("extrinsics", &self.extrinsics)
			.field("is_initial", &self.is_initial)
			.field("block_number", &self.block_number)
			.field("moment", &self.moment)
			.finish()
	}
}

impl<C: BlocksConfig> BlockProducer<C> {
	pub fn new(blocks_data: BlocksData<C>) -> Self {
		BlockProducer {
			blocks_data: blocks_data.blocks_data.into_iter(),
			extrinsics: blocks_data.extrinsics.into_iter(),
			is_initial: true,
			block_number: SystemPallet::<C::Runtime>::block_number(),
			moment: TimestampPallet::<C::Runtime>::get(),
		}
	}
	/// Build block producer from slice of intervals between blocks;
	/// count of blocks will be the same as of intervals.
	/// First block is produced on initial moment;
	/// last block will have infinite timestamp when `finalize` is `true`.
	pub fn from_intervals<M: Into<MomentOf<C>>>(intervals: &[M], finalize: bool) {
		//
	}
	pub fn next_block(&mut self) -> Option<Block<'_, C>> {
		match self.blocks_data.next() {
			Some(BlockData { interval, extrinsic_count }) => {
				if !self.is_initial {
					SystemPallet::<C::Runtime>::reset_events();
					SystemPallet::<C::Runtime>::set_block_number(self.block_number);
				}
				let weight = C::Hooked::on_initialize(self.block_number);
				let is_final = self.blocks_data.len() == 0;
				let moment = if !is_final { self.moment } else { MomentOf::<C>::max_value() };
				TimestampPallet::<C::Runtime>::set_timestamp(moment);

				let block = Block {
					extrinsics: &mut self.extrinsics,
					extrinsic_count,
					weight,
					is_initial: self.is_initial,
					is_final,
					block_number: self.block_number,
				};

				self.is_initial = false;
				self.block_number = self
					.block_number
					.checked_add(&BlockNumberOf::<C>::one())
					.expect("Hit the limit for block number!");
				self.moment = moment.saturating_add(interval);
				Some(block)
			},
			None => None,
		}
	}
}

struct BlockData<C: BlocksConfig> {
	interval: MomentOf<C>,
	extrinsic_count: usize,
}

impl<C: BlocksConfig> Clone for BlockData<C> {
	fn clone(&self) -> Self {
		Self { interval: self.interval, extrinsic_count: self.extrinsic_count }
	}
}

impl<C: BlocksConfig> Debug for BlockData<C> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("BlockData")
			.field("interval", &self.interval)
			.field("extrinsic_count", &self.extrinsic_count)
			.finish()
	}
}

impl<C: BlocksConfig> BlockData<C> {
	fn generate_empty(interval_rng: Range<u32>) -> impl Strategy<Value = Self> + Clone {
		interval_rng.prop_map(|interval| BlockData {
			interval: MomentOf::<C>::from(interval),
			extrinsic_count: 0,
		})
	}
	fn generate(
		interval_rng: Range<u32>,
		extrinsic_count_rng: Range<usize>,
	) -> impl Strategy<Value = Self> + Clone {
		(interval_rng, extrinsic_count_rng).prop_map(|(interval, extrinsic_count)| BlockData {
			interval: MomentOf::<C>::from(interval),
			extrinsic_count,
		})
	}
}

pub struct BlocksData<C: BlocksConfig> {
	blocks_data: Vec<BlockData<C>>,
	extrinsics: Vec<C::Extrinsic>,
}

impl<C: BlocksConfig> Clone for BlocksData<C>
where
	C::Extrinsic: Clone,
{
	fn clone(&self) -> Self {
		Self { blocks_data: self.blocks_data.clone(), extrinsics: self.extrinsics.clone() }
	}
}

impl<C: BlocksConfig> Debug for BlocksData<C>
where
	C::Extrinsic: Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("BlocksData")
			.field("blocks_data", &self.blocks_data)
			.field("extrinsics", &self.extrinsics)
			.finish()
	}
}

impl<C: BlocksConfig> BlocksData<C>
where
	C::Extrinsic: Debug,
{
	pub fn generate_empty(
		block_count_rng: Range<usize>,
		interval_rng: Range<u32>,
	) -> impl Strategy<Value = Self> + Clone {
		prop::collection::vec(BlockData::generate_empty(interval_rng), block_count_rng)
			.prop_map(|blocks_data| BlocksData { blocks_data, extrinsics: Vec::new() })
	}
	pub fn generate(
		block_count_rng: Range<usize>,
		interval_rng: Range<u32>,
		extrinsic_count_rng: Range<usize>,
		extrinsic_strategy: impl Strategy<Value = C::Extrinsic> + Clone,
	) -> impl Strategy<Value = Self> + Clone {
		prop::collection::vec(
			BlockData::generate(interval_rng, extrinsic_count_rng),
			block_count_rng,
		)
		.prop_flat_map(move |blocks_data| {
			let extrinsic_count =
				blocks_data.iter().fold(0, |acc, block_data| acc + block_data.extrinsic_count);
			prop::collection::vec(extrinsic_strategy.clone(), extrinsic_count).prop_map(
				move |extrinsics| BlocksData { blocks_data: blocks_data.clone(), extrinsics },
			)
		})
	}
}
