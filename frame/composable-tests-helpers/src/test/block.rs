use core::ops::Mul;

use composable_support::math::safe::SafeAdd;
use frame_support::traits::Hooks;
use frame_system::Config as FrameSystemConfig;
use pallet_timestamp::Config as PalletTimestampConfig;
use sp_runtime::traits::One;

pub const MILLISECS_PER_BLOCK: u64 = 6000;

/// Processes the specified amount of blocks, calls [`next_block()`] and then calls
/// [`Pallet::on_finalize`](Hooks::on_finalize).
pub fn process_and_progress_blocks<Pallet, Runtime>(blocks_to_process: usize)
where
	Runtime: FrameSystemConfig + PalletTimestampConfig,
	<Runtime as FrameSystemConfig>::BlockNumber: SafeAdd,
	Pallet: Hooks<<Runtime as FrameSystemConfig>::BlockNumber>,
	u64: Mul<
		<Runtime as FrameSystemConfig>::BlockNumber,
		Output = <Runtime as PalletTimestampConfig>::Moment,
	>,
{
	(0..blocks_to_process).for_each(|_| {
		let new_block = next_block::<Pallet, Runtime>();
		Pallet::on_finalize(new_block);
	})
}

/// Progresses to the next block, initializes the block with
/// [`Pallet::on_initialize`](Hooks::on_initialize), and then sets the timestamp to where it
/// should be for the block. Returns the next block.
pub fn next_block<Pallet, Runtime>() -> <Runtime as FrameSystemConfig>::BlockNumber
where
	Runtime: FrameSystemConfig + PalletTimestampConfig,
	<Runtime as FrameSystemConfig>::BlockNumber: SafeAdd,
	Pallet: Hooks<<Runtime as FrameSystemConfig>::BlockNumber>,
	u64: Mul<
		<Runtime as FrameSystemConfig>::BlockNumber,
		Output = <Runtime as PalletTimestampConfig>::Moment,
	>,
{
	let next_block = frame_system::Pallet::<Runtime>::block_number()
		.safe_add(&<<Runtime as FrameSystemConfig>::BlockNumber as One>::one())
		.expect("hit the numeric limit for block number");

	// uncomment if you want to obliterate your terminal
	// println!("PROCESSING BLOCK {}", next_block);

	frame_system::Pallet::<Runtime>::set_block_number(next_block);
	pallet_timestamp::Pallet::<Runtime>::set_timestamp(MILLISECS_PER_BLOCK * next_block);
	Pallet::on_initialize(next_block);

	next_block
}
