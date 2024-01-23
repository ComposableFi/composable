# Tools
## Development Progress
As of November 15, 2023 MANTIS is in mainnet testing. Details of this testing period are as follows:

Composable is running MANTIS and the Composable Virtual Machine (CVM) on mainnet. Osmosis, Neutron, and the Composable Cosmos chain will be accessible for cross-chain operations on the mainnet testing. The flow and components of the testing are as follows:
- We will create faucets for people to use
- There is a place where solvers can connect to the mainnet testing over an RPC
- There is a place where problems can be submitted by users over an RPC

Additional public details of this testing can be found on [this notion page](https://www.notion.so/composablefoundation/Banksy-Testnet-4-72b7f8317bcf406e9059ae258733eb31), which includes specifications and setup/joining instructions. The code repo is available on [this GitHub page](https://github.com/notional-labs/composable-networks/tree/main/banksy-testnet-4).

## Testing Period
With mainnet deployment, we can begin getting user traction and initial use of MANTIS via a campaign “game”. In this campaign, users will be able to submit problems/intentions through the frontend. We will provide test tokens for this purpose, which users will be able to deposit in a basic user interface. 

Users will be given a small amount of PICA token (the native token of the Composable Cosmos chain and Picasso parachain on Kusama). Then, users can participate in the intention submission process with this PICA. A number of solvers on MANTIS will then solve and fill these orders in mainnet testing. These solvers will have needed to onboard with Composable prior to this testing period. 

At this point, we will also need validators running in order to support the network. We will have existing oracles and collators run these validators.

Experiment/Test details are as follows:
- The goal is to have thousands of problems/intentions sent through MANTIS experimentally
- Swaps will occur between Composable and Neutron or Osmosis
- After this campaign, we will publish the results of these two weeks of mainnet testing and then ideally go to full production with Osmosis and Neutron etc.

We will measure:

- Swap volume
- Volume/proportion of solutions that were solved with coincidence of wants (CoWs) matching
- Volume/proportion of solutions that were solved with constant function market makers (CFMMs)
- Volume/proportion of solutions that were solved with solvers’ own liquidity

The timeline for this is December 2023.

Interested in participating as a solver? Join our solver channel on telegram [here](https://t.me/+Z69AYRzVTLVhNTk5). 

## MANTIS Games
The MANTIS app will have many challenges designed to incentivize participation in the protocol.

These “MANTIS Games” involve the following phases, all designed to make participation maximally enjoyable and rewarding:

### Phase 1 - NFT Auction 
Here, we introduce teams: during the course of MANTIS games, users can register for a team on mantis.app. Along with your team, you will be able to participate in various competitions on MANTIS.

In the first phase, NFTs were auctioned on the [Tensor marketplace](https://www.tensor.trade/) to serve as a mechanism for team leaders to create teams and compete within the Mantis platform. Only NFT holders can be team leaders and this will benefit you by accumulating revenue from your team members. If you are in the winning team, you will get the largest stake of reward in a given rewards pool. Moreover, purchasers of the NFT will get a percentage of the NFT sale. 

Users without an NFT can join an existing team by getting a referral code from a team leader. You’ll still be eligible for boosted rewards if your team wins.

The remaining 85% of the rewards will be distributed according to the percentage of the TVL that each group member contributed to his team’s total TVL. 

### Phase 2 - Team Staking 
In phase 2, once teams have been formed from phase 1, these teams will compete for who can have the highest amount of assets restaked into the MANTIS app (restaked assets could include mSOL, jitoSOL, etc.); the MANTIS application frontend will allow for staking into the contract for restaked assets. Information on getting a head start on the restaking process by participating in our Solana-IBC vault is available [here](https://medium.com/@Picasso_Network/restaking-vault-and-team-staking-competition-3f9dcfdc01cf). 

After a competition period of a few weeks or a month, the number 1 winning team will get a significant/elevation portion of bridge revenue for a period of time (exact details are to be determined).

### Phase 3 - Swap Competition 
In this phase, MANTIS games will be full fledged. We aim to generalize competition beyond staking. Here, we introduce users’ “Degen scores” (a referral score for on-chain usage and asset capitalization; users can share this code to be followed, so others can replicate their specific intent passageways). Judgements in the competition are based on this score and other factors.