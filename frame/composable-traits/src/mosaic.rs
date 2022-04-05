//! Traits used in the implementation of the Mosaic pallet

use frame_support::dispatch::DispatchResultWithPostInfo;

/// Trait containing the business logic relevant to managing the Relayer of the Mosaic pallet
pub trait RelayManager {
	type AccountId;
	type AssetId;
	type Balance;
	type BlockNumber;
	type BudgetPenaltyDecayer;
	type NetworkId;
	type NetworkInfo;
	type RelayerConfig;

	/// Rotates the Relayer Account.
	fn rotate_relayer(
		relayer: Self::RelayerConfig,
		new: Self::AccountId,
		ttl: Self::BlockNumber,
		current_block: Self::BlockNumber,
	);

	/// Burns funds waiting  in incoming transactions that are still unclaimed.
	fn rescind_timelocked_mint(
		asset_id: Self::AssetId,
		account: Self::AccountId,
		untrusted_amount: Self::Balance,
	) -> DispatchResultWithPostInfo;

	/// Sets the Relayer's budget of a specific asset for _incoming_ transactions.
	fn set_budget(
		asset_id: Self::AssetId,
		amount: Self::Balance,
		decay: Self::BudgetPenaltyDecayer,
	);

	/// Sets the supported networks and maximum transaction sizes accepted by the Relayer.
	fn set_network(network_id: Self::NetworkId, network_info: Self::NetworkInfo);

	/// Sets the current Relayer configuration
	fn set_relayer(relayer: Self::AccountId);

	/// Sets the duration, in blocks, of the timelock.
	fn set_timelock_duration(period: Self::BlockNumber);

	/// Mints new tokens into the pallet's wallet. Tokens will be available to users after the
	/// `lock_time` blocks have expired.
	fn timelocked_mint(
		asset_id: Self::AssetId,
		current_block: Self::BlockNumber,
		to: Self::AccountId,
		amount: Self::Balance,
		lock_time: Self::BlockNumber,
	) -> DispatchResultWithPostInfo;
}
