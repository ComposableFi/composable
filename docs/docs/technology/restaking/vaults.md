# Solana Restaking Vaults

The [Restaking Vaults on Solana](https://github.com/ComposableFi/emulated-light-client/blob/master/solana/restaking/README.md) offers users a secure and flexible way to **stake various Solana ecosystem tokens and delegate tokens to validators in order to secure the [AVS for Solana IBC](../ibc/restaking/sol-ibc-avs.md)**. Understanding the processes outlined in this documentation will enable users to be a part of implementing [IBC on Solana](../ibc/solana.md) and engage with the Restaking vaults effectively.

The Restaking Vaults on Solana provide users the opportunity to stake SOL (Solana) and earn additional yield on Solana LSTs (Liquid Staked Solana tokens). The vaults are currently live on [mantis.app](https://www.mantis.app/) and will remain open until the launch of [Solana IBC](../ibc/solana.md). Once launch occurs, the assets will be assigned to secure the [AVS for Solana IBC](../restaking/sol-ibc-avs.md). 

:::note
Deposits in the vault will remain locked until the implementation of IBC on Solana. After this point, users can withdraw their deposits at any time; however, they need to observe a 7-day unbonding period on the AVS for Solana IBC before receiving their tokens.
:::

## Token Types
Users can stake SOL and SOL LSTs (jitoSOL, mSOL and bSOL) in phase 1 of the restaking vault, and in phase 2, Orca LP tokens and receipt tokens from other platforms will also be eligible for staking. When users stake into the vault, they receive an NFT as a receipt token. This NFT serves as a unique identifier for the staked tokens, similar to the Uni V3 NFT receipt tokens. Once users stake their tokens in the vault, they are locked up until the launch of IBC on Solana. 

Each of the following tokens represents a significant market that can now be restaked on [mantis.app](https://www.mantis.app/). If you hold any of these tokens, you can enhance your yield by restaking them to provide security for the AVS for Solana IBC.

**SOL:**
As mentioned, Solana (SOL) is the native token of the Solana ecosystem. Its total market cap at the time of writing is over $28 billion dollars, making SOL the 6th largest token in terms of total market cap (as per CoinMarketCap). Also as mentioned, 92% of SOL is presently staked, but the remaining 8% of the circulating amount is unstaked, and still represents a huge market (around $3 billion). Thus, we are accepting SOL staking in addition to restaking options.

**mSOL:**
mSOL is the liquid staking token from [Marinade Finance](https://marinade.finance/), a Solana stake automation protocol monitoring all Solana validators and delegating to those that are the highest performing (e.g. provide the largest yield to users). [Blockworks Research](https://www.blockworksresearch.com/research/marinade-finance-the-base-layer-for-solana-defi) has described this token as the “...base layer of Solana DeFi”. According to Marinade’s website, they have a TVL of $659 million (nearly 10 million SOL staked), and provide users with an impressive 8.87% APY.

**jitoSOL:**
jitoSOL is the liquid staking token from [Jito](https://www.jito.network/), which [describes itself](https://www.jito.network/docs/jitosol/overview/) as “Solana’s first staking product including MEV rewards”. Users deposit SOL into Jito’s liquid staking pool, and then SOL is delegated to validators on Solana that meet minimum criteria for performance and network resiliency. Jito’s total TVL is 6,352,926 SOL, and they provide stakers with a 6.97% APY ([as per Jito’s website](https://www.jito.network/stats/)).

**bSOL**
BlazeStake is a Solana staking protocol that provides bSOL (liquid staked SOL tokens) that are usable in DeFi platforms. BlazeStake decentralizes Solana by automatically distributing staked SOL among validators. bSOL's value grows each epoch relative to staking APY, backed by compounding staking rewards in SOL.

## Delegation Options
Upon the launch of the bridge, users have the option to delegate their staked tokens to a validator. Users can choose to delegate to a validator of their choice, delegate to their own validator, or if they opt for neither of these, a validator will be randomly delegated the tokens.

Users who wish to remove their stake from the vault after IBC is live must wait until the 7 day unbonding period of the AVS for Solana IBC has elapsed. 

## Receipt Token
Once users deposit stake into the vaults, they receive a unique NFT which represents the value of their stake. To ensure accurate tracking of rewards, the decision to use NFTs for Receipt Tokens is crucial. Fungible tokens cannot make them transferable because the state would have to be connected to the staker’s public key. If they are made transferable, rewards cannot be tracked.

The NFT would be used to derive the seeds of an account which would store the following:

- Stake amount
- Stake token mint
- the last time they received the rewards
- validator pubkey

NFTs can be easily transferred to anyone while retaining the state information. The NFT holder is the only account that has the ability to claim the rewards. 

## Staking Process 

1. The following process is how a user would deposit their stake, either **SOL, JitoSOL, mSOL or bSOL**
2. An NFT is minted and a new Program Derived Address (PDA) is created with the NFT mint as the seed. The stake is updated through a CPI call to the AVS for Solana IBC chain program. The PDA would store the following data:
   - stake amount
   - stake mint
   - last epoch height at which rewards were claimed
   - validator pubkey

3. The user can claim the rewards if they own the receipt NFT. The rewards would be calculated from the height at which it was last claimed. If claiming for the first time, it will return all the rewards and store the current epoch height in the storage.

4. While withdrawing, the rewards and stake are returned to the user and the receipt NFT is burnt.

![vault-staking-flow](../restaking/flow.png)

:::tip Fractionalisation
At a later time, fractionalisation will be introduced. This will allow users to transfer a part of their receipt token to another account and split the stake in the ratio mentioned and maintain the same rewards epoch height. After which each user can claim their rewards separately. Please note this is the only way a user can transfer a part of their receipt token.
:::