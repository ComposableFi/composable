# Composable CosmWasm VM

## Overview of Composable CosmWasm VM (ccw-vm)

Composable is the first team building a CosmWasm virtual machine (VM) outside the Cosmos ecosystem that 
developers can use to build with all the benefits of CosmWasm (CW) and the Cosmos Network.

Confio's current implementation of CW is written half in Rust and half in Golang, 
which means it is not compatible with Substrate and has never been used outside Cosmos. 
Also, Confio's CW VM excludes library components but consists of a single implementation targeting the Cosmos SDK.

Our team is creating a portable library version of CosmWasm that can be hosted on different implementations to 
reduce the effort required to port CosmWasm to other ecosystems. 
[Composable CosmWasm VM] (ccw-vm) has two host implementations which 
allow developers to easily integrate CosmWasm into their ecosystems.
This ensures that ccw-vm is highly portable and has consistent execution semantics regardless of its host. 
As a result, developers can integrate ccw-vm as a pallet, within a frontend app, or inside a CLI tool.

[Composable CosmWasm VM]: https://github.com/ComposableFi/cosmwasm-vm

## Value proposition for CosmWasm projects deployed on Cosmos

Building a CosmWasm virtual machine (VM) will allow developers to run smart contracts directly on [Picasso], 
our chain on Kusama. 
This provides novel opportunities, such as enabling existing Cosmos projects and CW developers to run smart contracts in 
the Polkadot & Kusama (DotSama) ecosystems simultaneously.

Accessible, interoperable smart contract creation is crucial for developing ecosystems, especially nascent ecosystems, 
that will play a significant role in the future of DeFi.

[Picasso]: https://picasso.xyz/

## Overview of Components

CosmWasm on Picasso can be divided into three components: 

- **CosmWasm VM:** An abstract engine that enables other applications to run CosmWasm programs. 
    Pallet CosmWasm is an application of the CosmWasm VM.
- **Pallet Cosmwasm:** Similar to a Cosmos SDK module, Pallet CosmWasm is the Substrate version of a CosmWasm module. 
    It uses the CosmWasm VM under the hood, and implements it to be able to run CW contracts on Picasso.
- **CW-CLI:** A tool for interacting with the Pallet CW, 
    both to get started with CosmWasm and to interact with the CW Pallet in Picasso. 
    Essentially, it is a communicator and helper tool. It can be used in local development and called by other programs. 
    Anyone using CW can use the CW-CLI, and it is not specific to Picasso.

## Overview of Developer Tools

The following tools are a beta launch for CosmWasm developers. 
Users who wish to leave feature requests and feedback are welcome to raise [Github issues]. 
These tools are not exclusive for developers building on Picasso, but rather those building with CosmWasm in general.

**[CosmWasm-Orchestrate]:** CW Orchestrate is a tool for testing and simulating CW contracts. 
By integrating our CosmWasm Orchestrate library into developers' CosmWasm contracts, 
they can easily test in-memory simulations of real chain implementations executing their contracts, 
from simple to the most intricate orchestrations, all while being IBC supported. 
Furthermore, with CosmWasm Orchestrate, developers can write tests programmatically in 
the Continuous Integration (CI) pipeline and test them in a straightforward manner.


**[Smart Contract Interface]:** With Composable Finance’s Smart Contract Tooling Interface, 
we have abstracted the process of using CosmWasm and made it accessible through a browser. 
This allows developers to easily upload their smart contracts and instantly test them, 
saving time and reducing the overhead of interacting with a blockchain. 

Read our introductory thread for a preview of the [CosmWasm VM smart contract tooling interface].

[Github issues]: https://github.com/ComposableFi/composable/issues
[CosmWasm-Orchestrate]: https://github.com/ComposableFi/cosmwasm-vm/tree/main/orchestrate
[Smart Contract Interface]: https://tools.xcvm.dev/
[CosmWasm VM smart contract interface]: https://twitter.com/ComposableFin/status/1600538100282761216?s=20&t=NkUt9w8mush_wmMpkph7xw
