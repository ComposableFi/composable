use super::*;
use composable_traits::{
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
};
use core::borrow::Borrow;
use ibc::{
	applications::transfer::MODULE_ID_STR as IBC_TRANSFER_MODULE_ID,
	core::ics26_routing::context::{Ics26Context, Module, ModuleId, ReaderContext, Router},
};
use primitives::currency::CurrencyId;
use scale_info::prelude::string::ToString;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	router: IbcRouter<T>,
}

impl<T: Config + Send + Sync> Default for Context<T> {
	fn default() -> Self {
		Self { _pd: PhantomData::default(), router: IbcRouter::default() }
	}
}

impl<T: Config + Send + Sync> Context<T> {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IbcRouter<T: Config> {
	pallet_ibc_ping: pallet_ibc_ping::IbcModule<T>,
	ibc_transfer: ics20::IbcModule<T>,
}

impl<T: Config> Default for IbcRouter<T> {
	fn default() -> Self {
		Self {
			pallet_ibc_ping: pallet_ibc_ping::IbcModule::<T>::default(),
			ibc_transfer: ics20::IbcModule::<T>::default(),
		}
	}
}

impl<T: Config + Send + Sync> Router for IbcRouter<T>
where
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation: From<XcmAssetLocation>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation: From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
{
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
		match module_id.borrow().to_string().as_str() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			IBC_TRANSFER_MODULE_ID => Some(&mut self.ibc_transfer),
			&_ => None,
		}
	}

	fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
		matches!(
			module_id.borrow().to_string().as_str(),
			pallet_ibc_ping::MODULE_ID | IBC_TRANSFER_MODULE_ID
		)
	}
}

impl<T: Config + Send + Sync> Ics26Context for Context<T>
where
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation: From<XcmAssetLocation>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation: From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
{
	type Router = IbcRouter<T>;

	fn router(&self) -> &Self::Router {
		&self.router
	}

	fn router_mut(&mut self) -> &mut Self::Router {
		&mut self.router
	}
}

impl<T: Config + Send + Sync> ReaderContext for Context<T> where
	u32: From<<T as frame_system::Config>::BlockNumber>
{
}
