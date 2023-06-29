# A Look into the Composable Parachain's Governance

Similar to the [governance of Picasso](../parachains/picasso/governance), Composable’s Polkadot parachain will ultimately become completely decentralized and democratic. This requires a sufficient user base, so in the initial stages of parachain deployment, the Composable Council will aid in governance. 

:::info
Governance processes on Composable are carried out similarly to the V1 governance model of the Kusama and Polkadot networks. Governance also involves referenda, ensuring that the majority of our community supports any implemented changes and decisions. This further ensures transparency in alterations to the Composable parachain.
:::

## Governance Divisions

### Composable Council

Composable’s Polkadot parachain governance will initially be carried out via a Sudo account held by the Composable Foundation. Once core pallets are onboarded onto the parachain, the initial Sudo council will be burned, with the Composable Council taking over governance. The on-chain Composable Council comprises 11 members including senior Composable team members and Composable supporters. Council members are also responsible for controlling the Composable parachain’s multi-sig wallets.

Composable Council members are as follows:
- 0xBrainJar, Composable Founder/CRO
- Henry Love, Fundamental Labs Managing Partner
- Blas Rodriguez Irizar, Composable CTO
- 0xc0dejug, Protocol Engineer
- 0xslenderman, Composable Head of Design
- Joon W., Composable FE Lead
- Jafar Azam, Composable Devrel
- Jesse Abramowitz, Entropy Lead Software and Protocol Engineer
- Will Pankiewicz, Parity Master of Validators
- Tamara Frankel, D1 Ventures Founding Partner
- James Wo, Digital Finance Group (DFG) Founder & Chairman

For the Composable Council to pass a decision, a majority vote (6 out of 11 council members) is required. As the Composable parachain community grows, community members will have an increasing role in governance. Ultimately, the Composable parachain will become a full democracy. Until then, The Composable Council will act to protect and support the functionality of the Composable parachain. Once the Council determines that the Composable parachain is stable and self-sufficiently operating, governance will be transferred to the community, who will then govern:

- Elections of new council members
- Referenda proposals
- Vetos for risky or malicious referenda
- Sub-council/committee creation to manage specific subject matters (with veto power for this retained by the Composable Council)
- Election of Technical Committee Members

Councils (including the Composable Council) must elect a “Prime Member” from their ranks. This individual’s vote will act as the default vote for any Council members that may not vote before the time limit elapses. This ensures a quorum even in the event of multiple abstentions.

Additional Councils established after the initial Composable Council are able to initiate proposal creation and develop their own proposals. Some examples for proposal topics are as follows:

- Implementation of a Crowdloan Rewards pallet enabling those who contributed to the [Composable Crowdloan](../composable/composable-crowdloan.md) to claim their rewards
- Creation of new pallets, products, and protocols on the Composable parachain

All council members have veto power, which can be used once per proposal. If a council member vetoes a proposal, they cannot veto the same proposal if it is resubmitted after the 15-day cooldown period.

:::caution Emergency Measures
In case of extreme urgency, the Council may implement an “emergency measure”, such as when a major security vulnerability is found or when the imminence of great prejudice to the token holders or the network is clear, which does not need to be submitted as a proposal or for approval in a referendum. 

In such a case, the emergency measure will be immediately executable and implemented as fast as the Council deems fit.
:::

### Technical Committee

The Technical Committee for the Composable parachain consists of core developers who are chosen by the Council. The members of the Technical Committee will initially be composed of Composable Finance developers. The role of the Technical Committee includes, among others, ensuring the technical stability and critical safety measures of the parachain. 

#### Emergency Proposals

The Technical Committee may, along with the Council, submit an “emergency proposal”, which should not be confused with an “emergency measure”. An emergency proposal submitted by the technical committee will be submitted for immediate referendum, along with fast-tracked voting and implementation. This will be the method used for emergency bug fixes or rapid implementation of new features into the Composable network, and in general, the strategic furtherance of the Composable Network’s technology.

## Democracy guidelines

Proposals can be initiated by both the Community and the Council, and are required for any action that would directly affect the Composable parachain such as approving or rejecting a treasury payout, code upgrades, graduating pallets, and more. The following table portrays the initial criterion for interacting with governance proposals on the Composable parachain:

| Parameter                                          | Period/Number  |
|----------------------------------------------------|----------------|
| Referendum voting period                           | 3 days          |
| Enactment delay of an approved referendum          | 1 day         |
| Cool-off period after a proposal has been rejected | 3 days        |
| Maximum pending community proposals                | 100 |

### Adaptive Quorum Voting

There are different voting thresholds for approving any proposal in a referendum, similar to how the Polkadot network applies [adaptive quorum voting](https://wiki.polkadot.network/docs/learn-governance#adaptive-quorum-biasing).

### Community Proposals

Any token holder has the ability to initiate a proposal for a vote from the entire community by locking up a minimum deposit of 100 LAYR for a certain period and posting a vote on our governance forum. Other token holders can endorse proposals by locking up a matching deposit. The proposal with the highest amount of bonded LAYR will be passed as a referendum in the next voting cycle. There can be a maximum of 100 community proposals in the queue at a given time.

Unlike proposals made by a Council, the Community can submit proposals for referendum without need of undergoing a qualifying vote; however, the amount of bonded support would determine the proposal’s priority in terms of the time it is submitted for a referendum. The higher the bonded support, the sooner the proposal gets considered for a referendum.

In the case of community proposals, a super majority must approve the referendum in order for it to pass.

### Council Proposals

While the Council may submit their own proposals for referendum, these proposals must have While the Council may submit their own proposals for referendum, these proposals must have previously received a majority vote within the Council itself. Afterwards, the referendum will be submitted for voting by the Community. There are then two potential outcomes:

1. In the case of a unanimous decision by the Council, 
   a super majority vote from the Community to overrule the Council’s referendum.
2. In the case of a simple majority decision by the Council, 
   a simple majority vote from the Community will decide the outcome of the Council’s referendum.

### Canceling a referendum 

Once a proposal has been submitted for referendum, the referendum cannot be canceled until it has entered a voting round, unless proposals are found and deemed malicious (for example, faults in code additions) by a 60% supermajority of the Composable Council or a unanimous decision of the Technical Committee, in which case the referendum shall be revoked. All tokens locked by supporters of the proposal would be burned.

### Vote Power Calculation

The voting power of a token holder is determined by two factors: 
the amount of tokens and the length of time it is bonded or locked. 
This refers to when users lock their tokens for an extended period in order to gain increased voting power such that:

    Voting power = LAYR tokens * vote multiplier

| Lock Period | Vote Multiplier |
|:-----------:|:---------------:|
|      0      |       0.1       |
|      1      |        1        |
|      2      |        2        |
|      4      |        3        |
|      8      |        4        |
|     16      |        5        |
|     32      |        6        |

The lock periods are fixed which means that a LAYR token holder cannot lock LAYR tokens for a lock period of, say, f5 or 20. The default lock period shall be seven (7) days which is counted starting from the end of the referendum or the end of the default period for the enactment day. 

If an approved referendum has an enactment delay of other than seven (7) days, its lock period would be equal to the specific period of that enactment delay. While a token is locked, it can still be used for voting and staking but it cannot be transferred to another account.

Votes are tallied at the end of the voting period. All of the voters who win the proposal will have their LAYR locked into the chain for the duration of the enactment of the proposal and losing voters have no lock up period.
