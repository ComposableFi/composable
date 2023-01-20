# The Picasso Parachain 

## What is Picasso?

[Picasso] is the infrastructure layer pioneering interoperable DeFi solutions, with its native token [$PICA]. 
It is built on the Kusama network, and as such exists within the wider DotSama ecosystem. 
Picasso is highly secure, efficient, and natively interoperable due to its use of a parachain, 
and its proprietary technology stack.

[Picasso]: http://picasso.xyz
[$PICA]: ./picasso/tokenomics.md

Picasso houses a suite of modular and interoperable Substrate pallets:

- [Pablo] - DEX: serving as a cross-chain liquidity hub on Picasso
- [xPICA] - fNFTs: tradable and yield-bearing representation of a user's staked $PICA
- [Centauri] - trustless bridging between major DeFi ecosystems
- [Apollo] - Picasso’s native oracle pallet
- [Cubic] - Picasso’s native vault pallet

[Pablo]: ../products/pablo-overview.md
[xPICA]: ../products/xpica-fnft-overview.md
[Centauri]: ../products/centauri-overview.md
[Apollo]: ../products/apollo-overview.md
[Cubic]: ../products/cubic-overview.md

## How is Picasso enabling interoperable DeFi

Picasso benefits from the interoperability inherent to all parachains connected to Kusama or Polkadot. 
Parachains connected to the same Relay Chain may utilize the Cross-Consensus Message Format 
(XCM) as a framework to communicate with each other. 
The actual passing of messages between parachains is done through the use of Cross-Chain Message Passing 
(XCMP) which allows for functionality such as asset transferals or cross-chain smart contract calls.

### CosmWasm & IBC

Picasso exists as the only CosmWasm and IBC-enabled parachain. 
Thus, Picasso will be able to seamlessly integrate with other parachains 
as well as IBC-enabled blockchains in the Cosmos ecosystem. 
As the first instance of an IBC implementation outside of the Cosmos ecosystem, 
novel strategies can now be built that leverage the best of Substrate and Cosmos SDK blockchains. 
This is made possible through some key innovations by our bridging team.

### Insights into Picasso's Pallets

The Substrate blockchain development framework allows for parachain teams, 
such as ours, to quickly bootstrap a sovereign layer 1 blockchain through the utilization of core building blocks 
provided in the form of pallets. 
A pallet could be compared to a Lego brick 
that can be stacked in various arrangements to create a highly customizable runtime environment. 
With Substrate, developers can choose to reuse fundamental and proven pallets where possible 
or choose to create their own pallets to add new functionality when necessary.

Picasso delivers an extensive offering of Substrate pallets 
that come together to form DeFi’s most robust interoperable platform.
By reducing transaction costs, constructing modular applications that enable flexible liquidity movement,
and implementing innovative solutions,
Picasso is tackling some of the key difficulties of interoperable DeFi such as liquidity fragmentation,
asset transfer security, and the lack of generalized cross-chain communication standards.
Picasso, which houses DeFi primitives like [Centauri] and the [Pablo DEX], is designed to bring and maintain deep 
liquidity, leveraging new technologies and aligning itself closely with user requirements.

[Pablo Dex]: ../products/pablo-overview.md

### Composable’s XCVM on Picasso

Composable’s Cross-chain Virtual Machine [(XCVM)] 
will be able to leverage the pallets above to facilitate the creation of non-custodial, 
natively cross-chain smart contracts.
The XCVM serves as a top-layer orchestration layer, 
capable of calling into existing applications and pallets across multiple ecosystems asynchronously. 
XCVM applications will be deployable from any ecosystem housing the necessary satellite contracts and interpreter instances. 

[(XCVM)]: https://docs.composable.finance/products/xcvm

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
Picasso benefits from Kusama’s extensive validator set and is guaranteed 
that any other parachain on Kusama shares the same degree of security.

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
or leverage core pallets such as the Oracle [(Apollo)], Vaults [(Cubic)], 
DEX [(Pablo)] and Trustless Bridging [(Centauri)] pallets. 
These pallets and their functionalities are the foundation for Picasso that will lead us to enable interoperable DeFi.

[(Apollo)]: ../products/apollo-overview.md
[(Cubic)]: ../products/cubic-overview.md
[(Pablo)]: ../products/pablo-overview.md
[(Centauri)]: ../products/centauri-overview.md