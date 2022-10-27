# Composable Overview

## Problems with a multi-chain DeFi environment

Composable Finance seeks to solve the challenge of siloed and fragmented liquidity brought about by a lack of 
interoperability in the blockchain space.

The DeFi ecosystem has grown to encompass multiple chains. These chains have their own respective value propositions, 
and are optimized for different use-cases. Their existence is not driven by speculation, but by value. 
As such, the DeFi ecosystem has become multi-chain.

The challenge with multiple chains is that, due to the nature of blockchain technology, assets and liquidity on these 
different chains become fragmented between them. Different blockchains, blockchain layers (such as Ethereum Layer 2 
solutions like Polygon), and protocols that exist cross-chain suffer as a result of this fragmented liquidity.
It prevents the effective function of DeFi services and forces DeFi ecosystems into zero-sum competition. 

Current interoperability solutions are lacking in efficacy. Cross-chain bridges, a basic interoperability provision, 
are prone to hacks, are often unable to guarantee transfers, and can be prohibitively expensive. 
For example, in less than two years the exploits of bridges have totaled over $1.5 billion. 

Composable solves this by creating a natively interoperable ecosystem with a set of innovative, novel DeFi primitive 
services that are connected between and integrated with major blockchain ecosystems. As such, Composable allows the free 
flow of liquidity between the Cosmos network of blockchains, Ethereum Layer 1s and 2s, and the parachains in the DotSama
ecosystem. It therefore solves the challenge of siloed and fragmented liquidity.

## Composable Finance’s vision

Composable’s vision is to enable a DeFi landscape that is blockchain and ecosystem agnostic. 
This means that users and developers are able to operate in the most effective and optimal manner, 
and are indifferent to the underlying chain.

Blockchain agnosticism understands interoperability along a spectrum that inevitably progresses, and is viewed as the 
furthest conclusion of that spectrum. In such an environment, DeFi assets are not constrained by the chain or layer they
sit on. Instead, there is easy transfer of assets cross-chain and cross-layer, as well as active communication between 
them. 

Composable’s vision for blockchain agnosticism is distinguished by its aim for applications to be able to function in a 
distributed manner across chains and layers. In this future, applications exist on multiple chains and interoperate 
in a seamless manner.

Ultimately, Composable Finance seeks to build a cross-chain future that overcomes challenges of siloed liquidity and 
provides better returns to users. It is working to accomplish this through the Picasso network, and the Composable 
Parachain.

## Tech stack overview


Composable’s technical stack is composed of novel reinventions of the virtual machine, routing layer, bridge, and 
application layer.

*Transfer availability layer*

Composable does not have a traditional bridge. Instead, it has the Mosaic transfer availability layer. Mosaic forecasts 
liquidity demands and dynamically re-balances liquidity across ecosystems in order to complete asset transfer. 
Where this process is too time consuming, just-in-time liquidity bots are able to bridge any gaps, ensuring guaranteed 
transfer of assets even in large transactions.

*Routing layer*

Composable’s Routing Layer, available for use through the Composable Parachain, is able to perfectly select the optimal 
route for the transfer of assets across a network of bridges, chains, and layers, to ensure their transfer to the 
destination layer or chain with lowest cost, and highest speed, backed by the security of Polkadot.

*Cross-Chain Virtual Machine (XCVM)*

Composable’s XCVM functions as a place for developers to create smart contracts that span multiple chains and ecosystems. 
Though different ecosystems have siloed smart contracts, and often require different languages to write smart contracts,
the XCVM enables developers to write smart contracts that are cross-chain and layer in a single to use, accessible
interface. In addition, the contract is written in Rust, without the need to learn multiple languages.

*Application Layer*

Composable’s application layer makes use of composable pallets through the Substrate framework. As such, they are 
easy to create, deploy, and combine together or ‘stack’ in order to create novel applications. In addition, 
Composable has deployed a host of DeFi primitives that are ready to be used and combined by developers. 
Finally, once deployed on Picasso, applications can then be graduated to the Composable Parachain.

