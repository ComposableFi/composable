# Privilege

This pallet implements the privilege traits.
It currently consist of a simple storage, mapping accounts/groups to their privileges/members.
It can be wrapped later in a privilege-governance pallet that would redirect calls to votes,
or any other pallet which would operate filter on the privilege traits.

Arbitrary implementation behavior:
- Assuming we have a group A enforcing a privilege P, adding a user X to A is impossible (P not held by A after the operation).
- Assuming we have a group A enforcing a privilege P, revoking a user X from A is not going to revoke his privilege P.

An alternative implementation would be that promoting/revoking a user in/from a group automatically adjust its privileges.
