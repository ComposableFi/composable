use composable_traits::pool::{
	Bound, 
	WeightsVec,
};
	
use frame_support::{
	pallet_prelude::*,
	sp_runtime::Perquintill,
};
use scale_info::TypeInfo;

// Does not derive Copy as asset_ids is a Vector (i.e. the 
//     data resides on the heap) and thus doesn't derive Copy
#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct PoolInfo<AccountId, CurrencyId> {
	pub manager: AccountId,

	pub assets: 	  Vec<CurrencyId>,
	pub asset_bounds: Bound<u8>,

	pub weights:	   WeightsVec<CurrencyId>,
	pub weight_bounds: Bound<Perquintill>,

	pub deposit_bounds:  Bound<Perquintill>,
	pub withdraw_bounds: Bound<Perquintill>,

	pub transaction_fee: Perquintill,

	pub lp_token_id: CurrencyId,
}
