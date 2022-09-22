use super::super::*;
use crate::routing::Context;
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
};
use frame_support::traits::fungibles::{Mutate, Transfer};
use ibc::{
	applications::transfer::{
		context::{BankKeeper, Ics20Context, Ics20Keeper, Ics20Reader},
		error::Error as Ics20Error,
		PORT_ID_STR,
	},
	core::ics24_host::identifier::{ChannelId, PortId},
};
use ibc_trait::{get_channel_escrow_address, ibc_denom_to_foreign_asset_id};
use primitives::currency::CurrencyId;
use sp_runtime::traits::{IdentifyAccount, Zero};

impl<T: Config + Send + Sync> Ics20Reader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type AccountId = <T as transfer::Config>::AccountIdConversion;

	fn get_channel_escrow_address(
		&self,
		port_id: &PortId,
		channel_id: ChannelId,
	) -> Result<<Self as Ics20Reader>::AccountId, Ics20Error> {
		get_channel_escrow_address(port_id, channel_id)?
			.try_into()
			.map_err(|_| Ics20Error::parse_account_failure())
	}

	fn get_port(&self) -> Result<ibc::core::ics24_host::identifier::PortId, Ics20Error> {
		PortId::from_str(PORT_ID_STR)
			.map_err(|e| Ics20Error::invalid_port_id(PORT_ID_STR.to_string(), e))
	}

	fn is_receive_enabled(&self) -> bool {
		transfer::Pallet::<T>::is_receive_enabled()
	}

	fn is_send_enabled(&self) -> bool {
		transfer::Pallet::<T>::is_send_enabled()
	}
}

impl<T: Config + Send + Sync> Ics20Keeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
{
	type AccountId = <T as transfer::Config>::AccountIdConversion;
}

impl<T: Config + Send + Sync> Ics20Context for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
{
	type AccountId = <T as transfer::Config>::AccountIdConversion;
}

impl<T: Config + Send + Sync> BankKeeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::Balance: From<u128>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
{
	type AccountId = <T as transfer::Config>::AccountIdConversion;
	fn mint_coins(
		&mut self,
		account: &Self::AccountId,
		amt: &ibc::applications::transfer::PrefixedCoin,
	) -> Result<(), Ics20Error> {
		let amount: <T as DeFiComposableConfig>::Balance = amt.amount.as_u256().low_u128().into();
		let denom = amt.denom.to_string();
		let foreign_asset_id = ibc_denom_to_foreign_asset_id(&denom);
		// Before minting we need to check if the asset has been registered if not we register the
		// asset before proceeding to mint
		let asset_id = if let Some(asset_id) =
			<T as transfer::Config>::AssetRegistry::location_to_asset(
				foreign_asset_id.clone().into(),
			) {
			asset_id
		} else {
			let local_asset_id = <T as transfer::Config>::CurrencyFactory::create(
				RangeId::IBC_ASSETS,
				<T as DeFiComposableConfig>::Balance::zero(),
			)
			.map_err(|_| {
				Ics20Error::unknown_msg_type("Error creating a local asset id".to_string())
			})?;
			<T as transfer::Config>::AssetRegistry::set_reserve_location(
				local_asset_id.into(),
				foreign_asset_id.into(),
				None,
				None,
			)
			.map_err(|_| {
				Ics20Error::unknown_msg_type("Error registering local asset id".to_string())
			})?;
			transfer::Pallet::<T>::register_asset_id(local_asset_id, denom.as_bytes().to_vec());
			local_asset_id.into()
		};

		<<T as transfer::Config>::MultiCurrency as Mutate<T::AccountId>>::mint_into(
			asset_id.into(),
			&account.clone().into_account(),
			amount,
		)
		.map_err(|_| Ics20Error::invalid_token())?;
		Ok(())
	}

	fn burn_coins(
		&mut self,
		account: &Self::AccountId,
		amt: &ibc::applications::transfer::PrefixedCoin,
	) -> Result<(), Ics20Error> {
		let amount: <T as DeFiComposableConfig>::Balance = amt.amount.as_u256().low_u128().into();
		let denom = amt.denom.to_string();
		let foreign_asset_id = ibc_denom_to_foreign_asset_id(&denom);
		// Token should be registered already if burning a voucher
		let asset_id = if let Some(asset_id) =
			<T as transfer::Config>::AssetRegistry::location_to_asset(foreign_asset_id.into())
		{
			asset_id
		} else {
			return Err(Ics20Error::invalid_token())
		};
		<<T as transfer::Config>::MultiCurrency as Mutate<T::AccountId>>::burn_from(
			asset_id.into(),
			&account.clone().into_account(),
			amount,
		)
		.map_err(|_| Ics20Error::invalid_token())?;
		Ok(())
	}

	fn send_coins(
		&mut self,
		from: &Self::AccountId,
		to: &Self::AccountId,
		amt: &ibc::applications::transfer::PrefixedCoin,
	) -> Result<(), Ics20Error> {
		let amount: <T as DeFiComposableConfig>::Balance = amt.amount.as_u256().low_u128().into();
		// Token should be a native or local asset when the trace path is empty
		let is_local_asset = amt.denom.trace_path().is_empty();
		if is_local_asset {
			let local_asset_id =
				if let Ok(asset_id) = CurrencyId::to_native_id(amt.denom.base_denom().as_str()) {
					asset_id
				} else {
					let asset_id: CurrencyId = amt
						.denom
						.base_denom()
						.as_str()
						.parse::<u128>()
						.map_err(|_| Ics20Error::invalid_token())?
						.into();
					asset_id
				};

			return <<T as transfer::Config>::MultiCurrency as Transfer<T::AccountId>>::transfer(
				local_asset_id.into(),
				&from.clone().into_account(),
				&to.clone().into_account(),
				amount,
				true,
			)
			.map(|_| ())
			.map_err(|_| Ics20Error::invalid_token())
		}
		let denom = amt.denom.to_string();
		let foreign_asset_id = ibc_denom_to_foreign_asset_id(&denom);
		// Token should be registered already if sending an ibc asset
		let asset_id = if let Some(asset_id) =
			<T as transfer::Config>::AssetRegistry::location_to_asset(foreign_asset_id.into())
		{
			asset_id
		} else {
			return Err(Ics20Error::invalid_token())
		};
		<<T as transfer::Config>::MultiCurrency as Transfer<T::AccountId>>::transfer(
			asset_id.into(),
			&from.clone().into_account(),
			&to.clone().into_account(),
			amount,
			true,
		)
		.map_err(|_| Ics20Error::invalid_token())?;
		Ok(())
	}
}
