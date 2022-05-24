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
	use composable_traits::mosaic::TransferTo;
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
	pub(crate) type BridgeOf<T> = <T as Config>::Bridge;
	pub(crate) type BridgeTxIdOf<T> = <BridgeOf<T> as TransferTo>::TxId;
	pub(crate) type NetworkIdOf<T> = <BridgeOf<T> as TransferTo>::NetworkId;
	pub(crate) type AddressOf<T> = <BridgeOf<T> as TransferTo>::Address;
	pub(crate) type XCVMInstructionOf<T> =
		XCVMInstruction<XCVMNetwork, AbiEncoded, AccountIdOf<T>, XCVMTransfer>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Executed { instruction: XCVMInstructionOf<T> },
		Spawn { network: XCVMNetwork, network_txs: Vec<BridgeTxIdOf<T>>, program: Vec<u8> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidProgramEncoding,
		UnknownAsset,
		InvalidCallEncoding,
		CallFailed,
		UnknownNetwork,
		MissingNetworkSatellite,
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
		type Bridge: TransferTo<
			AccountId = AccountIdOf<Self>,
			AssetId = AssetIdOf<Self>,
			Balance = BalanceOf<Self>,
			BlockNumber = Self::BlockNumber,
		>;
		type ControlOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn satellite_address)]
	pub type SatelliteAddress<T: Config> =
		StorageMap<_, Blake2_128Concat, XCVMNetwork, AddressOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn network_mapping)]
	pub type NetworkMapping<T: Config> =
		StorageMap<_, Blake2_128Concat, XCVMNetwork, NetworkIdOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		AccountIdOf<T>: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	{
		#[pallet::weight(10_000)]
		pub fn set_satellite(
			origin: OriginFor<T>,
			network: XCVMNetwork,
			address: AddressOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = T::ControlOrigin::ensure_origin(origin)?;
			SatelliteAddress::<T>::insert(network, address);
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn set_network(
			origin: OriginFor<T>,
			network: XCVMNetwork,
			network_id: NetworkIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = T::ControlOrigin::ensure_origin(origin)?;
			NetworkMapping::<T>::insert(network, network_id);
			Ok(().into())
		}

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
						let bridge_network_id =
							NetworkMapping::<T>::get(network).ok_or(Error::<T>::UnknownNetwork)?;
						let network_satellite = SatelliteAddress::<T>::get(network)
							.ok_or(Error::<T>::MissingNetworkSatellite)?;
						let now = frame_system::Pallet::<T>::block_number();
						let network_txs = assets
							.into_iter()
							.map(|(asset, amount)| {
								let concrete_asset = TryInto::<AssetIdOf<T>>::try_into(asset)
									.map_err(|_| Error::<T>::UnknownAsset)?;
								BridgeOf::<T>::transfer_to(
									who.clone(),
									bridge_network_id.clone(),
									network_satellite.clone(),
									concrete_asset,
									amount.saturated_into(),
									false,
									now,
								)
							})
							.collect::<Result<Vec<_>, _>>()?;
						Self::deposit_event(Event::<T>::Spawn {
							network,
							network_txs,
							program: xcvm_protobuf::encode(XCVMProgram {
								instructions: instructions.clone(),
							}),
						});
					},
					XCVMInstruction::Spawn(network, program) =>
						Self::deposit_event(Event::<T>::Spawn {
							network,
							network_txs: Vec::new(),
							program: xcvm_protobuf::encode(XCVMProgram {
								instructions: program.clone(),
							}),
						}),
				}
				Self::deposit_event(Event::<T>::Executed { instruction })
			}

			Ok(().into())
		}
	}
}
