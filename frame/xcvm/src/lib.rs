#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		storage::bounded_btree_map::BoundedBTreeMap,
		traits::{
			fungibles::{Inspect as FungiblesInspect, Transfer as FungiblesTransfer},
			tokens::{AssetId, Balance},
		},
	};
	use frame_system::pallet_prelude::*;
	use xcvm_core::{AbiEncoded, Callable, XCVMInstruction, XCVMNetwork, XCVMProgram};

	pub(crate) type AccountIdOf<T> = <T as Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type MaxTransferAssetsOf<T> = <T as Config>::MaxTransferAssets;
	pub(crate) type MaxInstructionsOf<T> = <T as Config>::MaxInstructions;
	pub(crate) type XCVMInstructionOf<T> = XCVMInstruction<
		XCVMNetwork,
		AbiEncoded,
		AccountIdOf<T>,
		BoundedBTreeMap<AssetIdOf<T>, BalanceOf<T>, MaxTransferAssetsOf<T>>,
	>;
	pub(crate) type XCVMInstructionsOf<T> = Vec<XCVMInstructionOf<T>>;
	pub(crate) type XCVMProgramOf<T> = XCVMProgram<XCVMInstructionsOf<T>>;
	use sp_std::collections::btree_map::BTreeMap;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Executed { instruction: XCVMInstructionOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidProgramEncoding,
		InstructionPointerOutOfRange,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AccountId: Parameter
			+ MaybeSerializeDeserialize
			+ Ord
			+ MaxEncodedLen
			+ TryFrom<Vec<u8>>;

		type AssetId: AssetId + Ord;
		type Assets: FungiblesInspect<
				AccountIdOf<Self>,
				AssetId = AssetIdOf<Self>,
				Balance = BalanceOf<Self>,
			> + FungiblesTransfer<Self>;
		type Balance: Balance;
		type MaxTransferAssets: Get<u32>;
		type MaxInstructions: Get<u32>;
		type MaxProgramSize: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn execute(
			origin: OriginFor<T>,
			program: BoundedVec<u8, T::MaxProgramSize>,
		) -> DispatchResultWithPostInfo {
			let program = xcvm_protobuf::decode::<
				XCVMNetwork,
				<XCVMNetwork as Callable>::EncodedCall,
				AccountIdOf<T>,
				BTreeMap<u32, u128>,
			>(program.as_ref())
			.map_err(|_| Error::<T>::InvalidProgramEncoding)?;

			let instructions = program.instructions;
			let mut ip = program.instruction_pointer;

			while ip < instructions.len() as u32 {
				if let Some(instruction) = instructions.get(ip as usize) {
					match instruction {
						XCVMInstruction::Transfer(to, assets) => {
							// T::Assets::transfer(origin, to.clone(), assets.clone())?;
						},
						XCVMInstruction::Call(abi) => {
							// decoded abi
						},
						XCVMInstruction::Bridge(network, assets) => {
							// mosaic?
						},
					}
				} else {
					return Err(Error::<T>::InstructionPointerOutOfRange.into());
				}
				ip += 1;
			}
			Ok(().into())
		}
	}
}
