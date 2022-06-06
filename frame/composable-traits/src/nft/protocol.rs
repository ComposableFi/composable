use composable_support::collections::vec::bounded::BiBoundedVec;
use sp_runtime::{DispatchError, Permill};

/// This trait is aware of NFT and protocols which can be behind NFT
/// It may do complicated interactions and report about complex positions (staking, lending, etc)
pub trait ProtocolNft<AccountId> {
	type AssetId;
	type InstanceId;
	type Balance;
	/// no always may work, for examples  if locked for sale or voting
	fn split_into(
		instance: &Self::InstanceId,
		ratio: Permill,
	) -> Result<BiBoundedVec<Self::InstanceId, 1, 16>, DispatchError>;

	/// If NFT has some original asset behind it, it will be reported here.
	/// As named (nome) in original asset amount. real price and share may vary.
	fn nominal(instance: &Self::InstanceId) -> Option<(Self::AssetId, Self::Balance)>;
}
