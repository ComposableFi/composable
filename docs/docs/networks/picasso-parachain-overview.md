# The Picasso Parachain 

## What is Picasso?

[Picasso] is the infrastructure layer pioneering interoperable DeFi solutions, with its native token [$PICA]. 
It is built on the Kusama network, and as such exists within the wider DotSama ecosystem. 
Picasso is highly secure, efficient, and natively interoperable due to its use of a parachain, 
and its proprietary technology stack.

[Picasso]: http://picasso.xyz
[$PICA]: ./picasso/tokenomics.md

![picasso_diagram](./picasso/picasso-diagram.png)
**Picasso houses a suite of modular and interoperable Substrate pallets:**

- [Pablo] - DEX: serving as a cross-chain liquidity hub on Picasso
- [Composable-IBC] - trustless bridging between major DeFi ecosystems
- [Apollo] - Picasso’s native oracle pallet

[Pablo]: ../technology/pablo-overview.md
[Composable-IBC]: ../technology/composable-ibc.md
[Apollo]: ../technology/apollo-overview.md

## How is Picasso enabling interoperable DeFi

Picasso benefits from the interoperability inherent to all parachains connected to Kusama or Polkadot. 
Parachains connected to the same Relay Chain may utilize the Cross-Consensus Message Format 
(XCM) as a framework to communicate with each other. 
The actual passing of messages between parachains is done through the use of Cross-Chain Message Passing 
(XCMP) which allows for functionality such as asset transferals or cross-chain smart contract calls.

### CosmWasm & IBC

Picasso exists as the only [CosmWasm](../technology/cosmwasm-vm-overview.md) and IBC-enabled parachain. 
Thus, Picasso will be able to seamlessly integrate with other parachains 
as well as IBC-enabled blockchains in the Cosmos ecosystem. 
As the first instance of an IBC implementation outside of the Cosmos ecosystem, 
novel strategies can now be built that leverage the best of Substrate and Cosmos SDK blockchains. 
This is made possible through some key innovations by our bridging team.

As the first instance of CosmWasm outside of Cosmos, existing projects can deploy a satelite protocol on Picasso and gain access to a completely new ecosystem of users and builders.

### Insights into Picasso's Pallets

The Substrate blockchain development framework enables parachain teams to rapidly establish autonomous layer 1 blockchains using core building blocks referred to as pallets. These pallets, akin to Cosmos SDK modules, can be combined in various ways to create a customized runtime environment. Substrate offers developers the option to either reuse existing pallets or develop new ones to introduce additional functionalities as needed.

Picasso offers an extensive range of Substrate pallets that synergize to form a highly robust interoperable DeFi platform. By reducing transaction costs, creating modular applications for adaptable liquidity movement, and implementing pioneering solutions, Picasso effectively addresses key challenges within the realm of interoperable DeFi. These challenges encompass liquidity fragmentation, asset transfer security, and the absence of standardized cross-chain communication protocols. Within the realm of Picasso, DeFi essentials such as [Composable-IBC] and [Pablo] thrive.

### Composable’s VM on Picasso

Composable’s Virtual Machine [CVM](../technology/cvm.md) powered by CosmWasm will be able to leverage the pallets above to facilitate the creation of non-custodial, natively cross-chain smart contracts. The CVM serves as a top-layer orchestration layer, capable of calling into existing applications and pallets across multiple ecosystems asynchronously. Applications on any IBC-enabled chain can leverage the CVM to interact via cross-chain contracts in order to simplify cross-chain user experiences.

### Insights into Picasso’s Security

As a parachain connected to the Kusama Network, Picasso benefits from Kusama’s “shared security model”. 
The shared security model is unique to the Kusama and Polkadot ecosystems. 
Parachains (sovereign, highly customizable, layer 1 blockchains) 
are required to lease a connection to one of the 2 relay chains.
Over the course of this lease period, parachain teams, 
such as ours, benefit from the shared security of the Relay Chain.
Meaning that all parachains who secure a slot to either Kusama or Polkadot benefit from the economic security 
provided by the Relay Chain’s validators. 
Therefore, as a parachain connected to the Kusama Relay Chain, 
Picasso benefits from Kusama’s extensive validator set for shared security.

## What is Picasso’s vision?

We envision Picasso as an infrastructure layer that pioneers interoperable DeFi solutions. 
It is equipped with a robust set of novel DeFi primitives that are built as modular pallets. 
These pallets are designed 
to attract liquidity from the broader DeFi landscape whilst promoting interoperability between various ecosystems. 
Thus, Picasso, through its native interoperability, 
aims to become an ecosystem flush with liquidity from multiple different blockchain networks.
Developers interested in Picasso’s interoperability framework will find the core pallets and SDKs beneficial 
as they stimulate the development process for building on Picasso. 
Picasso’s modular base-level infrastructure allows developers to easily integrate, stack, 
or leverage core pallets such as the Oracle [(Apollo)], 
DEX [(Pablo)] and Trustless Bridging [(Composable-IBC)] pallets. 
These pallets and their functionalities are the foundation for Picasso that will lead us to enable interoperable DeFi.

[(Apollo)]: ../technology/apollo-overview.md
[(Pablo)]: ../technology/pablo-overview.md
[(Composable-IBC)]: ../technology/composable-ibc.md