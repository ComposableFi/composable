<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.066618Z -->

# Democracy Pallet Extrinsics

## Propose

[`propose`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.propose)

Propose a sensitive action to be taken.

The dispatch origin of this call must be *Signed* and the sender must
have funds to cover the deposit.

* `proposal_hash`: The hash of the proposal preimage.
* `asset_id` : The asset id of the proposal preimage.
* `value`: The amount of deposit (must be at least `MinimumDeposit`).

Emits `Proposed`.

Weight: `O(p)`

## Second

[`second`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.second)

Signals agreement with a particular proposal.

The dispatch origin of this call must be *Signed* and the sender
must have funds to cover the deposit, equal to the original deposit.

* `proposal`: The index of the proposal to second.
* `seconds_upper_bound`: an upper bound on the current number of seconds on this
  proposal. Extrinsic is weighted according to this value with no refund.

Weight: `O(S)` where S is the number of seconds a proposal already has.

## Vote

[`vote`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.vote)

Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;
otherwise it is a vote to keep the status quo.

The dispatch origin of this call must be *Signed*.

* `ref_index`: The index of the referendum to vote for.
* `vote`: The vote configuration.

Weight: `O(R)` where R is the number of referendums the voter has voted on.

## Emergency Cancel

[`emergency_cancel`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.emergency_cancel)

Schedule an emergency cancellation of a referendum. Cannot happen twice to the same
referendum.

The dispatch origin of this call must be `CancellationOrigin`.

-`ref_index`: The index of the referendum to cancel.

Weight: `O(1)`.

## External Propose

[`external_propose`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.external_propose)

Schedule a referendum to be tabled once it is legal to schedule an external
referendum.

The dispatch origin of this call must be `ExternalOrigin`.

* `proposal_hash`: The preimage hash of the proposal.
* `asset_id` : The asset id of the proposal.

Weight: `O(V)` with V number of vetoers in the blacklist of proposal.
Decoding vec of length V. Charged as maximum

## External Propose Majority

[`external_propose_majority`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.external_propose_majority)

Schedule a majority-carries referendum to be tabled next once it is legal to schedule
an external referendum.

The dispatch of this call must be `ExternalMajorityOrigin`.

* `proposal_hash`: The preimage hash of the proposal.
* `asset_id` : The asset id of the proposal.

Unlike `external_propose`, blacklisting has no effect on this and it may replace a
pre-scheduled `external_propose` call.

Weight: `O(1)`

## External Propose Default

[`external_propose_default`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.external_propose_default)

Schedule a negative-turnout-bias referendum to be tabled next once it is legal to
schedule an external referendum.

The dispatch of this call must be `ExternalDefaultOrigin`.

* `proposal_hash`: The preimage hash of the proposal.
* `asset_id` : The asset id of the proposal preimage.

Unlike `external_propose`, blacklisting has no effect on this and it may replace a
pre-scheduled `external_propose` call.

Weight: `O(1)`

## Fast Track

[`fast_track`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.fast_track)

Schedule the currently externally-proposed majority-carries referendum to be tabled
immediately. If there is no externally-proposed referendum currently, or if there is one
but it is not a majority-carries referendum then it fails.

The dispatch of this call must be `FastTrackOrigin`.

* `proposal_hash`: The hash of the current external proposal.
* `asset_id` : The asset id of the proposal.
* `voting_period`: The period that is allowed for voting on this proposal. Increased to
  `FastTrackVotingPeriod` if too low.
* `delay`: The number of block after voting has ended in approval and this should be
  enacted. This doesn't have a minimum amount.

Emits `Started`.

Weight: `O(1)`

## Veto External

[`veto_external`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.veto_external)

Veto and blacklist the external proposal hash.

The dispatch origin of this call must be `VetoOrigin`.

* `proposal_hash`: The preimage hash of the proposal to veto and blacklist.
* `asset_id` : The asset id of the proposal.

Emits `Vetoed`.

Weight: `O(V + log(V))` where V is number of `existing vetoers`

## Cancel Referendum

[`cancel_referendum`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.cancel_referendum)

Remove a referendum.

The dispatch origin of this call must be *Root*.

* `ref_index`: The index of the referendum to cancel.

### Weight: `O(1)`.

## Cancel Queued

[`cancel_queued`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.cancel_queued)

Cancel a proposal queued for enactment.

The dispatch origin of this call must be *Root*.

* `which`: The index of the referendum to cancel.

Weight: `O(D)` where `D` is the items in the dispatch queue. Weighted as `D = 10`.

## Delegate

[`delegate`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.delegate)

Delegate the voting power (with some given conviction) of the sending account.

The balance delegated is locked for as long as it's delegated, and thereafter for the
time appropriate for the conviction's lock period.

The dispatch origin of this call must be *Signed*, and the signing account must either:

* be delegating already; or

* have no voting activity (if there is, then it will need to be removed/consolidated
  through `reap_vote` or `unvote`).

* `to`: The account whose voting the `target` account's voting power will follow.

* `asset_id` : The asset id to be used in delegating.

* `conviction`: The conviction that will be attached to the delegated votes. When the
  account is undelegated, the funds will be locked for the corresponding period.

* `balance`: The amount of the account's balance to be used in delegating. This must not
  be more than the account's current balance.

Emits `Delegated`.

Weight: `O(R)` where R is the number of referendums the voter delegating to has
voted on. Weight is charged as if maximum votes.

## Undelegate

