pub use pallet::*;
pub use pallet::Error;

type AccoindIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use frame_support::pallet_prelude::*;

	/// ## Configuration
	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_ibc::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

	// The pallet's events
	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		
	}

	#[pallet::error]
	pub enum Error<T> {
		Error1,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}


	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's dispatchable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
    }

	use frame_system::{RawOrigin};
	
// use frame_system::Config;
use xcm::latest::prelude::*;
    impl<T : Config
	// frame_system::Config + pallet_ibc::Config + Send + Sync,
// 	T: ,
// T: Send
// `T: Sync`
// `frame_support::sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>`
// `u32: From<<T as frame_system::Config>::BlockNumber>
	> MultiCurrencyCallback<T> for Pallet<T>
	where 
	T: Send + Sync,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>,
	 {
		fn deposit_asset(asset: &MultiAsset, location: &MultiLocation, context: &XcmContext, deposit_result : Result, asset_id : Option<<T as pallet_ibc::Config>::AssetId>){
			let id = match location {
				MultiLocation { parents: 0, interior: X4(
					PalletInstance(_), 
					AccountId32{ id, network: None }, 
					GeneralIndex(_), 
					AccountId32{ id: _, network: None } ) } => Some(id.clone()),
				_ => None,
			};
			let Some(id) = id else{
				//does not match the pattern of multihop
				return;
			};

			let Ok(_) = deposit_result else {
				//deposit does not executed propertly. nothing todo. assets will stay in the account id address
				return;
			};

			let account_id = pallet_ibc::MultiAddress::<AccoindIdOf<T>>::Raw(id.to_vec());
			let transfer_params = pallet_ibc::TransferParams::<AccoindIdOf<T>>{
				to : account_id,
				source_channel : 1,
				timeout : ibc_primitives::Timeout::Offset{ timestamp : Some(1), height : Some(1)}
			};

			let account = sp_runtime::AccountId32::new(
				id
			);
			let mut to32 : &[u8] = sp_runtime::AccountId32::as_ref(&account);
			let account_id = T::AccountId::decode(&mut to32).unwrap();
			let signed_account_id = RawOrigin::Signed(account_id);
			
			// let 
			let Fungibility::Fungible(ref amount) = asset.fun else{
				return;
				//do not support non fungible.
			};

			let result = pallet_ibc::Pallet::<T>::transfer(signed_account_id.into(), transfer_params, asset_id.unwrap(), (*amount).into(), None);
		}
    }
}


use xcm::v3::*;
pub trait MultiCurrencyCallback<T : Config>{
	fn deposit_asset(asset: &MultiAsset, location: &MultiLocation, context: &XcmContext, deposit_result : Result, asset_id : Option<<T as pallet_ibc::Config>::AssetId>);
		//check result, unwrap memo if exists and execute ibc packet
	
}