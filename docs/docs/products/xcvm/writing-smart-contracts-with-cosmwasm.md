# Writing Smart Contracts with CosmWasm

## CosmWasm’s Advantages

Composable selected CosmWasm as the developer framework for smart contract deployment on the XCVM as it offers 
cross-chain support, is tightly compatible with Cosmos chains, and has superior security design.

[CosmWasm](https://cosmwasm.com/) is an abbreviation derived from the combination of the name 
[Cosmos](https://cosmos.network/) and the abbreviation for [WebAssembly](https://webassembly.org/), Wasm. It implements 
the WASM smart contract engine for the Cosmos SDK and runs on the IBC. This allows projects using CosmWasm to 
communicate cross-chain between all of the chains linked on the IBC protocol. It enables dApps to function as smart 
contracts on Cosmos chains, without the need to develop a new chain.

A key feature of CosmWasm is that it allows for multi-chain contracts; one dApp, one contract, but multiple chains. Any 
chain using the Cosmos SDK can easily integrate the CosmWasm module, which is built to maintain low overhead on systems 
demand. CosmWasm allows for composition across multiple chains and migration to other chains, with built-in, 
permissioned, pre-contract migration functionality. As such, CosmWasm is designed to connect different blockchains, and 
allows users to benefit from the utility across chains instead of being forced to choose between them.

In terms of security, CosmWasm is designed to avoid the attack vectors found in 
[Ethereum and Solidity](https://docs.cosmwasm.com/docs/0.16/architecture/smart-contracts), such as Reentrancy, 
Arithmetic Underflows/Overflows, and Default Visibilities. Its security is further reinforced by its 
[well-developed tooling and testing mechanisms](https://medium.com/cosmwasm/cosmwasm-for-ctos-i-the-architecture-59a3e52d9b9c) 
[prioritized early in its conception](https://medium.com/cosmwasm/cosmwasm-for-ctos-f1ffa19cccb8).

Due to these benefits, many DeFi projects have adopted CosmWasm as their smart contract framework, including notable 
examples like Terra, OKX, Crypto.org, and Osmosis.


## CosmWasm as the Developer Framework for the XCVM

CosmWasm’s inter-chain contract support works hand in hand with XCVM’s interoperable smart contract functionality. 
Through the adoption of CosmWasm, the XCVM will be the first to bring its functionality to the DotSama ecosystem. 
This will enable the XCVM to combine the benefits of both ecosystems and connect CosmWasm to a broader range of DeFi 
ecosystems. As such, developers will be able to write smart contract for DotSama, that exist as multi-chain contracts on
IBC-Cosmos. Accessible, interoperable smart contract creation is crucial for developing ecosystems, especially nascent 
ecosystems, that will form a considerable role in DeFi’s future. These ecosystems and protocols that exist cross-chain 
suffer from the fragmented liquidity of the DeFi space. As such, they need virtual machines like the XCVM that enable 
multi-chain contracts and cross-chain communication.

CosmWasm’s customizability and use of the Rust programming language makes it optimal for adoption as a developer 
framework for the XCVM. Compared to Solidity, Rust makes it easier for developers to write secure code and comes with 
stronger tooling support. This makes it well-suited to the development of infrastructure projects and the language of 
choice in the DotSama ecosystem.

CosmWasm is defined by its interoperability. Polkadot is defined by its shared security. By merging the two, the XCVM is
able to leverage their advantages to create a novel smart contract platform within the DeFi space that is deeply 
interoperable and highly secure. Thus, the XCVM can act as an easy portal for existing or new protocols to participate 
in the ever-expanding cross-chain movement, augmenting the unification of the DeFi industry and helping deliver the 
ultimate goal of chain agnosticism in DeFi.
