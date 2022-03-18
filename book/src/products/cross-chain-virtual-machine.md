# Cross-Chain Virtual Machine
*The Composable XCVM allows for cross-ecosystem communication.*

--- 

The Composable Cross Chain Virtual Machine (XCVM) is a single, developer friendly interface to interact orchestrate smart contract functions across the multitude of L1 and L2 networks available. In short, the XCVM serves to abstract complexity from the process of having to send instructions to the routing layer directly, initiate call-backs into smart contracts, and more.

Utilizing the [Innovation Availability Layer](./cross-chain-virtual-machine/innovation-availability-layer.html) (IAL) infrastructure, we are creating a set of tools for the Composable Cross-Chain Virtual Machine that developers can use to tap into various functions of communication and liquidity availability. The result is multifaceted; users can perform cross-chain actions, and the overarching blockchain ecosystem is repositioned as a network of agnostic liquidity and available yield.

![XCVM unites ecosystems](./xcvm-unites-ecosystems.jpg)
*The XCVM unites a number of ecosystems across DeFi.*

In order to facilitate this communication, we need two specific components that we are currently building: the Communication and Finality layers of our XCVM.

- **Innovation Availability Layer (IAL)**: This includes the following features:
    - Polkadot-IBC cross-chain communication and asset transfers
    - L2-L2 communication and transfer through our parachain
- **Finality Layer**: This will be our parachain offering — called [Picasso](./the-picasso-parachain.html) on Kusama, and [Composable](./the-composable-parachain.html) on Polkadot.