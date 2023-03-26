# Request for Proposal: Centauri Explorer

## Introduction:

Composable Finance is expanding IBC to other ecosystems through the development of [Centauri]. Centauri aims to become the primary digital asset transport layer across different chains. This roadmap includes connecting to Ethereum, Starknet, and other light client-based blockchains.

Currently, explorer solutions only cover IBC usage between Tendermint-based Cosmos chains. Centauri will ignite a wave of cross-ecosystem IBC activity. As part of this effort, an explorer will be required to enable: real-time search and access to historical information about a blockchain which includes data related to blocks, transactions, addresses, and more.

Composable is looking to engage builders to lead this effort in the form of an Infrastructure grant. Grant recipient(s) will lead this effort, supported by Composable Finance’s Product and front-end team to deliver this explorer. 

[Centauri]: https://docs.composable.finance/products/centauri-overview

## Objectives:

The core focus should be creating a user-friendly way to display IBC transactions that update in real-time. To make it possible for validators on the Picasso-Cosmos testnet to use it, the page should be connected to LCD restpoints for real-time data.

After the initial testnet stage, a dedicated backend & DB with extended functionality should be implemented. From a UX point of view, the explorer should allow users to track:

- Transaction confirmations
- Relayer status
- Confirmation on source and destination chain
- Transaction volume

All FE work should be done in React, backend should preferably use graphql / hasura (or similar), however, we are open to suggestions and are willing to consider other alternatives.

## Submission Requirements:

Please submit the following materials as part of your proposal:

- A proposed development roadmap outlining your milestones including timeline for delivery
- A detailed cost breakdown, including any additional fees or expenses
- Resourcing requirements from the Composable team (& for what scope)
- Past credentials (e.g. links to relevant past work)

## Contact Information:

If you have any questions about the RFP or the project, please contact us at outreach@composable.finance