[`undelegate`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.undelegate)

Undelegate the voting power of the sending account.

Tokens may be unlocked following once an amount of time consistent with the lock period
of the conviction with which the delegation was issued.

The dispatch origin of this call must be *Signed* and the signing account must be
currently delegating.

* `asset_id` : The asset id to be used in delegating.

Emits `Undelegated`.

Weight: `O(R)` where R is the number of referendums the voter delegating to has
voted on. Weight is charged as if maximum votes.

## Clear Public Proposals

[`clear_public_proposals`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.clear_public_proposals)

Clears all public proposals.

The dispatch origin of this call must be *Root*.

Weight: `O(1)`.

## Note Preimage

[`note_preimage`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.note_preimage)

Register the preimage for an upcoming proposal. This doesn't require the proposal to be
in the dispatch queue but does require a deposit, returned once enacted.

The dispatch origin of this call must be *Signed*.

* `encoded_proposal`: The preimage of a proposal.
* `asset_id` : The asset id of a proposal.

Emits `PreimageNoted`.

Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit).

## Note Preimage Operational

[`note_preimage_operational`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.note_preimage_operational)

Same as `note_preimage` but origin is `OperationalPreimageOrigin`.

## Note Imminent Preimage

[`note_imminent_preimage`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.note_imminent_preimage)

Register the preimage for an upcoming proposal. This requires the proposal to be
in the dispatch queue. No deposit is needed. When this call is successful, i.e.
the preimage has not been uploaded before and matches some imminent proposal,
no fee is paid.

The dispatch origin of this call must be *Signed*.

* `encoded_proposal`: The preimage of a proposal.
* `asset_id` : The asset id of a proposal.

Emits `PreimageNoted`.

Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit).

## Note Imminent Preimage Operational

[`note_imminent_preimage_operational`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.note_imminent_preimage_operational)

Same as `note_imminent_preimage` but origin is `OperationalPreimageOrigin`.

## Reap Preimage

[`reap_preimage`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.reap_preimage)

Remove an expired proposal preimage and collect the deposit.

The dispatch origin of this call must be *Signed*.

* `proposal_hash`: The preimage hash of a proposal.
* `asset_id` : The asset id of a proposal.
* `proposal_length_upper_bound`: an upper bound on length of the proposal. Extrinsic is
  weighted according to this value with no refund.

This will only work after `VotingPeriod` blocks from the time that the preimage was
noted, if it's the same account doing it. If it's a different account, then it'll only
work an additional `EnactmentPeriod` later.

Emits `PreimageReaped`.

Weight: `O(D)` where D is length of proposal.

## Unlock

[`unlock`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.unlock)

Unlock tokens that have an expired lock.

The dispatch origin of this call must be *Signed*.

* `target`: The account to remove the lock on.
* `asset_id` : The asset id of a proposal.

Weight: `O(R)` with R number of vote of target.

## Remove Vote

[`remove_vote`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.remove_vote)

Remove a vote for a referendum.

If:

* the referendum was cancelled, or
* the referendum is ongoing, or
* the referendum has ended such that
  * the vote of the account was in opposition to the result; or
  * there was no conviction to the account's vote; or
  * the account made a split vote
    ...then the vote is removed cleanly and a following call to `unlock` may result in more
    funds being available.

If, however, the referendum has ended and:

* it finished corresponding to the vote of the account, and
* the account made a standard vote with conviction, and
* the lock period of the conviction is not over
  ...then the lock will be aggregated into the overall account's lock, which may involve
  *overlocking* (where the two locks are combined into a single lock that is the maximum
  of both the amount locked and the time is it locked for).

The dispatch origin of this call must be *Signed*, and the signer must have a vote
registered for referendum `index`.

* `asset_id` : The asset id of a referendum.
* `index`: The index of referendum of the vote to be removed.

Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on.
Weight is calculated for the maximum number of vote.

## Remove Other Vote

[`remove_other_vote`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.remove_other_vote)

Remove a vote for a referendum.

If the `target` is equal to the signer, then this function is exactly equivalent to
`remove_vote`. If not equal to the signer,d then the vote must have expired,
either because the referendum was cancelled, because the voter lost the referendum or
because the conviction period is over.

The dispatch origin of this call must be *Signed*.

* `target`: The account of the vote to be removed; this account must have voted for
  referendum `index`.
* `asset_id` : The asset id of a referendum.
* `index`: The index of referendum of the vote to be removed.

Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on.
Weight is calculated for the maximum number of vote.

## Enact Proposal

[`enact_proposal`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.enact_proposal)

Enact a proposal from a referendum. For now we just make the weight be the maximum.

## Blacklist

[`blacklist`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.blacklist)

Permanently place a proposal into the blacklist. This prevents it from ever being
proposed again.

If called on a queued public or external proposal, then this will result in it being
removed. If the `ref_index` supplied is an active referendum with the proposal hash,
then it will be cancelled.

The dispatch origin of this call must be `BlacklistOrigin`.

* `proposal_hash`: The proposal hash to blacklist permanently.
* `asset_id` : The asset id of a referendum.
* `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be
  cancelled.

Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a
reasonable value).

## Cancel Proposal

[`cancel_proposal`](https://dali.devnets.composablefinance.ninja/doc/pallet_democracy/pallet/enum.Call.html#variant.cancel_proposal)

Remove a proposal.

The dispatch origin of this call must be `CancelProposalOrigin`.

* `prop_index`: The index of the proposal to cancel.

Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`
