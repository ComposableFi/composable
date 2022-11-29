# PICA use cases

PICA is the native token of the Picasso parachain. 
We have made a concerted effort to ensure that the PICA token withholds as much utility as possible by incorporating 
various value accrual methods, governance features, and time-weighted benefits in the form of fNFTs. 
While the PICA token provides the community a strong voice and rewards for participating within the ecosystem, 
it is also fundamental for the operation of collators and oracles. Thus, the PICA token is fundamental for 
governance, network usage, and the security of Picasso.

This means that Picasso serves the following functions within the ecosystem:

## Gas (network usage)

The PICA token is uniquely positioned as a gas token which sits at the center of Picasso as a cross-chain DeFi hub. 
This means that PICA will not only be used for transactions on Picasso itself but will also be required for bridging
assets via Centauri, as well as cross-chain function calls via XCVM. 
Additionally, PICA will also act as the gas token for the CosmWasm VM on Picasso. 
Notably, in order to further support users from other ecosystems Picasso will offer a feature called “bring your own gas” (BYOG),
which will allow users to pay their gas fee in any supported tokens.

For example, after channels to Kusama and Statemine are opened, 
any cross-chain XCMP transactions can choose which assets to use to pay for the transaction. 
These tokens are then swapped for PICA under the hood, 
allowing users and liquidity to flow seamlessly through Picasso regardless of what ecosystem they are arriving from.

The following is an example of fees for transferring some KSM from Karura using XCMP:

1. Fee on Karura for XCMP.
2. Fee for XCMP on Picasso. Consists of a base fee plus additional fees calculated dynamically for each instruction
3. Transaction fee on Picasso
4. DEX swap fee

All fees may change dynamically depending on network load and pool or protocol fee formulas.
The most fundamental factor for gas fees is the computational resources it consumes
which is represented as the transactions "weight".
The weight of a transaction is converted into an appropriate amount of PICA by the polynomial formula
which changes dynamically depending on the target load of the network.
This means as the usage of the chain increases towards maximum capacity, the price of a unit of weight increases as well.

## Oracle staking
Apollo is our permissionless, MEV-resistant oracle solution. 
Anyone can run an oracle node on Picasso by providing a PICA stake.

## Collator staking
25% of fees on Picasso is distributed to collators, with the remaining 75% going directly to the community-governed treasury.
Collators on Picasso are required to put down a stake to produce blocks on our parachain, as with most proof of stake networks.

## Stake for xPICA: Financial NFTs

xPICA is PICA that has been staked for a financial NFT (fNFT). 
fNFTs are a novel concept that provides a tradeable representation of a time-weighted staking position. 
The benefits of time-weighted staking are increased rewards and governance power proportional to a specified lock period. 
Upon locking, the PICA tokens themselves become non-transferable until the end of the lock period. 
The fNFT then serves as a voucher, the holder of which can:

1. Benefit from the rewards associated with the staking position itself and redeem for the locked PICA at the end of the lock period
2. Or, should they choose, to trade the fNFT to exit their position early. 
   Thus allowing users to exit their position before their lock period expires, 
   transferring the benefits and ability to redeem the fNFT for PICA to the new holder.

## Primary pairing on Pablo

Pablo is the native DEX of the Picasso ecosystem and is integrated directly into the runtime of our parachain as a pallet.
As such, a primary trading pair on Pablo will be PICA with some of the first pairs available being PICA/USDT and PICA/KSM. 
You can also expect various liquidity incentives with 15% of PICA’s supply being allocated towards liquidity programs on Picasso. 
Overall, Pablo will offer users:

- Bonding mechanisms
- Permanent, protocol-owned liquidity
- LBPs
- Yield farming opportunities

## Governance & Picasso treasury

The treasury will be community owned and controlled by PICA token holders.

Picasso is waging war on centralization with our vision of a 100% trustless future for DeFi, 
entirely removing any drop of centralization from our parachains. 
As such the PICA token will play an important role in helping to realize this vision, 
as governance will be handed over to the community after the core infrastructure is in place. 
PICA holders will then be responsible for governing the network by submitting proposals for

- Election of council members
- Which pallets are incorporated into Picasso’s runtime
- Which pallets will graduate to the Composable parachain on Polkadot
- Which pallets will live only on Picasso, and which will live on both chains
- Directing treasury initiatives
- Which channels are opened through XCM
- What chains and channels are supported on Centauri, our IBC<\>Substrate bridge <!--avoids MDX false positive-->
- And any other action/feature the community decides to implement into the network.


## Ecosystem growth incentives
Holders and stakers of PICA and PBLO could become eligible for unique airdrops of protocols that are newly deploying 
onto the Picasso ecosystem. This may not only provide a unique benefit for holders and stakers of PICA and PBLO, 
but could also provide increased pressure to hold and stake in anticipation of future opportunities. 
Furthermore giving protocols that deploy onto Picasso a unique opportunity to enter the ecosystem with airdrops 
to supporters of the ecosystem.  
