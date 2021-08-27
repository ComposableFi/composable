
pub trait Lending  {
	type AccountId: core::cmp::Ord;
	type AssetId;
	type Error;
	type Vault: crate::vault::Vault,
}
