//! API extracted from pallet-proxy.

use frame_support::pallet_prelude::*;

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	Governance,
	CancelProxy,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
/// TODO (vim): Upstream the following APIs to Substrate/pallet-proxy and use.
/// The parameters under which a particular account has a proxy relationship with some other
/// account.
#[derive(
	Encode,
	Decode,
	Clone,
	Copy,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
)]
pub struct ProxyDefinition<AccountId, ProxyType, BlockNumber> {
	/// The account which may act on behalf of another.
	pub delegate: AccountId,
	/// A value defining the subset of calls that it is allowed to make.
	pub proxy_type: ProxyType,
	/// The number of blocks that an announcement must be in place for before the corresponding
	/// call may be dispatched. If zero, then no announcement is needed.
	pub delay: BlockNumber,
}

/// API into pallet-account-proxy. Provides functions to manage delegation of operations of
/// one account to another.
pub trait AccountProxy {
	type AccountId;

	type ProxyType;

	type BlockNumber;

	/// Register a proxy account for the delegator that is able to make calls on its behalf.
	///
	/// Parameters:
	/// - `delegator`: The delegator account.
	/// - `delegatee`: The account that the `delegator` would like to make a proxy.
	/// - `proxy_type`: The permissions allowed for this proxy account.
	/// - `delay`: The announcement period required of the initial proxy. Will generally be
	/// zero.
	fn add_proxy_delegate(
		delegator: &Self::AccountId,
		delegatee: Self::AccountId,
		proxy_type: Self::ProxyType,
		delay: Self::BlockNumber,
	) -> DispatchResult;

	/// Unregister a proxy account for the delegator.
	///
	/// Parameters:
	/// - `delegator`: The delegator account.
	/// - `delegatee`: The account that the `delegator` would like to make a proxy.
	/// - `proxy_type`: The permissions allowed for this proxy account.
	/// - `delay`: The announcement period required of the initial proxy. Will generally be
	/// zero.
	fn remove_proxy_delegate(
		delegator: &Self::AccountId,
		delegatee: Self::AccountId,
		proxy_type: Self::ProxyType,
		delay: Self::BlockNumber,
	) -> DispatchResult;

	/// Find any existing proxy between the given accounts.
	///
	/// Parameters:
	/// - `real`: The delegator account.
	/// - `delegate`: The account that the `delegator` has a proxy to.
	/// - `force_proxy_type`: Only find proxies of this type.
	fn find_proxy(
		real: &Self::AccountId,
		delegate: &Self::AccountId,
		force_proxy_type: Option<Self::ProxyType>,
	) -> Result<ProxyDefinition<Self::AccountId, Self::ProxyType, Self::BlockNumber>, DispatchError>;
}
