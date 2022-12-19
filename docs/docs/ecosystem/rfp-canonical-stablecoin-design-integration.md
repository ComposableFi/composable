# Request for Proposal: Canonical Stable Coin Design and Integration

## Introduction:
Composable Finance is seeking proposals from qualified blockchain developers to design and integrate 
a Proof-of-Concept for a custom stablecoin for multi-chain usage using our proprietary 
[Cross-Chain Virtual Machine(XCVM)]. 
The goal of this project is to allow projects to create and maintain mint and burn privileges 
for issuing truly canonical stablecoins across different chains.

## Requirements & Scope of Work:
The purpose of this RFP is to build a proof of concept for a XCVM-powered stablecoin for multi-chain usage.

Requirements for creating a custom XCVM-powered stablecoin for multi-chain use are as follows:
- Design and develop a custom stablecoin for multi-chain usage using our technology stack, specifically the XCVM
- A minting contract must be on each chain that you want to mint your coin on
- An XCVM contract must be on the chain of your choice in order to control mint and burn via 
  [Inter-Blockchain Communication(IBC)] Protocol messages between chains
    - This will handle the actual accounting of the stablecoin

## Evaluation Criteria:
- Proposals will be evaluated based on the following criteria:
- Quality of past work and experience with similar projects
- Understanding of the project scope and ability to meet project requirements
- Proven ability to deliver projects on time and within budget
- Quality and clarity of the proposal
- Cost

## How it Works:

Despite the importance of stablecoins in the industry, these tokens have had very limited cross-chain capability. 
Now, with the introduction of our XCVM, 
Composable enables projects to maintain mint and burn privileges for issuing truly canonical stablecoins across chains.
How this works is depicted below, 
using an example of the movement of an XCVM-based canonical stablecoin between two chains (“chain A” and “chain B”). 
USDC is used as the example stablecoin, though custom coins can be created.

![high_level_crosschain_canonical_stablecoin](./high-level-crosschain-canonical-stablecoin.png)

As you can see, the minting contract exists on one chain, 
and is controlled solely by the project offering the stablecoins. 
The minting contract is able to mint this coin on chain A whenever it is programmed/instructed to. 
This minting contract interacts with an interpreter contract also on chain A. This then messages the XCVM contract, 
which leverages the IBC to in turn message another interpreter on chain B. 
This triggers a separate minting contract on chain B, minting the stablecoin onto chain B.
This simply and efficiently moves stablecoins across different chains for storage and utility
in whatever manner the user wants, in a completely decentralized manner.

## Submission Requirements:

Please submit the following materials as part of your proposal:

- A detailed project plan outlining your approach to the stablecoin design and integration project, 
    including timeline and deliverables
- A detailed cost breakdown, including any additional fees or expenses

## Contact Information:

If you have any questions about the RFP or the project, please contact us at outreach@composable.finance

[Cross-Chain Virtual Machine(XCVM)]: (https://docs.composable.finance/products/xcvm)
[Inter-Blockchain Communication(IBC)]: https://ibcprotocol.org/
