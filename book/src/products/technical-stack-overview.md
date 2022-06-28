# Technical Stack Overview


---


## Simplicity at Our Core: Our Guiding Principle

**Composable Finance is dedicated to eliminating the complexity that current DeFi users experience, abstracting away difficulties and unnecessary costs and delays. Our technical infrastructure takes on the complications of DeFi so that users can navigate the space easily and efficiently, in whatever manner suits their needs.**

Unnecessary complexity in many cases causes end-user friction and barricades developers from either expanding innovation or accessing it. To this end, we believe that sophisticated infrastructures must be rooted in simple and easy to use interfaces to propel efficiency and enable builders to ship confidently and faster. That is why at Composable Finance ‘simplicity’ is engrained in the DNA of our technologies and development ethos. Our purpose is to ensure pioneers get the most out of our whole host of products in the Composable ecosystem, and thus create exceptional experiences for the end user.

Upon the launch of our technology, our team and community of developers are committed to furthering the Composable tech stack through continual upgrades to user experience, exploring and attacking new opportunities to strive to push DeFi towards ultimate ubiquity.


## Our Current Technical Stack

DeFi developers are able to leverage the existing suite of foundational pallets, such as Mosaic, Cubic, Apollo, Centauri, and more, to rapidly design, build, and launch interoperable products. Each foundational pallet provides unique and necessary DeFi functionality which can now be stacked and incorporated to make functions modular. Our infrastructure enabled ease of pallet composition and the immense flexibility of our tech stack provides each new innovation with **modular composability**.


![technical_stack_overview](./composable-technical-stack.png)
*The core components of the Composable Technical Stack. Similar to how Port Control Protocol became an essential networking fabric for the Internet, Composable’s vision is to become the entryway and fabric for interactions, transfers, and communication cross-ecosystem. The result is a Port Control Protocol-like system for blockchains.* 


### The Composable Application Layer


Composable is the first to implement the CosmWasm smart-contracting framework within the Substrate and DotSama ecosystems. The application layer consists of applications that are deployed on the Composable XCVM. Composable’s cross-chain virtual machine (XCVM) is our novel solution to facilitate the orchestration of cross-chain applications, by using the CosmWasm framework for smart contracts.

### The Composable Cross-Chain Virtual Machine


The Composable Cross-Chain Virtual Machine (XCVM) is a top-level orchestration layer for deploying protocols that can communicate across multiple chains at once. It handles the complexity of routing to different networks so that developers can focus on while architecting cross-chain native smart contracts and protocols..


### The Composable Routing Layer


The [Routing Layer](https://dali.devnets.composablefinance.ninja/products/routing-layer.html) is our pathway execution layer that assesses all of the possibilities for a given action (e.g taking out a loan of 1,000 USDC) across all potential layers and chains and selects the optimal pathway for a user. This layer will be crypto-economically secured, with incentives provided for actors to properly select the best routes for user actions. Thus, this layer will act as a function aggregator, providing optimal services to users without them having to scour the entire expanse of the DeFi space themselves for the most promising opportunities.


### The Modular Transfer Availability Layer

[Mosaic](https://dali.devnets.composablefinance.ninja/products/mosaic.html) is our transfer availability layer and a primary pallet on Picasso that combines a [dynamic fee model](https://medium.com/composable-finance/the-dynamic-fee-model-powering-mosaics-transfer-availability-layer-f91011309592), [liquidity forecasting](https://medium.com/composable-finance/liquidity-forecasting-in-mosaic-part-iv-machine-learning-based-methods-17e8f2e5de14), [passive liquidity rebalancing](https://medium.com/@composablefi/mosaics-passive-liquidity-rebalancing-module-cb708605cf51), and [active liquidity management](https://composablefi.medium.com/understanding-mosaics-active-management-e1894fc90a00) to allow for seamless cross-chain transfers and liquidity routing to bridging infrastructures. As a proactive bridging infrastructure, Mosaic leverages a network of existing bridges to manage its LP vaults on different layers ensuring the ability to transfer assets cross-chain regardless of volume or size.


### Centauri

To connect the Cosmos ecosystem to Kusama, Composable is developing [Centauri](https://medium.com/composable-finance/centauri-facilitating-communication-between-interoperable-networks-5c1a997f9154) — a [trustless bridging](https://medium.com/composable-finance/trustless-bridging-438a6e5c917a) infrastructure that can communicate with the IBC (Inter-Blockchain Communication) protocol and enabling the interecosystem transfer of assets between Cosmos and Substrate-native protocols. Centauri creates the underpinning link between existing and soon to be deployed pallets on Dostama and Cosmos, a first of its kind Kusama — Cosmos bridge that will further help boost utility for assets within both ecosystems. In addition, Composable is actively working to bridge in a trustless manner to ecosystems beyond Cosmos utilizing the IBC.

Composable's integration of the IBC and Cosm Wasm smart contracts into the XCVM will allow for many new integrations between ecosystems beyond Cosmos, helping to expand the capacity of Cosmos.


### Apollo

Apollo is a MEV-resistant decentralized oracle that is leverageable for DeFi protocols to gain accurate information and price feeds in a decentralized manner. Apollo sets the standard for oracles first in DotSama and other ecosystems using different blockchain hooks to medianize and update data. Apollo will be flexible enough such that protocols can employ it based on their differing levels of security. We have orchestrated the design of Apollo into a leverageable oracle stack and we intend to work with partners who wish to utilize this technology.


### Cubic

Cubic is a primary pallet that sets the standard as the first vault pallet in DotSama. Vaults are vital infrastructures leverageable universally in DeFi. Their utility is tied to storing and moving tokens for other primitives or dApps such as lending protocols and automated market makers (AMMs) that require vault technologies to store and move collaterals without incurring expensive transaction costs. Cubic ties the Kusama ecosystem, Mosaic (encompassing all the major EVM chains and L2s), and Centauri (Composable’s Substrate-IBC bridge connecting DotSama to Cosmos) together for the unification of yield generated between the DotSama, EVM-compatible and IBC-enabled ecosystems in vault strategies and beyond.