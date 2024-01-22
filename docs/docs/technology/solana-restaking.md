# Solana Restaking

The first opportunity for restaking on Solana via Picasso is launching soon, and we’re giving users a chance to get in on the action early and earn boosted rewards through a Restaking Vault and team staking competition.

:::note
As previously [announced](https://twitter.com/Picasso_Network/status/1734941879068762305) and [described](https://medium.com/@Picasso_Network/restaking-is-coming-to-solana-via-picasso-5ea0b027d269), Picasso is introducing the DeFi primitive of restaking into the Solana ecosystem via the Solana IBC connection on [trustless.zone](https://www.trustless.zone/). To allow users to participate in this opportunity as early as possible, we are opening a restaking vault prior to the launch of Solana IBC. Restaked assets will then be delegated to validators upon launch. 

From here, users will not only begin earning staking rewards, but also they will get a head start in a new team staking competition. In this competition, teams of users will compete to earn boosted transfer revenues.
:::

## The Importance of Restaking
Restaking has been [described as a new primitive](https://consensys.io/blog/eigenlayer-a-restaking-primitive) in crypto economic security that enables the rehypothecation of a token on the consensus layer. Specifically, the process of staking involves a user staking an ecosystem’s native asset to that ecosystem’s validators. The user then receives a receipt token representing this stake. They then “restake” this receipt token with validators again. This mechanism enables users to multiply the crypto economic security (and the yield) of their initial tokens, as they are essentially able to stake the same assets twice, receiving yield and supporting PoS validation both times.

Restaking has been pioneered and popularized by [EigenLayer](https://www.eigenlayer.xyz/), which is a protocol for restaking ETH on Ethereum. In particular, users staking ETH are able to opt into EigenLayer’s smart contracts for restaking their ETH and thus extending the crypto economic security to additional applications within the ecosystem. EigenLayer thus addresses rising concerns of fragmented security on Ethereum, helping to bootstrap the security of various protocols/applications. [EigenLayer’s total value locked (TVL)](https://defillama.com/protocol/eigenlayer) at the time of writing is over $275 million, indicating that there is a clear demand for restaking.

Despite the benefits of restaking, this concept has largely not yet expanded beyond the Ethereum ecosystem. However, there is a huge potential for restaking on other chains. This is particularly true on Solana, where there is a massive amount of staking occurring, with many prominent staking protocols already offering liquid staking tokens (LSTs) and receipt tokens that can be used for various purposes while a user’s original assets remain staked. In fact, at the time of writing, [over 392 million SOL are staked](https://solanacompass.com/statistics/staking), representing a staking market capitalization of over $25 billion dollars. This is a staggering 92% of the total circulating supply of SOL. Therefore, there is an incredibly large market for restaking these assets that are already staked in Solana. Yet, there has been very little use for these receipt tokens - until now.

## Why a Restaking Vault?
For the strength and security of the network, it is important to have validators powering the Solana IBC connection (e.g. validators on the guest blockchain) bootstrapped when the connection goes live. Thus, the chain is secured via Proof-of-Stake (PoS). To meet this need, we have created a restaking vault solution that will provide validators with an initial supply of (re)stake. 

Moreover, the restaking vault allows users to beat the crowd and participate in restaking in the Solana ecosystem before everyone else - and get rewarded for doing so. 

## How the Vault Works

Here’s how you can make use of the restaking vaults:

1. Navigate to the restaking vaults and ensure your Solana wallet is connected
2. Optional: select a team to join (more details on teams can be found below)
3. Deposit some mSOL or jitoSOL
4. You can acquire mSOL by staking on Marinade Finance here
   - You can acquire jitoSOL by staking on Jito here
   - Your deposit will be locked until the launch of the Solana IBC connection
5. At the launch of the connection, all deposited assets in the vault will be delegated to validators for the Solana IBC connection, helping to secure the network
6. Your assets will then be designated as restaked with validators
7. You’ll start accruing restaking rewards proportionate to the amount of assets and time you’re staked, in addition to a bonus for being a vault participant
8. You can withdraw your restaked assets at any time after the launch of Solana IBC. You must wait for the guest blockchain's unbonding period to receive your tokens.

## How Restaking Will Work

Restaking is a critical part of the guest blockchain mechanism that facilitates the Solana IBC connection. Essentially, the guest blockchain serves as an L2 of Solana, and this network needs to be validated like any other chain using the proof-of-stake (PoS) model, as Solana does. 

Specifically, on the guest blockchain, previously staked assets are restaked with validators to secure the network. The security model involves control by a supermajority of nodes/validators on the guest blockchain. It is the nodes’ responsibility to sign corresponding payloads of transfer transactions. To join, a validator must provide a bonded stake. Thus, this model is gated from independent actors joining. Validators in the guest blockchain will be rewarded with a portion of bridging gas/transaction fees.

:::info
Picasso will accept staking of both Solana’s native SOL token as well as restaking of various receipt tokens for SOL staking platforms. These tokens can be staked with validators of the guest blockchain powering the Solana IBC connection. Users will be able to stake from trustless.zone, depositing their assets from their connected digital wallets into the staking contracts. From these contracts, assets will be delegated to validators of [the guest blockchain](https://research.composable.finance/t/crossing-the-cross-blockchain-interoperability-chasm/33) that supports the IBC Solana connection. Thus, restaking in this manner will support the guest blockchain along the premise of PoS, which enhances the security of this connection.
:::

In this mechanism, it is critical that we properly determine the value of these restaked tokens. To accomplish this, oracles will need to be utilized to query different token pricing. The oracles can provide price feeds on token pairs, eg. stETH / ETH and provide a reasonable estimate of the current value based on the swap price. 

Users will accumulate staking rewards proportionate to their staking amount and time. Thus, they can receive not only the yield on their original stake, but also the yield from restaking.

Below are some of the initial assets that can be staked on Picasso’s Solana IBC validators:

**SOL:**
As mentioned, Solana (SOL) is the native token of the Solana ecosystem. Its total market cap at the time of writing is over $28 billion dollars, making SOL the 6th largest token in terms of total market cap (as per CoinMarketCap). Also as mentioned, 92% of SOL is presently staked, but the remaining 8% of the circulating amount is unstaked, and still represents a huge market (around $3 billion). Thus, we are accepting SOL staking in addition to restaking options.

**mSOL:**
mSOL is the liquid staking token from [Marinade Finance](https://marinade.finance/), a Solana stake automation protocol monitoring all Solana validators and delegating to those that are the highest performing (e.g. provide the largest yield to users). [Blockworks Research](https://www.blockworksresearch.com/research/marinade-finance-the-base-layer-for-solana-defi) has described this token as the “...base layer of Solana DeFi”. According to Marinade’s website, they have a TVL of $659 million (nearly 10 million SOL staked), and provide users with an impressive 8.87% APY.

**jitoSOL:**
jitoSOL is the liquid staking token from [Jito](https://www.jito.network/), which [describes itself](https://www.jito.network/docs/jitosol/overview/) as “Solana’s first staking product including MEV rewards”. Users deposit SOL into Jito’s liquid staking pool, and then SOL is delegated to validators on Solana that meet minimum criteria for performance and network resiliency. Jito’s total TVL is 6,352,926 SOL, and they provide stakers with a 6.97% APY ([as per Jito’s website](https://www.jito.network/stats/)).

**Orca LP Tokens**
[Orca](https://www.orca.so/) is a decentralized exchange (DEX) on Solana where every trade supports charities fighting climate change. Users providing liquidity into [Orca’s concentrated liquidity pools](https://v1.orca.so/liquidity) receive liquidity provider (LP) tokens to represent their deposits. The TVL in these liquidity pools is over $85 million, providing LPers with approximately $200,000 in weekly rewards.

Each of these tokens represents a significant market that can now be restaked to Picasso’s Solana IBC validators. If you already hold any of these tokens, or if we’ve inspired you to acquire some, we hope you’ll consider restaking them with us to support Picasso’s Solana IBC connection and to enhance your own yield.

## Multisig signers
The restaking layer will initially be governed by two multisigs with limited abilities until a decentralised governance system is established.

1. The **Admin multisig** is responsible for the following:

- Whitelisting tokens
- Setting the staking cap
- Setting if the guest chain is initialised or not
  
2. The **Upgradability multisig** holds the upgrade authority. This multisig has the ability to upgrade the restaking contract and no other powers.
