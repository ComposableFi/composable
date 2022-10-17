use bitflags::bitflags;
use frame_support::pallet_prelude::*;

bitflags! {
	/// Privilege of an account.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Privilege: u32 {
		/// An account we prioritize in liquidation processes.
		/// In a dutch auction for instance, the account will be able to buy at a previous block price,
		/// getting priority over the rest of the users.
		const PRIVILEGED_LIQUIDATOR = 0b0000_0000_0000_0000_0000_0000_0000_0001;
	}
}

impl Default for Privilege {
	fn default() -> Self {
		Self::empty()
	}
}

/// An object from which we can determine the privileges of an account
/// and extract the set of accounts matching a privilege.
pub trait InspectPrivilege {
	type AccountId;

	/// Determine whether the provided account has the given privilege.
	fn has_privilege(account_id: &Self::AccountId, privilege: Privilege) -> bool;
}

/// An object from which we can alter the privileges of an account.
pub trait MutatePrivilege: InspectPrivilege {
	/// Enable the given privilege.
	fn promote(account_id: &Self::AccountId, privilege: Privilege) -> DispatchResult;

	/// Revoke the given privilege.
	/// The implementation should ensure that the user is revoked
	/// from any group that enforce this privilege.
	fn revoke(account_id: &Self::AccountId, privilege: Privilege) -> DispatchResult;
}

pub type PrivilegedGroupOf<T> = <T as InspectPrivilegeGroup>::Group;

/// An privilege group, a set of privileged accounts.
pub trait InspectPrivilegeGroup {
	type AccountId: Copy;
	type GroupId;
	type Group;

	/// Retrieve the privilege that is held for all the members.
	/// Implementation should ensure that this privilege is held for all the members.
	fn privilege(group_id: Self::GroupId) -> Result<Privilege, DispatchError>;

	/// `account_id` is part of `group_id` has specific `privilege`
	fn is_privileged(
		group_id: Self::GroupId,
		account_id: Self::AccountId,
	) -> Result<bool, DispatchError>;

	/// Retrieve a group of privileged accounts.
	fn members(group_id: Self::GroupId) -> Result<Self::Group, DispatchError>;
}

/// An privilege group, a set of privileged accounts.
pub trait MutatePrivilegeGroup: InspectPrivilegeGroup {
	/// Create a group of privileged accounts.
	/// The implementation should ensure that this group is consistent,
	/// meaning that every user within the group has the given privilege.
	fn create(group: Self::Group, privilege: Privilege) -> Result<Self::GroupId, DispatchError>;

	/// Delete a group of privileged accounts.
	fn delete(group_id: Self::GroupId) -> DispatchResult;

	/// Promote the given account into this group.
	/// The implementation should ensure that the account has the required privilege.
	fn promote(group_id: Self::GroupId, account_id: &Self::AccountId) -> DispatchResult;

	/// Revoke the given account from this group.
	fn revoke(group_id: Self::GroupId, account_id: &Self::AccountId) -> DispatchResult;
}
