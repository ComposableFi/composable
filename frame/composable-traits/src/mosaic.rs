//! Traits used in the implementation of the Mosaic pallet.

use frame_support::dispatch::DispatchResultWithPostInfo;

/// Trait containing the business logic relevant to managing the Relayer of the Mosaic pallet.
pub trait RelayerInterface {
	type AccountId;
	type AssetId;
	type Balance;
	type BlockNumber;
	type BudgetPenaltyDecayer;
	type NetworkId;
	type NetworkInfo;
	type RelayerConfig;
	type RemoteAssetId;

	/// Confirms that the Relayer will relay a transaction.
	fn accept_transfer(
		asset_id: Self::AssetId,
		from: Self::AccountId,
		network_id: Self::NetworkId,
		remote_asset_id: Self::RemoteAssetId,
		amount: Self::Balance,
	) -> DispatchResultWithPostInfo;

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

/// Trait containing relevant business logic for outgoing transactions.
pub trait TransferTo {
	type AccountId;
	type AssetId;
	type Balance;
	type BlockNumber;

	/// Claims user funds from outgoing transactions in the event the Relayer has not picked the
	/// funds up.
	fn claim_stale_to(
		caller: Self::AccountId,
		asset_id: Self::AssetId,
		to: Self::AccountId,
		now: Self::BlockNumber,
	) -> DispatchResultWithPostInfo;

	/// Creates an outgoing transaction request.
	fn transfer_to(
		caller: Self::AccountId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
		now: Self::BlockNumber,
	) -> DispatchResultWithPostInfo;
}

/// Trait containing relevant business logic for incoming transactions.
pub trait Claim {
	type AccountId;
	type AssetId;
	type BlockNumber;

	/// Collects funds deposited by the Relayer into the owner's account.
	fn claim_to(
		caller: Self::AccountId,
		asset_id: Self::AssetId,
		to: Self::AccountId,
		now: Self::BlockNumber,
	) -> DispatchResultWithPostInfo;
}
