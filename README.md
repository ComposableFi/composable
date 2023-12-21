<br />

<p align="center">
  <img alt="Composable Finance" title="Composable Finance" src="./docs/banner.png">
</p>

<br />

# Composable Finance | Picasso

[![Latest Release](https://img.shields.io/github/v/tag/composablefi/composable)][latest-url]
![Build][build-badge]
[![Discord][discord-badge]][discord-url]

[latest-url]: https://github.com/composablefi/composable/tags
[build-badge]: https://github.com/composablefi/composable/actions/workflows/check.yml/badge.svg

[discord-badge]: https://img.shields.io/badge/Discord-gray?logo=discord
[discord-url]: https://discord.gg/composable

[mergify]: https://dashboard.mergify.com/github/ComposableFi/repo/composable/queues
[mergify-status]: https://img.shields.io/endpoint.svg?url=https://api.mergify.com/v1/badges/ComposableFi/composable&style=flat

Composable Finance is dedicated to building the infrastructure for trust-minimized interoperability and decentralised block building for efficient cross-chain execution. Within this repository, you'll find the codebase for Composable Polkadot and Picasso. Additionally, it houses pallets included in their respective runtimes.

Among the noteworthy pallets included are:

- [CosmWasm VM for Substrate](./code/parachain/frame/cosmwasm/)
- [Multihop (XCM + IBC)](./code/parachain/frame/pallet-multihop-xcm-ibc/) 
- [Apollo](./code/parachain/frame/oracle/)
- [Pablo DEX](./code/parachain/frame/pablo/)
- [Liquid staking](./code/parachain/frame/liquid-staking/)

Composable's core technology extends beyond this repository. It includes key components such as [CVM and MANTIS](https://github.com/ComposableFi/cvm), the first IBC implementation on Substrate – [Pallet-ibc](https://github.com/ComposableFi/composable-ibc), the [Composable Cosmos chain](https://github.com/notional-labs/composable-centauri), and [Solana IBC](https://github.com/ComposableFi/emulated-light-client).

## Join Composable Networks
To supply price feeds for Apollo or run a collator on Picasso, check out our documentation [here](https://docs.composable.finance/develop/collator-guide). 

To become a validator on Composable Cosmos mainnet or testnet, see the instructions [here](https://docs.composable.finance/develop/composable-cosmos).

## Audits

Within the audits folder, you'll discover a collection of audit reports, along with any corresponding fixes. This includes any audit reports that have been published for any in-production code, and not just limited to this repository. 

## Documentation

This repository serves as a hub for contributions to the Composable documentation. Composable uses Docusauras, for insights into major structural contributions, please refer to their documentation. Should you encounter broken links within the docs, submit an issue or PR to address the matter.

Visit our [docs](https://docs.composable.finance) to learn more about our vision and technology.

## Installation

Refer to the [Releases](https://github.com/ComposableFi/composable/releases) page.

### Release Process

Refer to [RELEASE.md](./RELEASE.MD).

## Nix

Use [`nix`](https://docs.composable.finance/nix) to run and build Composable developer environments.




