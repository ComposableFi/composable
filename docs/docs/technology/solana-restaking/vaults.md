# Restaking Vaults

Picasso's [Restaking Vaults](https://github.com/ComposableFi/emulated-light-client/blob/master/solana/restaking/README.md) offer users a secure and flexible way to **stake various Solana ecosystem tokens, participate in the launch of IBC on Solana, and delegate tokens to validators**. Picasso's [Restaking Vaults](https://github.com/ComposableFi/emulated-light-client/blob/master/solana/restaking/programs/restaking/src/lib.rs) on Solana provide users the opportunity to stake SOL (Solana) and earn additional yield on Solana LSTs (Liquid Staked Solana tokens) in phase 1, and later, in phase 2, users will also be able to stake Orca LP tokens and receipt tokens from other platforms. The vaults are planned for a January launch where they will stay open until the launch of Solana IBC. Once launch occurs, the assets will be assigned to the guest blockchain layer. It is important to be aware vault deposits will be locked until launch. There will be three rounds of deposits with caps.

Understanding the processes outlined in this documentation will enable users to be a part of implementing IBC on Solana and help navigate engaging with the Restaking vaults effectively.

## Token Types
Users can stake SOL and SOL LSTs in phase 1, and in phase 2, Orca LP tokens and receipt tokens from other platforms are also eligible for staking. When users stake into the vault, they receive an NFT as a receipt token. This NFT serves as a unique identifier for the staked tokens, similar to the Uni V3 NFT receipt tokens. Once users stake their tokens in the vault, they are locked up until the launch of IBC on Solana.

There will be a cap set on the vaults in the following three phases during the launch of the vaults:

- Round 1: 50,000 SOL 
- Round 2: 150,000 SOL
- Round 3: 500,000 SOL 

## Delegation Options
Upon the launch of the bridge, users have the option to delegate their staked tokens to a validator. Users can choose to delegate to a validator of their choice, delegate to their own validator, or if they opt for neither of these, a validator will be randomly delegated the tokens.

Users who wish to remove their stake from the vault after IBC is live must wait until the unbonding period (denoted as "x") of the guest blockchain has elapsed. 

## Receipt Token
Once users deposit stake into the vaults, they receive a unique NFT which represents the value of their stake. To ensure accurate tracking of rewards, the decision to use NFTs for Receipt Tokens is crucial. Fungible tokens cannot make them transferable because the state would have to be connected to the staker’s public key. If they are made transferable, rewards cannot be tracked.

The NFT would be used to derive the seeds of an account which would store the following:

- Stake amount
- Stake token mint
- the last time they received the rewards
- validator pubkey

NFTs can be easily transferred to anyone while retaining the state information. The NFT holder is the only account that has the ability to claim the rewards. 

## Staking Process 

1. The following process is how a user would deposit their stake, either SOL, JitoSOL, mSOL or bSOL
2. An NFT is minted and a new PDA is created with the NFT mint as the seed. The stake is updated throgh a CPI call to the guest chain program. The PDA would store the following data:
   - stake amount
   - stake mint
   - last epoch height at which rewards were claimed
   - validator pubkey

3. The user can claim the rewards if they own the receipt NFT. The rewards would be calculated from the height at which it was last claimed. If claiming for the first time, it will return all the rewards and store the current epoch height in the storage.

4. While withdrawing, the rewards and stake are returned to the user and the receipt NFT is burnt.

![vault-staking-flow](../solana-restaking/flow.png)

:::tip Fractionalisation
At a later time, fractionalisation will be introduced. This will allow users to transfer a part of their receipt token to another account and split the stake in the ratio mentioned and maintain the same rewards epoch height. After which each user can claim their rewards separately. Please note this is the only way a user can transfer a part of their receipt token.
:::