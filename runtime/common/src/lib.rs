#![cfg_attr(not(feature = "std"), no_std)]

pub mod impls;
pub use constants::*;
pub use types::*;

/// Common types of statemint and statemine.
mod types {
	use sp_runtime::traits::{IdentifyAccount, Verify};

	/// An index to a block.
	pub type BlockNumber = u32;

	/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
	pub type Signature = sp_runtime::MultiSignature;

	/// Some way of identifying an account on the chain. We intentionally make it equivalent
	/// to the public key of our transaction signing scheme.
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

	/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
	/// never know...
	pub type AccountIndex = u32;

	/// Balance of an account.
	pub type Balance = u128;

	/// An amount
	pub type Amount = i128;

	/// Index of a transaction in the chain.
	pub type Index = u32;

	/// The address format for describing accounts.
	pub type Address = sp_runtime::MultiAddress<AccountId, AccountIndex>;

	/// A hash of some data used by the chain.
	pub type Hash = sp_core::H256;

	/// Digest item type.
	pub type DigestItem = sp_runtime::generic::DigestItem<Hash>;

	// Aura consensus authority.
	pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;

	/// Council Instance
	pub type CouncilInstance = collective::Instance1;
}

/// Common constants of statemint and statemine
mod constants {
	use super::types::{AccountId, Balance, BlockNumber, CouncilInstance};
	use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
	use frame_system::{EnsureOneOf, EnsureRoot};
	use sp_core::u32_trait::{_1, _2};
	use sp_runtime::Perbill;

	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u64 = 12000;
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// PICA = 12 decimals
	pub const PICA: Balance = 1_000_000_000_000;
	pub const MILLI_PICA: Balance = PICA / 1_000;
	pub const MICRO_PICA: Balance = MILLI_PICA / 1_000;

	/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
	/// used to limit the maximal weight of a single extrinsic.
	// TODO changed to be more in line with statemine
	pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);
	/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
	/// Operational  extrinsics.
	pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

	/// Origin for either root or half of general council
	pub type EnsureRootOrHalfCouncil = EnsureOneOf<
		AccountId,
		EnsureRoot<AccountId>,
		collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilInstance>,
	>;
}
