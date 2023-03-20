# Picasso OpenGov tracks



| Track/Origin | Prospoals Threshold | Lead Period | Decision Depos | Decision Period | Confirmation Period | Parallel Decisions | Initial approval | Initial support | Slope | Enactment Period |
| ------------ | ------------------- | ----------- | -------------- | --------------- | ------------------- | ------------------ | ---------------- | --------------- | ----- | ---------------- |
| Sudo         | 100 PICA            | 1 D         | 2 days         | 500K Pica       | 3 days              |                    | 1                |                 | 1     |                  |
| Cancellation | 100 PICA            | 1 D         | 2 days         | 500K Pica       | 3 days              |                    | 1                |                 | 1     |                  |
| Origin
| Root
| Treasury
| DeFi
| Bridge
| Runtime


Sudo - default until decided to lower.

Submission Deposit - The minimum amount to be used as a (refundable) deposit to submit a public referendum proposal.

Prepare Period - The minimum time the referendum needs to wait before it can progress to the next phase after submission. Voting is enabled, but the votes do not count toward the outcome of the referendum yet.

Decision Deposit - This deposit is required for a referendum to progress to the decision phase after the end of prepare period.

Decision Period - Amount of time a decision may take to be approved to move to the confirming period. If the proposal is not approved by the end of the decision period, it gets rejected.

Max Deciding - The maximum number of referenda that can be in the decision period of a track all at once.

Approval: the share of the approval vote-weight after adjustments for conviction against the total number of vote-weight for both approval and rejection

Support: The total number of votes in approval (ignoring adjustments for conviction) compared to the total possible amount of votes that could be made in the system. Support also takes into account abstained votes.

Min Approval - The threshold of approval (along with the min support) needed for a proposal to meet the requirements of the confirm period.

Min Support - The threshold of support (along with the min approval) needed for a proposal to meet the requirements of the confirm period.

Confirm Period - The total time the referenda meets both the min approval and support criteria during the decision period.

Min Enactment Period - Minimum time that an approved proposal must be in the dispatch queue after approval. The proposer has the option to set the enactment period to be of any value greater than the min enactment period.



Tracks 
thresholds and longer consideration periods.
Root track  - 2 days before voting begins, 5 days in voting, 1 day for enacting. Runtime upgrade track is Root track.
Treasury Track - treaasurin spending (Either Sudo or Treasury) capped by 500KPICA  Same named origin or Sudo
DeFi  track - allowed to setup safe parameters (e.g. asset symbols). Same named origin or Sudo.
Bridge track - allows to do permissoned bridge operations. Same named track
Runtime track - technical track


If some operation is undecided or risky, it will be sudo from start to speed up applicaiton
