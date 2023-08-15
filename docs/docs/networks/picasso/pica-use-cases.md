# PICA use cases

PICA is the native token of [Picasso](../picasso-parachain-overview.md) and the [Centauri chain](../centauri-chain.md). We have made a concerted effort to ensure that the PICA token holds as much utility as possible by incorporating various value accrual methods, and governance features. While the PICA token provides the community with a strong voice and rewards for participating within the ecosystem, it is also fundamental for the operation of collators, validators, oracles, and our other cross-ecosystem strategies. Thus, the PICA token is fundamental for governance, network usage, and the security of Picasso.

This means that PICA serves the following functions within the ecosystem:

## Tri-staking token model

Within the Picasso ecosystem there are multiple staking avenues that can be utilized to earn yield in PICA. This tri-staking model strengthens the security of the network while maintaining liquidity and distributing maximum value back to Picasso token holders.

### Oracle staking​
Apollo is a permissionless, MEV-resistant oracle solution. Anyone can run an oracle node on Picasso by providing a PICA stake.

### Collator staking​ 
25% of fees on Picasso are distributed to collators, with the remaining 75% going directly to the community-governed treasury. Collators on Picasso are required to put down a stake to produce blocks on our parachain, as with most proof of stake networks. 

### Centauri staking
Initially, PICA will also be used by Cosmos validators for securing our Cosmos chain, entitled Centauri. This is the first instance of a token being utilized for validation within both the Kusama and Cosmos ecosystems and highlights the critical role PICA plays within cross-ecosystem communication. Our Delegation Program has set forth a proposal to delegate 1bn PICA tokens from the treasury to validators to ensure a robust and secure network while ensuring a 10% APR in PICA.

## The first cross-ecosystem token
As Picasso plays a pivotal role in both our Kusama and Cosmos strategies, PICA will be utilized across both ecosystems as we continue to explore new use cases and integrations.

### Gas (network usage)​
The PICA token is uniquely positioned as the gas token at the center of Picasso, powering Composable’s efforts to enhance blockchain interoperability. Initially, this means that PICA will not only be used for transactions on Picasso itself but will also be required for bridging assets via Centauri, and cross-chain function calls. Additionally, PICA will also act as the gas token for the CosmWasm dApps deployed on the ccw-vm on Picasso. Notably, in order to further support users from other ecosystems Picasso offers a feature called “bring your own gas” (BYOG), which allows users to pay their gas fee in any supported tokens.

For example any cross-chain XCMP transactions can choose which assets to use to pay for the transaction. These tokens are then swapped for PICA under the hood, allowing users and liquidity to flow seamlessly through Picasso regardless of what ecosystem they are arriving from.

All fees may change dynamically depending on network load and pool or protocol fee formulas. The most fundamental factor for gas fees is the computational resources it consumes which is represented as the transaction's "weight". The weight of a transaction is converted into an appropriate amount of PICA by the polynomial formula which changes dynamically depending on the target load of the network. This means as the usage of the chain increases towards maximum capacity, the price of a unit of weight increases as well.

### Primary pairing on Pablo​

Pablo is the native DEX of the Picasso ecosystem and is integrated directly into the runtime of our parachain as a pallet. As such, a primary trading pair on Pablo will be PICA. You can also expect various liquidity incentives with 15% of PICA’s supply being allocated towards liquidity programs on Picasso. 

### Powering our advancement into the Cosmos
New use cases will continue to be established as we move forward with XCM channel openings and our expansion into the Cosmos.

## Governance & treasury management

The treasury will be community owned and controlled by PICA token holders.

Picasso is waging war on centralization with our vision of a seamlessly interoperable, trustless future for DeFi. As such the PICA token will play an important role in helping to realize this vision, as governance will be handed over to the community after the core infrastructure is in place. PICA holders will then be responsible for governing the network by submitting proposals for:

- Election of council members
- Which pallets are incorporated into Picasso’s runtime
- Which pallets will graduate to the Composable parachain on Polkadot
- Which pallets will live only on Picasso, and which will live on both chains
- Directing treasury initiatives
- What chains and channels are supported on Centauri
- And any other action/feature the community decides to implement into the network.