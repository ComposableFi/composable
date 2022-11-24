# PICA use cases

PICA is the native token of the Picasso parachain. 
We have made a concerted effort to ensure that the PICA token withholds as much utility as possible by incorporating 
various value accrual methods, governance features, and time-weighted benefits in the form of fNFTs. 
While the PICA token provides the community a strong voice and rewards for participating within the ecosystem, 
it is also fundamental for the operation of collators and oracles. Thus, the PICA token is fundamental for 
governance, network usage, and the security of Picasso.

## Gas fees

Transaction fees on Picasso are payable in PICA. Of the PICA paid in transaction fees, 75% is reserved for the community
governed treasury and 25% to collators In addition to transfers and smart contract fees on Picasso itself, 
Picasso is uniquely positioned to take advantage of cross-chain bridging fees via Centauri and the facilitation of 
cross-chain smart contract calls via XCVM. All of these fees will follow the same outline mentioned above, 
with 75% going to the treasury and 25% to collators.

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

## Governance
When the First Council is able to confirm proper operation of the Picasso parachain (estimated for the end of Release 2),
sudo privileges will be burned and governance will be handed over to the community. 
PICA token holders will then decide on:

- Election of subsequent council members
- Which pallets are incorporated into Picasso’s runtime.
- Which pallets will graduate to the Composable parachain on Polkadot.
- Which pallets will live only on Picasso, and which will live on both chains.
- Directing treasury initiatives
- Which channels are opened through XCM
- What chains and channels are supported on Centauri, our IBC<\>Substrate bridge <!--\ prevents JSX false positive-->

## Collator staking

Collators for the Picasso parachain are required to put down a stake to produce blocks on the Picasso parachain. 
In doing so, collators will earn PICA through transaction fees.

## Oracle staking
Oracle operators are required to put down stakes to provide price feeds. 
They will be rewarded/ slashed according to the accuracy of the data they provide.

## Pablo
Pablo is Picasso’s flagship DEX which utilizes novel bonding and protocol owned liquidity mechanisms to ensure deep 
liquidity and transaction fulfillment with minimal slippage. As such, Pablo will be the first place users can go to 
swap tokens, provide liquidity, or bond their LPTs in return for PBLO.

## xPICA (fNFTs)
Financial NFTs (fNFTs) are another novel use case which will be available to PICA users. 
In short, fNFTs are a time weighted value accrual and governance mechanism which increases a user’s rewards and voting 
power relative to their lock period. fNFTs are then minted as a tradeable representation of this position, 
allowing users to enter or exit positions freely.