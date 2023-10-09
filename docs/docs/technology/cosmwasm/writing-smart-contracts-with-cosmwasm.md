# Writing Smart Contracts with CosmWasm

## CosmWasm’s Advantages

Composable selected CosmWasm as the developer framework for smart contract deployment on the CVM as it offers 
cross-chain support, is tightly compatible with Cosmos chains, and has superior security design.

[CosmWasm](https://cosmwasm.com/) is an abbreviation derived from the combination of the name 
[Cosmos](https://cosmos.network/) and the abbreviation for [WebAssembly](https://webassembly.org/), Wasm. It implements 
the WASM smart contract engine for the Cosmos SDK and runs on the IBC. This allows projects using CosmWasm to 
communicate cross-chain between all of the chains linked on the IBC protocol. It enables dApps to function as smart 
contracts on Cosmos chains, without the need to develop a new chain.

CosmWasm has a significant feature that enables multi-chain contracts, where a single contract can be used across multiple chains in one dApp. With the use of the Cosmos SDK, any chain can easily incorporate the CosmWasm module with minimal system overhead. Our development of pallet-CosmWasm has made it possible for any parachain to integrate and implement the CosmWasm framework on their chain. This integration provides a composition of multiple chains and migration functionality across different chains with pre-contract permission. CosmWasm is designed to connect various blockchains, allowing users to benefit from the utility of multiple chains without the need to choose one over the other. The framework also includes built-in, permissioned, pre-contract migration functionality.

In terms of security, CosmWasm is designed to avoid the attack vectors found in Ethereum and Solidity, such as Reentrancy, 
Arithmetic Underflows/Overflows, and Default Visibilities. Its security is further reinforced by its 
[well-developed tooling and testing mechanisms](https://medium.com/cosmwasm/cosmwasm-for-ctos-i-the-architecture-59a3e52d9b9c) 
[prioritized early in its conception](https://medium.com/cosmwasm/cosmwasm-for-ctos-f1ffa19cccb8).

Due to these benefits, many DeFi projects have adopted CosmWasm as their smart contract framework, including notable 
examples like Neutron, Secret Network, OKX, Crypto.org, and Osmosis.


## CosmWasm as the Developer Framework for the CVM

CosmWasm’s inter-chain contract support works hand in hand with CVM’s interoperable smart contract functionality. 
Through the adoption of CosmWasm, the CVM will be the first to bring its functionality to the DotSama ecosystem. 
This will enable the CVM to combine the benefits of both ecosystems and connect CosmWasm to a broader range of DeFi 
ecosystems. As such, developers will be able to write smart contract for DotSama, that exist as multi-chain contracts on
IBC-Cosmos. 

Accessible, interoperable smart contract creation is crucial for developing ecosystems, especially nascent 
ecosystems, that will form a considerable role in DeFi’s future. These ecosystems and protocols that exist cross-chain 
suffer from the fragmented liquidity of the DeFi space. As such, they need virtual machines like the CVM that enable 
multi-chain contracts and cross-chain communication.

CosmWasm’s customizability and use of the Rust programming language makes it optimal for adoption as a developer 
framework for the CVM. Compared to Solidity, Rust makes it easier for developers to write secure code and comes with 
stronger tooling support. This makes it well-suited to the development of infrastructure projects and the language of 
choice in the DotSama ecosystem.

CosmWasm is defined by its interoperability. Polkadot is defined by its shared security. By merging the two, the CVM is
able to leverage their advantages to create a novel smart contract platform within the DeFi space that is deeply 
interoperable and highly secure. Thus, the CVM can act as an easy portal for existing or new protocols to participate 
in the ever-expanding cross-chain movement, augmenting the unification of the DeFi industry and helping deliver the 
ultimate goal of chain agnosticism in DeFi.
