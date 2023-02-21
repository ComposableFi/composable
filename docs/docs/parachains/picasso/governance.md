# A look into Picasso Governance

As we plan to completely decentralize the governance on the Picasso network, 
we have designed the decisions affecting the parachain to go through a fair 
democratic process supported by the Substrate governance affiliated pallets. 
The Democracy pallets we are utilizing are provided within the Substrate libraries 
and a core piece of the logic that constitutes the runtime of the Kusama and Polkadot networks. 
Referendums will be carried out to ensure agreement from the wider community 
and to establish transparency when significant changes are made to Picasso.

## Governance Divisions

### Picasso Council

Upon launch, governance on Picasso parachain will be done by a Sudo account held by the Composable. 
Following the onboarding of core pallets on Picasso, 
the Sudo council will be burned and the First Council will then be in charge of governance. 
The First Council is an on-chain entity made up of 11 senior team members from and our supporters. 
The Council members also control Picasso’s multi-sig wallet.

When the Picasso parachain is live, each council member will be represented as an on-chain account on Polkadot.js. 
Members of the First Council will consist of:
- 0xBrainJar, Composable Founder/CEO
- Jeff Smith, Composable General Counsel (GC)
- Eoin Whelan, Composable CFO
- Blas Rodriguez Irizar, Composable Principal Bridging Lead
- 0xc0dejug, Composable Principal XCVM Lead
- 0xslenderman, Composable Head of Design
- Jesse Abramowitz, Entropy Lead Software and Protocol Engineer
- Will Pankiewicz, Parity Master of Validators
- Tamara Frankel, D1 Ventures Founding Partner
- James Wo, Digital Finance Group (DFG) Founder & Chairman

As the Picasso community strengthens, community members will play a bigger role in governance and Picasso will transition into a full democracy. 
In the meantime, the First Council will safeguard the functioning of the parachain. 
First Council motions require a strict majority or at least 6 out of the 11 members to be passed.

When the First Council has reasonably determined that Picasso is operating in a stable and self-sufficient manner, 
governance will be handed over to the community. 
The community will then be responsible for governance of the network including:

- The periodic election of new Council members
- Proposing referenda
- Vetoing risky or malicious referenda
- Creating sub-councils or other committees to focus on specific subject matters. 
  The Council will maintain veto power over any such sub-council or committee.
- Electing the new Technical Committee members, which will be discussed in more detail shortly

The subsequent Councils can initiate the proposal creation process and create their own proposals, 
on matters including, but not limited to: 

- (a) the implementation of a Crowdloan Rewards pallet enabling those who contributed to the Picasso Crowdloan to claim their rewards; 
- (b) creation of liquidity pools in Pablo, a new-generation decentralized exchange (DEX) that will be the first protocol to launch on Picasso; 
- (c) setting and adjusting the rate of staking rewards and other emissions for Picasso and Pablo; and 
- (d) management and allocation of the Pablo and Picasso treasury.

A council member has a veto power but which can only be exercised once for any single proposal. 
If the proposal has been vetoed by a Council member, 
any Council member cannot veto the same proposal upon resubmission after the 15-day cooldown period.

Both the First Council and subsequent Councils will elect a “Prime Member” amongst themselves whose vote will act as 
the default for Council members who fail to vote before the timeout. 
The main purpose of the foregoing measure is to ensure a quorum even if multiple Council members choose to abstain from a vote.

#### Emergency Measures

In case of extreme urgency, the Council may implement an “emergency measure”, 
such as when a major security vulnerability is found or when the imminence of great prejudice to the token holders or the network is clear, 
which does not need to be submitted as a proposal or for approval in a referendum. 
In such a case, the emergency measure will be immediately executable and implemented as fast as the Council deems fit.

### Technical Committee

Picasso’s Technical Committee consists of core developers who are chosen by the Council. 
The members of the Technical Committee will initially be composed of Composable Finance developers. 
The role of the Technical Committee includes, among others, 
ensuring the technical stability and critical safety measures of the parachain. 

#### Emergency Proposals

The Technical Committee may, along with the Council, submit an “emergency proposal”, 
which should not be confused with an “emergency measure”. 
An emergency proposal submitted by the technical committee will be submitted for immediate referendum, 
along with fast-tracked voting and implementation. 
This will be the method used for emergency bug fixes or rapid implementation of new features into the PICA network, 
and in general, the strategic furtherance of Picasso Network’s technology.

## Democracy guidelines

Proposals can be initiated by both the Community and the Council, 
and are required for any action that would directly affect the parachain such as
approving or rejecting a treasury payout, code upgrades, graduating pallets, and more. 
The following table portrays the initial criterion for interacting with governance proposals on Picasso:

