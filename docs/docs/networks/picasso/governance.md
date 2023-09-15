# Picasso OpenGov

As we plan to completely decentralize the governance on the Picasso network, the decisions affecting the parachain will go through a democratic process as Picasso transitions from Governance V1 to OpenGov, a two-phase process that will empower the community holders. Previously, Picasso operated via Governance V1 whilst Sudo operations took place with the approval of the Picasso council. Referendums can be proposed by anyone to ensure agreement from the wider community and to establish transparency when significant changes are made to Picasso.

The new OpenGov structure for Picasso will be implemented in two phases.

Phase 1: Implement OpenGov with governance handled by two collectives: The Picasso Council and the Technical Committee.

This phase lays the foundation for a more transparent and community-driven governance model while maintaining the speed and efficiency demonstrated by the recent launches and connections on Picasso, especially in preparation for our upcoming IBC connection to Ethereum.

Phase 2: Release OpenGov to public PICA holders.

The second phase will be implemented after the launch of the Ethereum ⬌ IBC connection which will be released in Q4 2023. Through OpenGov, the Picasso parachain will undergo a new era of decentralization, allowing PICA holders to actively participate in governance and a new structure of collectives.

## OpenGov Tracks

During Phase 1, Council members and the Technical Committee will be able to vote via a GOV token. The GOV token has no monetary value and held by the Picasso Council and Technical Committee members. Initially, there are two tracks, and four origins:

- Root
- Whitelisted Caller (Fast Track)

Additional tracks are also planned to be added during Phase 1 such as General Admin, Treasury and more.

The governance model during Phase 1 will introduce four separate roles, each serving a unique purpose:

- Proposal Creation: Enabling the ability to create proposals, fostering innovation and community-driven development.
- Voting: Allowing GOV holders to vote on proposals, ensuring that decisions reflect the collective will of the community.
- Canceling Proposals/Slashing: Implementing mechanisms to cancel proposals and penalize those who voted maliciously.
- Expedition: Introducing measures to speed up the voting process when needed, ensuring timely decision-making without compromising on democratic principles.

These tracks and origins are designed to ensure a balanced and fair approach, aligning with our commitment to transparency, decentralization, and community engagement.

## OpenGov Parameters

The following two tables provide information about the voting and decision-making processes for the two collectives. They summarize the support thresholds required for various actions, the time it takes for those actions to pass when specific support percentages are met, and the confirmation and decision periods for each track. With X % of support, referenda can pass after Y duration (times in the table) since the beginning of referenda if the approval rate is above the approval curve. The decision deposit for Root is 500,000 PICA whereas for Whitelist, its is 50,000 PICA. 


| Support Threshold | Whitelist Time (Passing) | Root Time (Passing) |
|-------------------|--------------------------|----------------------|
| 10%               | 12 hours        | 5 days 14 hours      |
| 20%               | 3 hours 20 minutes       | 4 days 4 hours       |
| 30%               | 1 hour 15 minutes        | 2 days 18 hours      |
| 10%               | 12 hours 0 minutes       | 5 days 14 hours      |
| 20%               | 3 hours 20 minutes       | 4 days 4 hours       |
| 30%               | 1 hour 15 minutes        | 2 days 18 hours      |


| Track Type            | Confirm Period    | Decision Period (Voting) |
|-----------------------|-------------------|--------------------------|
| Whitelist        | 30 minutes        | 4 days                   |
| Root             | 1 day             | 7 days                   |

The following table summarizes the rules and requirements related to certain actions within the OpenGov system. It provides a concise reference for understanding the conditions and costs associated with various actions in the OpenGov process.


| Action        | Origin & Threshold         | Submission Deposit | 
|---------------|----------------------------|--------------------|
| Submit        | Anyone from any collective | 1 PICA             | 
| Cancel        | 1/3 Tech Committee        | 1 PICA             | 
| Kill          | 1/2 Council                | 1 PICA             | 

## OpenGov Collectives

### Picasso Council

The Council is an on-chain entity made up of 11 senior team members and supporters.

Each council member is represented as an on-chain account on Polkadot.js. Members of the Council consist of:
- 0xBrainJar, Composable Founder/CEO
- Blas Rodriguez Irizar, Composable Co-Founder & CTO
- Joe DeTommasso, Composable Head of Governance & Strategy
- Miguel Santefé, Composable Co-Founder & Head of Design
- Jafar Azam, Composable Devrel
- Henry Love, Fundamental Labs Managing Partner
- Jacob Gadikian, Notional Ventures Founder & CEO
- Jesse Abramowitz, Entropy Lead Software and Protocol Engineer
- Will Pankiewicz, Parity Master of Validators
- Tamara Frankel, D1 Ventures Founding Partner
- James Wo, Digital Finance Group (DFG) Founder & Chairman

The Council members also control Picasso’s multi-sig wallets holding the allocation of the Treasury, Liquidity Programs and Ecosystem incentives. Please note that the funds from these wallets will only be transferred upon the approval of on-chain governance. For more details, look at the [PICA token transparency commitment statement](../picasso/token-transparency.md).

:::note
There are certain features of Governance V1 still active on Picasso such as Democracy Referenda, Emergency Measures and Emergency Proposals, this will be removed in the coming weeks as OpenGov is given time to be deemed as effective as stable and more tracks are added. 
:::

#### Emergency Measures

In case of extreme urgency, the Council may implement an “emergency measure”, 
such as when a major security vulnerability is found or when the imminence of great prejudice to the token holders or the network is clear, 
which does not need to be submitted as a proposal or for approval in a referendum. 
In such a case, the emergency measure will be immediately executable and implemented as fast as the Council deems fit.

### Technical Committee

Picasso’s Technical Committee consists of 5 core developers who are chosen by the Council. The role of the Technical Committee includes, among others, ensuring the technical stability and critical safety measures of the parachain. 

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
| Referendum voting period                           | 3 days         |
| Enactment delay of an approved referendum          | 0.5 days       |
| Cool-off period after a proposal has been rejected | 3 days         |
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