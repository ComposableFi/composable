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
#![deny(clippy::unseparated_literal_suffix, unused_imports, dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(test)]
mod runtime;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_support::abstractions::{
		nonce::Nonce,
		utils::{
			increment::{Increment, SafeIncrement},
			start_at::ZeroInit,
		},
	};
	pub use composable_traits::{
		assets::Asset,
		currency::{
			AssetExistentialDepositInspect, AssetRatioInspect, BalanceLike, Exponent,
			Rational64 as Rational,
		},
		defi::Ratio,
		xcm::assets::{ForeignMetadata, RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
	};

	pub use codec::{Decode, Encode};

	use composable_traits::{
		assets::{
			AssetInfo, AssetInfoUpdate, BiBoundedAssetName, BiBoundedAssetSymbol, GenerateAssetId,
		},
		storage::UpdateValue,
		xcm::assets::{CosmWasmIssuance, PermissionlessAsset},
	};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{fungibles::Mutate, Currency, EnsureOrigin, ExistenceRequirement::KeepAlive},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::fmt::Debug;

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Factory to create new lp-token.
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Into<u128>
			+ Debug
			+ Default
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;

		/// Foreign location of an asset
		type Location: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ TypeInfo
			+ MaxEncodedLen;

		type Balance: BalanceLike + From<u128>;

		type Currency: Currency<Self::AccountId, Balance = Self::Balance>;

		type AssetsRegistry: RemoteAssetRegistryMutate<
				AssetId = Self::AssetId,
				AssetNativeLocation = Self::Location,
				Balance = Self::Balance,
			> + GenerateAssetId<AssetId = Self::AssetId>
			+ AssetExistentialDepositInspect<AssetId = Self::AssetId, Balance = Self::Balance>;

		/// Asset creation fee goes to treasury.
		type TreasuryAccount: Get<Self::AccountId>;

		// Set asset creation fee origin
		type SetFeeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		type Assets: Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Asset Nonce
	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // Allow for `ValueQuery` because of nonce
	pub type AssetNonce<T: Config> =
		StorageValue<_, u64, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// Fee to pay for new asset registry
	#[pallet::storage]
	pub type AssetCreationFee<T: Config> = StorageValue<_, T::Balance, OptionQuery>;

	/// Asset admin
	#[pallet::storage]
	pub type AssetAdmin<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, T::AccountId, OptionQuery>;

	/// Is Asset CosmWasm Asset
	#[pallet::storage]
	pub type IsCosmWasmAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, (), OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewCreationFeeSet(T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		BadOrigin,
		AssetNotFound,
		NotAssetAdmin,
		BadDecimals,
		TransferError,
		CreationFeeIsNotSet,
		NotCosmWasmAsset,
		CosmWasmAsset,
		BadAssetId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an asset for bridge transfer
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_asset(
			origin: OriginFor<T>,
			location: Option<T::Location>,
			name: Option<BiBoundedAssetName>,
			symbol: Option<BiBoundedAssetSymbol>,
			decimals: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let protocol_id = (Pallet::<T>::index() as u32).to_be_bytes();

			let asset_id = <T::AssetsRegistry as GenerateAssetId>::generate_asset_id(
				protocol_id,
				AssetNonce::<T>::increment().expect("Does not exceed u64::MAX"),
			);
			<Self as PermissionlessAsset>::register_asset(
				who, asset_id, location, name, symbol, decimals, false,
			)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_cosmwasm_asset(
			origin: OriginFor<T>,
			name: Option<BiBoundedAssetName>,
			symbol: Option<BiBoundedAssetSymbol>,
			decimals: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let protocol_id = (Pallet::<T>::index() as u32).to_be_bytes();

			let asset_id = <T::AssetsRegistry as GenerateAssetId>::generate_asset_id(
				protocol_id,
				AssetNonce::<T>::increment().expect("Does not exceed u64::MAX"),
			);
			<Self as PermissionlessAsset>::register_asset(
				who, asset_id, None, name, symbol, decimals, true,
			)?;
			Ok(())
		}

		/// Creates an asset for bridge transfer
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::set_creation_fee())]
		pub fn set_creation_fee(origin: OriginFor<T>, fee_amount: T::Balance) -> DispatchResult {
			T::SetFeeOrigin::ensure_origin(origin).map_err(|_| Error::<T>::BadOrigin)?;
			AssetCreationFee::<T>::put(fee_amount);
			Self::deposit_event(Event::NewCreationFeeSet(fee_amount));
			Ok(())
		}

		/// Update stored asset information.
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::update_asset())]
		pub fn update_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			name: UpdateValue<Option<BiBoundedAssetName>>,
			symbol: UpdateValue<Option<BiBoundedAssetSymbol>>,
			decimals: UpdateValue<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let asset_admin =
				AssetAdmin::<T>::try_get(asset_id).map_err(|_| Error::<T>::AssetNotFound)?;
			ensure!(asset_admin == who, Error::<T>::NotAssetAdmin);
			let mut decimals_param = UpdateValue::Ignore;
			if let UpdateValue::Set(decimals_update) = decimals {
				ensure!(decimals_update > 0 && decimals_update < 30, Error::<T>::BadDecimals);
				decimals_param = UpdateValue::Set(Some(decimals_update));
			}

			let asset_info = AssetInfoUpdate {
				name,
				symbol,
				decimals: decimals_param,
				existential_deposit: UpdateValue::Ignore,
				ratio: UpdateValue::Ignore,
			};
			<T::AssetsRegistry as RemoteAssetRegistryMutate>::update_asset(asset_id, asset_info)?;
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::update_asset_location())]
		pub fn update_asset_location(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			location: Option<T::Location>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let asset_admin =
				AssetAdmin::<T>::try_get(asset_id).map_err(|_| Error::<T>::AssetNotFound)?;
			ensure!(IsCosmWasmAsset::<T>::get(asset_id).is_none(), Error::<T>::CosmWasmAsset);
			ensure!(asset_admin == who, Error::<T>::NotAssetAdmin);
			<T::AssetsRegistry as RemoteAssetRegistryMutate>::set_reserve_location(
				asset_id, location,
			)?;
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::mint_cosmwasm())]
		pub fn mint_cosmwasm(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let asset_admin =
				AssetAdmin::<T>::try_get(asset_id).map_err(|_| Error::<T>::AssetNotFound)?;
			ensure!(asset_admin == who, Error::<T>::NotAssetAdmin);
			<Self as CosmWasmIssuance>::mint_into(asset_id, dest, amount)?;
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::burn_cosmwasm())]
		pub fn burn_cosmwasm(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let asset_admin =
				AssetAdmin::<T>::try_get(asset_id).map_err(|_| Error::<T>::AssetNotFound)?;
			ensure!(asset_admin == who, Error::<T>::NotAssetAdmin);
			<Self as CosmWasmIssuance>::burn_from(asset_id, dest, amount)?;
			Ok(())
		}
	}

	impl<T: Config> PermissionlessAsset for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type AssetNativeLocation = T::Location;
		type Balance = T::Balance;

		fn register_asset(
			account_id: Self::AccountId,
			asset_id: Self::AssetId,
			location: Option<T::Location>,
			name: Option<BiBoundedAssetName>,
			symbol: Option<BiBoundedAssetSymbol>,
			decimals: u8,
			is_cosmwasm: bool,
		) -> DispatchResult {
			ensure!(decimals > 0 && decimals < 30, Error::<T>::BadDecimals);

			ensure!(
				<T::AssetsRegistry as AssetExistentialDepositInspect>::existential_deposit(
					asset_id
				)
				.is_err(),
				Error::<T>::BadAssetId
			);

			let asset_info: AssetInfo<T::Balance> = AssetInfo {
				name,
				symbol,
				decimals: Some(decimals),
				existential_deposit: T::Balance::from(1_u128),
				ratio: None,
			};
			let fee_amount = AssetCreationFee::<T>::get().ok_or(Error::<T>::CreationFeeIsNotSet)?;
			let result = T::Currency::transfer(
				&account_id,
				&T::TreasuryAccount::get(),
				fee_amount,
				KeepAlive,
			);
			ensure!(result.is_ok(), Error::<T>::TransferError);

			<T::AssetsRegistry as RemoteAssetRegistryMutate>::register_asset(
				asset_id, location, asset_info,
			)?;

			AssetAdmin::<T>::insert(&asset_id, account_id);
			if is_cosmwasm {
				IsCosmWasmAsset::<T>::insert(&asset_id, ());
			}

			Ok(())
		}
	}
	impl<T: Config> CosmWasmIssuance for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		fn mint_into(
			asset_id: Self::AssetId,
			dest: Self::AccountId,
			amount: Self::Balance,
		) -> DispatchResult {
			IsCosmWasmAsset::<T>::try_get(asset_id).map_err(|_| Error::<T>::NotCosmWasmAsset)?;
			T::Assets::mint_into(asset_id, &dest, amount)?;
			Ok(())
		}
		fn burn_from(
			asset_id: Self::AssetId,
			dest: Self::AccountId,
			amount: Self::Balance,
		) -> DispatchResult {
			IsCosmWasmAsset::<T>::try_get(asset_id).map_err(|_| Error::<T>::NotCosmWasmAsset)?;
			T::Assets::burn_from(asset_id, &dest, amount)?;
			Ok(())
		}
	}
}
