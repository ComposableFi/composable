use composable_support::math::safe::SafeAdd;
use frame_support::traits::Hooks;
use frame_system::Config as FrameSystemConfig;
use pallet_timestamp::Config as PalletTimestampConfig;
use sp_runtime::traits::{Header, One};

pub const MILLISECS_PER_BLOCK: u64 = 6000;

/// Processes the specified amount of blocks with [`next_block()`]
pub fn process_and_progress_blocks<Pallet, Runtime>(blocks_to_process: usize)
where
	Runtime: FrameSystemConfig + PalletTimestampConfig,
	<Runtime as FrameSystemConfig>::BlockNumber: SafeAdd,
	Pallet: Hooks<<Runtime as FrameSystemConfig>::BlockNumber>,
	<Runtime as FrameSystemConfig>::BlockNumber: Into<u64>,
	<Runtime as PalletTimestampConfig>::Moment: From<u64>,
{
	(0..blocks_to_process).for_each(|_| {
		next_block::<Pallet, Runtime>();
	})
}

/// Finalizes the previous block with [`Pallet::on_finalize`](Hooks::on_finalize), progresses to the
/// next block, initializes the block with [`Pallet::on_initialize`](Hooks::on_initialize), and then
/// sets the timestamp to where it should be for the block. Returns the block number of the block
/// that was progressed to.
pub fn next_block<Pallet, Runtime>() -> <Runtime as FrameSystemConfig>::BlockNumber
where
	Runtime: FrameSystemConfig + PalletTimestampConfig,
	<Runtime as FrameSystemConfig>::BlockNumber: SafeAdd,
	Pallet: Hooks<<Runtime as FrameSystemConfig>::BlockNumber>,
	<Runtime as FrameSystemConfig>::BlockNumber: Into<u64>,
	<Runtime as PalletTimestampConfig>::Moment: From<u64>,
{
	frame_system::Pallet::<Runtime>::note_finished_extrinsics();
	let header = frame_system::Pallet::<Runtime>::finalize();
	let current_block = frame_system::Pallet::<Runtime>::block_number();

	Pallet::on_finalize(current_block);

	let next_block = current_block
		.safe_add(&<<Runtime as FrameSystemConfig>::BlockNumber as One>::one())
		.expect("hit the numeric limit for block number");

	frame_system::Pallet::<Runtime>::reset_events();
	frame_system::Pallet::<Runtime>::initialize(
		&next_block,
		header.parent_hash(),
		// &[0u8; 32].into(),
		&Default::default(),
	);

	// uncomment if you want to obliterate your terminal
	// println!("PROCESSING BLOCK {}", next_block);

	frame_system::Pallet::<Runtime>::on_initialize(next_block);
	frame_system::Pallet::<Runtime>::set_block_number(next_block);

	pallet_timestamp::Pallet::<Runtime>::set_timestamp(
		(MILLISECS_PER_BLOCK * Into::<u64>::into(next_block)).into(),
	);

	Pallet::on_initialize(next_block);

	next_block
}
