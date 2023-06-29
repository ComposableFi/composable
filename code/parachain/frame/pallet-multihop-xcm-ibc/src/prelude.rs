pub use composable_traits::{
	assets::Asset,
	currency::{
		AssetExistentialDepositInspect, AssetRatioInspect, BalanceLike, Exponent,
		Rational64 as Rational,
	},
	defi::Ratio,
	xcm::assets::{ForeignMetadata, RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
};

pub use codec::{Decode, Encode, FullCodec};

// use serde::Serialize;
// use crate::{Config, Error, ChainInfo};
// use xcm::v3::*;
// use frame_support::BoundedVec;
// pub trait MultiCurrencyCallback<T: Config> {
// 	fn deposit_asset(
// 		asset: &xcm::latest::MultiAsset,
// 		location: &xcm::latest::MultiLocation,
// 		context: &xcm::latest::XcmContext,
// 		deposit_result: xcm::latest::Result,
// 		asset_id: Option<<T as pallet_ibc::Config>::AssetId>,
// 	) -> core::result::Result<(), Error<T>>;
// 	//check result, unwrap memo if exists and execute ibc packet
// }

// #[derive(serde::Serialize, Debug)]
// pub struct MemoForward {
// 	receiver: String,
// 	port: String,
// 	channel: String,
// 	timeout: String,
// 	retries: u64,
// 	next: Option<Box<MemoForward>>,
// }

// #[derive(serde::Serialize, Debug)]
// pub struct MemoData {
// 	forward: MemoForward,
// }

// impl MemoData {
// 	/// Support only addresses from cosmos ecosystem based on bech32.
// 	pub fn new<T: Config>(
// 		mut vec: Vec<(ChainInfo, BoundedVec<u8, T::ChainNameVecLimit>, [u8; 32])>,
// 	) -> core::result::Result<Option<Self>, Error<T>> {
// 		// vec.reverse();
// 		// let mut memo_data: Option<MemoData> = None;
// 		// for (i, name, address) in vec {
// 		// 	let result: core::result::Result<Vec<bech32::u5>, bech32::Error> =
// 		// 		address.into_iter().map(bech32::u5::try_from_u8).collect();
// 		// 	let data =
// 		// 		result.map_err(|_| Error::<T>::IncorrectAddress { chain_id: i.chain_id as u8 })?;

// 		// 	let name = String::from_utf8(name.into())
// 		// 		.map_err(|_| Error::<T>::IncorrectChainName { chain_id: i.chain_id as u8 })?;
// 		// 	let result_address = bech32::encode(&name, data.clone()).map_err(|_| {
// 		// 		Error::<T>::FailedToEncodeBech32Address { chain_id: i.chain_id as u8 }
// 		// 	})?;

// 		// 	let new_memo = MemoData {
// 		// 		forward: MemoForward {
// 		// 			receiver: result_address,
// 		// 			port: String::from("transfer"),
// 		// 			channel: String::from(format!("channel-{}", i.channel_id)),
// 		// 			timeout: String::from(i.timeout.unwrap_or_default().to_string()),
// 		// 			retries: i.retries.unwrap_or_default(),
// 		// 			next: memo_data.map(|x| Box::new(x.forward)), // memo_data is boxed here
// 		// 		},
// 		// 	};
// 		// 	memo_data = Some(new_memo);
// 		// }
// 		// Ok(memo_data)
// 		// core::prelude::v1::None
// 		todo!()
// 	}
// }
