use std::collections::btree_set::IntoIter;

pub use pallet::*;
pub use pallet::Error;

type AccoindIdOf<T> = <T as frame_system::Config>::AccountId;

#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	Hash,
	codec::Encode,
	codec::Decode,
	scale_info::TypeInfo,
	Ord, PartialOrd
)]
pub struct ChainInfo{
	pub chain_id : u128,
	pub channel_id : u64,        //for packet or memo
	pub timestamp : Option<u64>, //for packet
	pub height : Option<u64>,  //for memo packet message forwarding
	pub retries : Option<u64>, //for memo packet message forwarding
	pub timeout : Option<u64>, //for memo packet message forwarding
}

#[derive(Serialize, Debug)]
struct MemoForward{
	receiver: String,
	port: String,
	channel: String,
	timeout: String,
	retries: u64,
	next: Option<Box<MemoForward>>
}

#[derive(Serialize, Debug)]
struct MemoData{
	forward: MemoForward
}

impl MemoData{
	fn new(iter : IntoIter<ChainInfo>) -> Option<Self>{
		let mut chain_info_vec: Vec<_> = iter.collect();
		chain_info_vec.reverse();

		let mut memo_data: Option<MemoData> = None;
		for i in chain_info_vec {
			let new_memo = MemoData {
				forward: MemoForward {
					receiver: String::from("TODO!!!"),
					port: String::from("transfer"),
					channel: String::from(format!("channel-{}", i.channel_id)),
					timeout: String::from(i.timeout.unwrap_or_default().to_string()),
					retries: i.retries.unwrap_or_default(),
					next: memo_data.map(|x| Box::new(x.forward)), // memo_data is boxed here
				},
			};
			memo_data = Some(new_memo);
		}
		memo_data
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use frame_support::{pallet_prelude::*, BoundedBTreeSet};
	use pallet_ibc::{MultiAddress, TransferParams};
	use ibc_primitives::Timeout as IbcTimeout;
	use std::str::FromStr;

	/// ## Configuration
	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_ibc::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type PalletInstanceId: Get<u8>;

		#[pallet::constant]
		type MaxMultihopCount: Get<u32>;
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

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type ChainIdToMiltihopRoutePath<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128, //chain id
		BoundedBTreeSet<ChainInfo, T::MaxMultihopCount>, //route to forward
		ValueQuery,
	>;
	

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}


	

	// The pallet's dispatchable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
    }

	use frame_system::{RawOrigin};
	
use xcm::latest::prelude::*;
    impl<T : Config> MultiCurrencyCallback<T> for Pallet<T>
	where 
	T: Send + Sync,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>,
	 {
		fn deposit_asset(asset: &MultiAsset, location: &MultiLocation, context: &XcmContext, deposit_result : Result, asset_id : Option<<T as pallet_ibc::Config>::AssetId>){
			let id = match location {
				MultiLocation { parents: 0, interior: X4(
					PalletInstance( pallet_id ) , 
					AccountId32{ id, network: None }, 
					GeneralIndex( chain_id ), 
					AccountId32{ id: _, network: None } ) } if *pallet_id == T::PalletInstanceId::get() => Some((*id, *chain_id)),
				_ => None,
			};
			let Some((id, chain_id)) = id else{
				//does not match the pattern of multihop
				return;
			};

			let Ok(_) = deposit_result else {
				//deposit does not executed propertly. nothing todo. assets will stay in the account id address
				return;
			};

			let Ok(route) = ChainIdToMiltihopRoutePath::<T>::try_get(chain_id) else {
				//route does not exist
				return;
			};


			let mut chain_info_iter = route.into_iter();

			let Some(chain_info) = chain_info_iter.next() else{
				//route does not exist
				return;
			};
			
			let account_id = MultiAddress::<AccoindIdOf<T>>::Raw(id.to_vec());
			let transfer_params = TransferParams::<AccoindIdOf<T>>{
				to : account_id,
				source_channel : chain_info.channel_id,
				timeout : IbcTimeout::Offset{ timestamp : chain_info.timestamp, height : chain_info.height}
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

			//todo construct memo from xcm multilocation!!!
			let mut memo : Option<<T as pallet_ibc::Config>::MemoMessage> = None;

			let memo_data = MemoData::new(chain_info_iter);
			match memo_data{
				Some(memo_data) => {
					let memo_str = format!("{:?}", memo_data); //create a string memo
			
					let memo_result = <T as pallet_ibc::Config>::MemoMessage::from_str(&memo_str);
					
					match memo_result{
						Ok(m) => { memo = Some(m) },
						Err(e) => {
							//todo memo failed. need to stop multi hop and emit event.
							//track event with error?
							//TODO should we continew to send IBC if failed to consturct memo for message forwarding?
						}
					};
				}
				_ => {}
			}

			let result = pallet_ibc::Pallet::<T>::transfer(
				signed_account_id.into(), 
				transfer_params, 
				asset_id.unwrap(), 
				(*amount).into(), 
				memo);
			match result{
				Ok(_) => {
					//todo emit success multi hop ibc transfer event
				},
				Err(e) => {
					//todo emit error
				}
			}
		}
    }
}


use serde::Serialize;
use xcm::v3::*;
pub trait MultiCurrencyCallback<T : Config>{
	fn deposit_asset(asset: &MultiAsset, location: &MultiLocation, context: &XcmContext, deposit_result : Result, asset_id : Option<<T as pallet_ibc::Config>::AssetId>);
		//check result, unwrap memo if exists and execute ibc packet
	
}