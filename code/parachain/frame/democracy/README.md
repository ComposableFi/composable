# Democracy Pallet

### Fork
Our pallet-democracy is a fork of frame's. Please refer to the docs for most of the functionality. Here we describe the main changes made and how that affects interactions and operations with our version.

#### Changes

- Proposals are identified by their hash + AssetId.
- Voting is performed using the proposals AssetId to identify what token to lock.
- During dispatching, [GovernanceRegistry](../governance-registry) is used to lookup the associated Origin.
- Different Currency traits are used, such as MutateHold.
- Orml traits MultiCurrency, MultiLockableCurrency and MultiReservableCurrency are used to support multiple assets.

#### Public

- These calls can be made from any externally held account capable of creating
a signed extrinsic.

Basic actions with changes:

- `propose` ― Submits a sensitive action in a preferred currency(token), represented as a hash. Requires a deposit in native currency(token).
- `second` ― Signals agreement with a proposal, moves it higher on the proposal queue, and requires a matching deposit in native currency(token) to the original.
- `vote` ― Votes in a referendum with preferred currency(token), either the vote is "Aye" to enact the proposal or "Nay" to keep the status quo.

Administration actions that can be done to any account:
- `reap_vote` ― Remove some account's expired votes.
- `unlock` ― Redetermine the account's balance lock, potentially making tokens available.

Preimage actions:
- `note_preimage` ― Registers the preimage for an upcoming proposal, requires
  a deposit that is returned once the proposal is enacted.
- `note_preimage_operational` ― same but provided by `T::OperationalPreimageOrigin`.
- `note_imminent_preimage` ― Registers the preimage for an upcoming proposal.
  Does not require a deposit, but the proposal must be in the dispatch queue.
- `note_imminent_preimage_operational` ― same but provided by `T::OperationalPreimageOrigin`.
- `reap_preimage` ― Removes the preimage for an expired proposal. Will only
  work under the condition that it's the same account that noted it and
  after the voting period, OR it's a different account after the enactment period.

License: Apache-2.0
