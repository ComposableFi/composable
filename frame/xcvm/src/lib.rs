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
		sp_runtime::{traits::Dispatchable, SaturatedConversion},
		traits::{
			fungibles::{Inspect as FungiblesInspect, Transfer as FungiblesTransfer},
			tokens::{AssetId, Balance},
		},
		transactional,
	};
	use frame_system::{ensure_signed, pallet_prelude::*};
	use xcvm_core::*;

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type XCVMInstructionOf<T> =
		XCVMInstruction<XCVMNetwork, AbiEncoded, AccountIdOf<T>, XCVMTransfer>;
	pub(crate) type XCVMProgramOf<T> = XCVMProgram<XCVMInstructionOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Executed { instruction: XCVMInstructionOf<T> },
		Spawn { network: XCVMNetwork, program: XCVMProgramOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidProgramEncoding,
		UnknownAsset,
		InvalidCallEncoding,
		CallFailed,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Dispatchable: Parameter + Dispatchable<Origin = Self::Origin>;
		type AssetId: AssetId + Ord + TryFrom<XCVMAsset>;
		type Assets: FungiblesInspect<
				AccountIdOf<Self>,
				AssetId = AssetIdOf<Self>,
				Balance = BalanceOf<Self>,
			> + FungiblesTransfer<AccountIdOf<Self>>;
		type Balance: Balance;
		type MaxProgramSize: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		AccountIdOf<T>: TryFrom<Vec<u8>>,
	{
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn execute(
			origin: OriginFor<T>,
			program: BoundedVec<u8, T::MaxProgramSize>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			let XCVMProgram { mut instructions } = xcvm_protobuf::decode::<
				XCVMNetwork,
				<XCVMNetwork as Callable>::EncodedCall,
				AccountIdOf<T>,
				XCVMTransfer,
			>(program.as_ref())
			.map_err(|_| Error::<T>::InvalidProgramEncoding)?;

			while let Some(instruction) = instructions.pop_front() {
				match instruction.clone() {
					XCVMInstruction::Transfer(to, XCVMTransfer { assets }) => {
						for (asset, amount) in assets {
							let concrete_asset = TryInto::<AssetIdOf<T>>::try_into(asset)
								.map_err(|_| Error::<T>::UnknownAsset)?;
							T::Assets::transfer(
								concrete_asset,
								&who,
								&to,
								amount.saturated_into(),
								false,
							)?;
						}
					},
					XCVMInstruction::Call(abi) => {
						let payload: Vec<u8> = abi.clone().into();
						let call = <T::Dispatchable as Decode>::decode(&mut &payload[..])
							.map_err(|_| Error::<T>::InvalidCallEncoding)?;
						call.dispatch(origin.clone()).map_err(|_| Error::<T>::CallFailed)?;
					},
					XCVMInstruction::Bridge(network, XCVMTransfer { assets }) => {
						for (asset, amount) in assets {
							let concrete_asset = TryInto::<AssetIdOf<T>>::try_into(asset)
								.map_err(|_| Error::<T>::UnknownAsset)?;
							// TODO: mosaic
						}
						Self::deposit_event(Event::<T>::Spawn {
							network,
							program: XCVMProgram { instructions: instructions.clone() },
						});
					},
					XCVMInstruction::Spawn(network, program) =>
						Self::deposit_event(Event::<T>::Spawn {
							network,
							program: XCVMProgram { instructions: program.clone() },
						}),
				}
				Self::deposit_event(Event::<T>::Executed { instruction })
			}

			Ok(().into())
		}
	}
}
