//! # Clearing House
//!
//! Common traits for clearing house implementations
use frame_support::pallet_prelude::DispatchError;

pub trait ClearingHouse {
	/// The trader's account identifier type
	type AccountId;
	/// The asset identifier type
	type AssetId;
	/// The balance type for an account
	type Balance;
	/// The identifier type for each market
	type MarketId;
	/// Parameters for creating and initializing a new vAMM instance.
	type VammParams;

	/// Add margin to a user's account
	///
	/// Assumes margin account is unique to each wallet address, i.e., there's only one margin
	/// account per user.
	fn add_margin(
		acc: &Self::AccountId,
		asset: Self::AssetId,
		amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Create a new perpetuals market
	/// 
	/// ## Parameters
	/// - `asset`: Asset id of the underlying for the derivatives market
	/// - `vamm_params`: Parameters for creating and initializing the vAMM for price discovery
	/// 
	/// ## Returns
	/// The new market id, if successful
	fn create_market(
		asset: Self::AssetId,
		vamm_params: Self::VammParams,
	) -> Result<Self::MarketId, DispatchError>;
}
