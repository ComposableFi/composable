# AVS Use Cases

This section provides a brief overview on the some of the initial AVS use cases for the Generalized Restaking layer:

## Securing an L1 network
A major use case that can be adopted by L1 blockchains will be the ability to power an existing or launch a new PoS chain leveraging the security of the Generalized Restaking layer. This flexibility allows chains to customize their security requirements and decouple the network launch from the token launch process. In this scenario, the AVS will be the L1 network.

## Restaked Rollups
Another application of the restaking layer is for Restaked Rollups. Initially, this will be utilized for MANTIS, with the potential for expansion to other rollups in the future. Rollups can benefit from shared security for their network and settle transactions on Picasso Cosmos.

## Middleware Applications
Middleware protocols such as Oracles, Bridges and Sidechains will not need to devote time and resources to their own security models, and can instead focus on their respective roles/services. Importantly, the security delivered to these protocols can be tailored to their needs, so that the cost of corrupting a middleware service equals the aggregate amount of all restaked assets via Picasso’s Restaking Layer. This means middleware services can no longer be the “weakest link” when it comes to security, making them less likely targets.
This is already in production on Solana to implement IBC on previously IBC-incompatible chains. This connection will happen via the first AVS for Solana IBC. The purpose of this AVS is to provide state proofs of Solana via existing validators of the host network running a lightweight sidecar to generate these proofs. The AVS for Solana IBC is described in detail in this Composable Research forum post. 

## Reduced Solver Collateral Requirements
Within the MANTIS workflow, solver collateral will be necessary. However, solvers will have the ability to integrate with the restaking layer, significantly reducing collateral requirements.

## Partial Block Building
Restaking could enable partial block building. If Block proposers restake LSTs, Block builders may be able to construct all or a portion of the block as they wish. Even with this flexibility, blocks will always have space, as the price of continually building full blocks exponentially increases. Block builders can assemble this portion of the block and compute a merkle root of the transactions therein. The transactions, merkle root, and bid are sent to the relay. Then, the relay enables data availability by storing the transactions and sending the merkle root and bid to the block proposer. 

From there, the proposer can select the highest bid and assembles an alternative block. They send an attestation to the merkle root of the winning bid that is linked to a commitment other than the header (i.e. the transaction root) to the proposer’s alternative block. Then, the relay sends the proposer the underlying transactions of the block builder-constructed portion of the winning bid’s merkle root. From here, the proposer assembles a new block with these transactions in the first portion, filling the remainder of the block with whatever transactions they desire. If the relay fails to release underlying transactions, the proposer then proposes their alternative block.


## DeFi Restaking 
The restaking layer has the capability to support various use cases in DeFi such as integrating with LP tokens.
Liquidity providers can restake their LP tokens that are already providing revenue within DeFi protocols, during swaps conducted through the associated protocol. For example, when a user performs a swap and LP tokens are linked to Curve, the fees collected can be distributed exclusively to these LPs. This functionality may also be feasible through Uniswap v4.