| Parameter                                          | Period/Number  |
|----------------------------------------------------|----------------|
| Referendum voting period                           | 7days          |
| Enactment delay of an approved referendum          | 7 days         |
| Cool-off period after a proposal has been rejected | 15 days        |
| Maximum pending community proposals                | 100 |

#### Adaptive Quorum Voting

There are different voting thresholds for the approval of any proposal in a referendum. 
The threshold would depend on: (1) whether it is the Community or Council that initiated the proposal and (2) whether, 
in the case of the Council, there was a unanimous or simple majority approval of the proposal:

|        Entity         |                                                                                                                                                                                   Metric                                                                                                                                                                                  |
|:---------------------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
|       Community       | Positive Turnout Bias (Super-Majority Approve) <br/><br/> A voting requirement with a *positive turnout bias*, whereby a heavy super-majority of *aye* votes is required to carry at low turnouts. However, as the voting turnout increases towards one hundred (100%), the required number of *aye* votes to approve the proposal gradually lowers to a simple majority. |
|  Council (Unanimous)  |  Negative Turnout Bias (Super-Majority Against) <br/><br/> A voting requirement with a *negative turnout bias*, whereby a heavy super-majority of *nay* votes is required to reject at low turnouts. However, as the voting turnout inches towards one hundred (100%), the required number of *nay* votes to reject the proposal gradually lowers to a simple majority.   |
|  Council (Majority)   |                                       Simple Majority <br/><br/> If there are more aye votes than *nay*, then the proposal is carried. Conversely, if there are more nay votes than *aye*, then the proposal is rejected. Determining the existence of a simple majority entails a simple comparison of votes in a simple majority.                                       |                                                                                   

### Community Proposals

Any token holder has the ability to initiate a proposal for a vote from the entire community 
by locking up a minimum deposit of 100 PICA for a certain period and posting a vote on our governance forum. 
Other token holders can endorse proposals by locking up a matching deposit. 
The proposal with the highest amount of bonded PICA will be passed as a referendum in the next voting cycle. 
There can be a maximum of 100 community proposals in the queue at a given time.

Unlike proposals made by a Council, 
the Community can submit proposals for referendum without need of undergoing a qualifying vote;
however, the amount of bonded support would determine the proposal’s 
priority in terms of the time it is submitted for a referendum. 
The higher the bonded support, the sooner the proposal gets considered for a referendum.

In the case of community proposals, a super majority must approve the referendum in order for it to pass.

### Council Proposals

While the Council may submit their own proposals for referendum, 
these proposals must have While the Council may submit their own proposals for referendum, 
these proposals must have previously received a majority vote within the Council itself. 
Afterwards, the referendum will be submitted for voting by the Community. There are then two potential outcomes:

1. In the case of a unanimous decision by the Council, 
   a super majority vote from the Community to overrule the Council’s referendum.
2. In the case of a simple majority decision by the Council, 
   a simple majority vote from the Community will decide the outcome of the Council’s referendum.

### Canceling a referendum 

Once a proposal has been submitted for referendum, 
the referendum cannot be canceled until it has entered a voting round, 
unless proposals are found and deemed malicious (for example, faults in code additions) 
by a 60% supermajority of the Picasso council or a unanimous decision of the Technical committee, 
in which case the referendum shall be revoked. All tokens locked by supporters of the proposal would be burned.

### Vote Power Calculation

The voting power of a token holder is determined by two factors: 
the amount of tokens and the length of time it is bonded or locked. 
This refers to when users lock their tokens for an extended period in order to gain increased voting power such that:

    Voting power = PICA tokens * vote multiplier

| Lock Period | Vote Multiplier |
|:-----------:|:---------------:|
|      0      |       0.1       |
|      1      |        1        |
|      2      |        2        |
|      4      |        3        |
|      8      |        4        |
|     16      |        5        |
|     32      |        6        |

The lock periods are fixed which means that a PICA token holder cannot lock PICA tokens for a lock period of, say, f5 or 20.
The default lock period shall be seven (7) days which is counted starting from the end of the referendum or the end of the default period for the enactment day. 
If an approved referendum has an enactment delay of other than seven (7) days, its lock period would be equal to the specific period of that enactment delay. 
While a token is locked, it can still be used for voting and staking but it cannot be transferred to another account. 

Votes are tallied at the end of the voting period.
All of the voters who win the proposal will have their PICA locked into the chain for the duration of the enactment of the proposal and losing voters have no lock up period. 

