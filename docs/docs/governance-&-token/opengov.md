# Picasso Kusama OpenGov

Governance mechanisms for Picasso Kusama are intended to ensure the growth and adaptation of the ecosystem in alignment with the wants and needs of the Picasso Kusama community. Therefore, all token holders are able to participate, with their votes being weighted by stake. Moreover, any alteration to Picasso Kusama must be approved by a referendum decided by PICA token holders.

[Picasso’s Polkassembly](https://picasso.polkassembly.io/opengov) will serve as a governance forum where open discussion about the future of Picasso Kusama can occur. Here, proposals can be refined based on community input.  

Picasso Kusama OpenGov is live, a new era of decentralisation, allowing PICA holders to actively participate in governance and a new structure of collectives. PICA serves as the native token of Picasso Kusama and plays a vital role on the parachain. It is extensively used as a gas token and governance token [powered by OpenGov](../governance-%26-token/opengov.md). Additionally, PICA is a requirement for infrastructure providers who operate oracles and collators on the network.

Picasso’s OpenGov structure and design is an adapted version of Polkadot OpenGov.

## Core Tenets

Core principles guiding participation in the Picasso Kusama OpenGov process are as follows:

- Supporting engagement of token holders who are influenced by and in turn would like to influence Picasso Kusama governance, even when their opinions and desires differ from that of the Composable team
- Prioritising the greater good of Picasso Kusama and its community over any individual interests
- Upholding transparency and openness with the public
- Acting morally and with a mind for consequences of action or inaction
- Standing firmly against any malicious language, behaviour, and actions

## On-Chain Governance

Picasso’s hard governance involves on-chain mechanisms where the majority of tokens on Picasso Kusama determine key decisions. These decisions are made via token holders voting on proposed referenda. 

### OpenGov Committees

The **Technical Committee** is a group of 6 core developers that are able to whitelist proposals. Its purpose is to provide technical review of urgent security issues and upgrades. There must always be 1/3 approval from the committee to whitelist proposals. 

The **Treasury Committee** is an on-chain entity made up of 11 senior team members and supporters. Members of the Treasury committee consist of:

- Henry Love, Executive Director of Composable Foundation
- 0xBrainJar, Composable Founder & Research Director
- Blas Rodriguez Irizar, Composable Co-Founder & CTO
- Joe DeTommasso, Composable Head of Governance & Strategy
- Miguel Santefé, Composable Co-Founder & Head of Design
- Jafar Azam, Composable Devrel
- Jacob Gadikian, Notional Ventures Founder & CEO
- Jesse Abramowitz, Entropy Lead Software and Protocol Engineer
- Will Pankiewicz, Parity Master of Validators
- Tamara Frankel, D1 Ventures Founding Partner
- James Wo, Digital Finance Group (DFG) Founder & Chairman

The Treasury Committee also control Picasso’s multi-sig wallets holding the allocation for Liquidity Programs and Ecosystem incentives. Treasury proposals can be submitted by anyone but spending can only be approved by this council; the funds from any of these wallets will only be transferred upon the approval of on-chain governance. For more details, refer to the PICA token transparency commitment statement.

The **Relayer Committee** consists of accounts running the Hyperspace relayer.

## Definitions
Definitions and components for OpenGov on Picasso Kusama are detailed below:

#### Origins 
An origin is an authorization-based dispatch source for an operation. This determines the Track that a referendum is posted in.

#### Pre-Image Deposit
This is the amount of tokens that a proposer must bond to submit a pre-image. It is a base deposit per network plus a fee per byte of the proposed pre-image.

#### Pre-Image Hash
This is the hash of the proposal to be enacted. The first step to make a proposal is to submit a pre-image; the hash is its identifier. The proposer of the pre-image can be different than the user that proposes that pre-image as a formal proposal

#### Proposals
Proposals are an action or item (defined by the pre-image hash) proposed by a token holder and open for consideration/discussion by token holders.

#### Referendum
A referendum is a stake-based voting model. Each referendum is associated with an individual proposal for modifying Picasso Kusama in some way. This could include changes to code, parameters, or the governance of Picasso Kusama.

## Tracks
This is a specific pipeline delineating the life cycle of a proposal. Tracks in Picasso Kusama OpenGov are Root, Whitelist Caller, General Admin, Referendum Canceller, and Referendum Killer:

| Track           | Description    | Example |
|-----------------------|-------------------|--------------------------|
| Root        | Highest Privileges        | Runtime Upgrades                   |
| Whitelist Caller           | Fast-track proposals          | Accelerated proposal  |
| General Admin          | On-chain changes            | Apollo and Collator onboarding, Release committee, LSD         |
| Referendum Canceller                 |  Cancelling proposal                           |  Incorrect referendum      |
| Referendum Killer            |  Cancelling proposals & slashing deposits            |    Malicious referendum              |


### Voting
Token holders can approve or reject proposals. 

A vote’s weight is defined by the following:

1. The number of tokens a user commits to a vote
2. The lock period of the vote; in Picasso Kusama OpenGov, users can voluntarily lock tokens to increase their voting power, with longer lock periods associated with a conviction multiplier on vote weight: 

| Lock Period After Enactment     | Conviction Multiplier    | Lock Time |
|-----------------------|-------------------|--------------------------|
| 0        | 0.1x        | None                   |
| 1             | 1x             | 28 days                   |
| 2             | 2x             | 56 days                   |
| 4             | 3x             | 112 days                   |
| 8             | 4x             | 224 days                   |
| 16             | 5x             | 448 days                   |
| 32             | 6x             | 896 days                   |


#### Vote Delegation
Voters can delegate voting power (including conviction multiplier) to other token holders (“delegates”). This feature exists to allow tokens to be delegated to those who may be more knowledgeable about Picasso Kusama and thus able to make more informed decisions on specific referenda.

#### Multirole Delegation
Voting power can be delegated based on tracks, e.g. token holders can specify different delegates for each track.

#### Approval
This is the minimum number of votes for passing a referendum, as a percentage of total conviction-weighted votes needed to approve the referendum.

#### Support
This is the minimum number of votes for passing a referendum (NOT taking into consideration conviction-weighted votes) needed to approve the referendum.

#### Lead-In Period
This is the initial period of discussion and voting on a proposal. During this period, proposals are undecided until they pass the criteria for a Track, which include:

- The prepare period, or the minimum time a referendum needs to wait before it can progress to the next phase after submission
- Capacity, or the limit for a number of referenda on a given track that can be decided at once
- Decision deposit, or the minimum deposit amount needed for a referendum to progress to the decision phase after the lead-in period ends; this deposit is larger than the submission deposit in order to limit spam proposals/referenda

Details on the lead-in period (specifically, the prepare period) for each track are found in the OpenGov Parameters section of this documentation.

#### Decision Period
During this period, token holders continue to vote on the referendum. If a referendum does not pass by the end of the period, it will be rejected, and the Decision Deposit will be refunded.
Details on the decision period for each track are found in the OpenGov Parameters section of this documentation.

#### Confirm Period
This is a period of time within the decision period where the referendum needs to have maintained enough Approval and Support to be approved and move to the enactment period. 

Details on the confirm period for each track are found in the OpenGov Parameters section of this documentation.

#### Enactment Period
This is a specified time, defined at the time the proposal was created, that an approved referendum waits before it can be dispatched. 
There is a minimum amount of time for each Track. Details on the enactment period for each track are found in the OpenGov Parameters section of this documentation.
## OpenGov Parameters
Governance parameters (for each referenda track) are as follows:

| Track | Track ID | Concurrent Proposals | Decision Deposit |
| -------- | -------- | --- | -------- |
| Root         |  0   | 5    | 5,000,000 PICA         |
| Whitelist Caller   |   1   | 25    | 500,000 PICA      |
| General Admin   |   2       | 10    |  1,000,000 PICA    |
| Referendum Canceller    |  3   | 10    | 1,000,000 PICA   |
| Referendum Killer  |   4       | 25    | 1,000,000 PICA   |

## Period Parameters by Track

| Track    |Prepare Period     | Decision Period | Confirm Period | Min. enactments |
| --- | --- | -------- | -------- | -------- |
| Root    | 1 day 2 Hours (600 Blocks)    |  10 Days (72000 Blocks)        | 1 Day (7200 Blocks)      | 1 day         |
| Whitelist Caller    |  10 mins (50 Blocks)   | 10 Days (72000 Blocks)         |  30 mins (150 Blocks)        | 10 mins         |
| General Admin    | 1 hour (300 blocks)    | 10 Days (72000 Blocks)         |  1 Day (7200 Blocks)        |   1 day       |
| Referendum Canceller    | 1 hour 1 Day (7200 Blocks)    | 10 Days (72000 Blocks)         | 3 Hours (3600 Blocks)         | 10 mins   |
| Referendum Killer    | 1 hour 1 Day (7200 Blocks)    | 10 Days (72000 Blocks)         | 3 Hours (3600 Blocks)         |  10 mins        |

## Support and Approval Parameters by Track

| Track    | Approval Curve     | Parameters | Support Curve | Parameters |
| --- | --- | -------- | -------- | -------- |
| Root    | Reciprocal   |  Day 0: 100% Day 2: 80% Day 10: 50%     | Linear      | Day 0: 50% Day 10: 0.5% |
| Whitelist Caller    | Reciprocal | Day 0: 100% Day 2: 80% Day 10: 50%   | Reciprocal      | Day 0: 2% Hour 1: 1% Day 14: 0%  |
| General Admin    | Reciprocal  | Day 0: 100% Day 2: 80% Day 10: 50%  | Reciprocal  | Day 0: 50% Day 5: 10% Day 10: 0% |
| Referendum Canceller    | Reciprocal | Day 0: 100% Day 2: 80% Day 10: 50% | Reciprocal  | Day 0: 10% Day 1: 1% Day 10: 0% |
| Referendum Killer    | Reciprocal  | Day 0: 100% Day 2: 80% Day 10: 50% | Reciprocal   |  Day 0: 10% Day 1: 1% Day 10: 0% |

## Approval Curves
With X % of support, Referenda can pass after Y duration (time periods in the table) since the beginning of referenda depending on whethere the approval rate is above the approval curve.

![whitelist-curve](../governance-&-token/whitelist-track.png)
*Approval curve for the Whitelist Track*

![root-curve](../governance-&-token/root-track.png
*Approval curve for the Root Track*
                                                                                                             
## Proposal Roadmap

1. A proposal author should submit their idea to Picasso’s Polkassembly governance forum, where they should be open to community feedback for at least five days before moving forward
2. Taking into account feedback, the proposal author can submit their proposal on-chain
   - The proposer must first submit the preimage (if you need assistance with creating the preimage or would like secondary approval, reach out to our team on Discord)
   - Note: your preimage deposit will be returned once via unnoting after the proposal is submitted
   - The proposer then can submit the Referendum, and place the decision deposit (which covers the on-chain storage cost of the proposal)
3. Thus veins the lead-in period, where the community can begin voting
4. The proposal will then move to the decision period when the following are met:
   - The referenda waits the duration of the prepare period (ensuring enough time for discussion)
   - There is capacity in the chosen track
   - A decision deposit has been submitted and meets the minimum requirements of the track
5. During the decision period, voting continues and the referendum has a set amount of days to reach approval.
   - If the Referendum is rejected, the decision deposit will be returned
6. If the Referendum is approved, it enters the confirm period where it must remain approved for the duration of this period.
   - If the referendum fails to meet these requirements at any time, it moves back to the decide period; if it again meets these requirements, it moves back to the confirm period and the decide period is delayed until the end of the confirm period
7. If the referendum receives enough approval and support throughout the confirm period, it will be approved and move to the enactment period
8. Once the enactment period elapses, the referendum will be executed

## Proposal Cancellations
If a proposal in the voting stage is found to have an issue, it may be necessary to prevent its approval. This could be due to malicious activity or technical issues that void the proposal.

Cancellations must be voted on by the network. Cancellation proposals are expedited, as they must be decided before the enactment of the proposal they seek to cancel. However, the same process as standard referenda applies.

Moreover, a cancellation Origin called the Emergency Canceller exists for use against any referendum with an unanticipated issue. The Emergency Canceller Origin and the Root Origin can cancel referenda. Regardless of the Origin, if a proposal is cancelled, it is rejected and the decision deposit is refunded.

The Kill Origin called Emergency Killer exists for use against malicious referenda. The Emergency Killer Origin and the Root Origin have the ability to kill referenda. The difference between killing and cancelling a referenda is that in the case of a kill, not only is the proposal cancelled, but also the Decision Deposit is slashed, meaning the deposit amount is burned regardless of the Origin.
