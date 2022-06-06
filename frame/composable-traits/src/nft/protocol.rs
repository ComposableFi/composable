use composable_support::collections::vec::bounded::BiBoundedVec;
use sp_runtime::{DispatchError, Permill};

/// protocol nft is aware of NFT and protocols, so it manages to do complex intercation
pub trait ProtocolNft<AccountId> {
	type AssetId;
	type InstanceId;
	type Balance;
	/// no always may work, for examples  if locked for sale or voting
	fn split_into(
		instance: &Self::InstanceId,
		parts: BiBoundedVec<Permill, 1, 16>,
	) -> Result<BiBoundedVec<Self::InstanceId, 1, 16>, DispatchError>;

	/// if nft has asset behind it, it will be returned   
	fn nominal(instance: &Self::InstanceId) -> Option<(Self::AssetId, Self::Balance)>;
}
