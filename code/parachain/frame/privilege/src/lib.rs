#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use composable_support::{
		abstractions::{
			counter::Counter,
			utils::{
				decrement::{Decrement, SafeDecrement},
				increment::{Increment, IncrementToMax},
				start_at::ZeroInit,
			},
		},
		error_to_pallet_error,
		math::wrapping_next::WrappingNext,
	};
	use composable_traits::privilege::{
		InspectPrivilege, InspectPrivilegeGroup, MutatePrivilege, MutatePrivilegeGroup, Privilege,
		PrivilegedGroupOf,
	};
	use frame_support::pallet_prelude::*;
	use sp_runtime::{traits::MaybeDisplay, DispatchError};
	use sp_std::fmt::Debug;

	type AccountIdOf<T> = <T as Config>::AccountId;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PrivilegeAdded { account_id: AccountIdOf<T>, privilege: Privilege },
		PrivilegeRemoved { account_id: AccountIdOf<T>, privilege: Privilege },
		GroupCreated { group_id: T::GroupId, privilege: Privilege },
		GroupDeleted { group_id: T::GroupId },
		GroupMemberAdded { group_id: T::GroupId, account_id: AccountIdOf<T> },
		GroupMemberRemoved { group_id: T::GroupId, account_id: AccountIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		// TODO: Rename to `TooManyGroups` (pluralize properly)
		TooManyGroup,
		// TODO: Rename to `TooManyMembers` (pluralize properly)
		TooManyMember,
		GroupNotFound,
		GroupPrivilegeNotHeld,
		NotGroupMember,
		AlreadyGroupMember,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type AccountId: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaybeDisplay
			+ Ord
			+ Default
			+ MaxEncodedLen
			+ TypeInfo
			+ Copy;

		type GroupId: FullCodec
			+ MaxEncodedLen
			+ WrappingNext
			+ Default
			+ Debug
			+ Copy
			+ PartialEq
			+ TypeInfo;

		/// The max number of groups this pallet can handle.
		#[pallet::constant]
		type MaxGroup: Get<u32>;

		/// The max number of member a group can handle.
		#[pallet::constant]
		type MaxMember: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn account_privileges)]
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_types)]
	pub type AccountPrivileges<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Privilege, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn group_privileges)]
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_types)]
	pub type GroupPrivileges<T: Config> =
		StorageMap<_, Blake2_128Concat, T::GroupId, Privilege, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn group_members)]
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_types)]
	pub type GroupMembers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::GroupId,
		BoundedVec<<T as Config>::AccountId, T::MaxMember>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn group_id_last)]
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_types)]
	pub type GroupId<T: Config> = StorageValue<_, T::GroupId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn group_count)]
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_types)]
	pub type GroupCount<T: Config> = StorageValue<
		_,
		u32,
		ValueQuery,
		Counter<ZeroInit, IncrementToMax<T::MaxGroup, TooManyGroup, Error<T>>, SafeDecrement>,
	>;

	error_to_pallet_error!(TooManyGroup,);

	impl<T: Config> InspectPrivilege for Pallet<T> {
		type AccountId = AccountIdOf<T>;

		fn has_privilege(account_id: &Self::AccountId, privilege: Privilege) -> bool {
			AccountPrivileges::<T>::try_get(account_id)
				.map(|account_privileges| account_privileges.contains(privilege))
				.unwrap_or_else(|_| false)
		}
	}

	impl<T: Config> MutatePrivilege for Pallet<T> {
		fn promote(account_id: &Self::AccountId, privilege: Privilege) -> DispatchResult {
			AccountPrivileges::<T>::try_mutate(account_id, |account_privileges| {
				if !account_privileges.contains(privilege) {
					account_privileges.insert(privilege);
					Self::deposit_event(Event::PrivilegeAdded {
						account_id: *account_id,
						privilege,
					});
				}
				Ok(())
			})
		}

		fn revoke(account_id: &Self::AccountId, privilege: Privilege) -> DispatchResult {
			AccountPrivileges::<T>::try_mutate(account_id, |account_privileges| {
				if !account_privileges.is_empty() {
					account_privileges.remove(privilege);
					Self::deposit_event(Event::PrivilegeRemoved {
						account_id: *account_id,
						privilege,
					});
					GroupPrivileges::<T>::iter()
						.filter(|(_, group_privileges)| group_privileges.contains(privilege))
						.for_each(|(group_id, _)| {
							let _ = <Self as MutatePrivilegeGroup>::revoke(group_id, account_id);
						});
				}
				Ok(())
			})
		}
	}

	impl<T: Config> InspectPrivilegeGroup for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type GroupId = T::GroupId;
		type Group = BoundedVec<Self::AccountId, T::MaxMember>;

		fn privilege(group_id: Self::GroupId) -> Result<Privilege, DispatchError> {
			GroupPrivileges::<T>::try_get(group_id).map_err(|_| Error::<T>::GroupNotFound.into())
		}

		fn members(group_id: Self::GroupId) -> Result<PrivilegedGroupOf<Self>, DispatchError> {
			GroupMembers::<T>::try_get(group_id).map_err(|_| Error::<T>::GroupNotFound.into())
		}

		fn is_privileged(
			group_id: Self::GroupId,
			account_id: Self::AccountId,
		) -> Result<bool, DispatchError> {
			let members = Self::members(group_id)?;
			Ok(members.contains(&account_id))
		}
	}

	impl<T: Config> MutatePrivilegeGroup for Pallet<T> {
		fn create(
			group: PrivilegedGroupOf<Self>,
			privilege: Privilege,
		) -> Result<Self::GroupId, DispatchError> {
			GroupId::<T>::try_mutate(|previous_group_id| {
				let group_id = previous_group_id.next();
				*previous_group_id = group_id;

				GroupCount::<T>::increment()?;
				GroupPrivileges::<T>::insert(group_id, privilege);
				// NOTE(hussein-aitlahcen): we don't know if it's correctly sorted at creation,
				// hence we promote member per member.
				GroupMembers::<T>::insert(group_id, BoundedVec::with_bounded_capacity(group.len()));
				Self::deposit_event(Event::GroupCreated { group_id, privilege });

				for member in group {
					<Self as MutatePrivilegeGroup>::promote(group_id, &member)?;
				}

				Ok(group_id)
			})
		}

		fn delete(group_id: Self::GroupId) -> DispatchResult {
			GroupCount::<T>::decrement()?;
			GroupPrivileges::<T>::remove(group_id);
			GroupMembers::<T>::remove(group_id);
			Self::deposit_event(Event::GroupDeleted { group_id });
			Ok(())
		}

		/* NOTE(hussein-aitlahcen):
			I don't know whether promoting a user to a group
			automatically adjust it's privilege to the group?
			The code is currently assuming that there is two distinct steps right now.
			First getting a privilege promotion, and then getting promoted to a group.
		*/
		fn promote(group_id: Self::GroupId, account_id: &Self::AccountId) -> DispatchResult {
			let privilege = Self::privilege(group_id)?;
			ensure!(Self::has_privilege(account_id, privilege), Error::<T>::GroupPrivilegeNotHeld);
			GroupMembers::<T>::try_mutate(group_id, |group| {
				// Match to make it clear that in case of Ok => already present
				match group.binary_search(account_id) {
					Ok(_) => Err(Error::<T>::AlreadyGroupMember.into()),
					Err(i) => {
						group.try_insert(i, *account_id).map_err(|_| Error::<T>::TooManyMember)?;
						Self::deposit_event(Event::GroupMemberAdded {
							group_id,
							account_id: *account_id,
						});
						Ok(())
					},
				}
			})
		}

		/* NOTE(hussein-aitlahcen):
			 Pretty much the same comment as the one for `promote`.
			 Should the user get it's privileges adjusted when getting kicked out of a group?
			 Currently it's not the case.
		*/
		fn revoke(group_id: Self::GroupId, account_id: &Self::AccountId) -> DispatchResult {
			GroupMembers::<T>::try_mutate(group_id, |group| {
				/* NOTE(hussein-aitlahcen):
				   No hashset on-chain, is there a better way?
				   This shouldn't happen so much, probably only done by governance.
				*/
				let index =
					group.binary_search(account_id).map_err(|_| Error::<T>::NotGroupMember)?;
				group.remove(index);
				Self::deposit_event(Event::GroupMemberRemoved {
					group_id,
					account_id: *account_id,
				});
				Ok(())
			})
		}
	}
}
