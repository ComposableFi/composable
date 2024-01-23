# Protocol Architecture/Features

The MANTIS framework is architected with the following components:
- Cross-Domain Communication via the IBC 
- Multi-Domain Auctions 
- Language for Execution 
- Verifiable Settlement 

### Cross-Domain Communication via the IBC

MANTIS leverages Composable’s IBC bridge to facilitate cross-chain intent settlement. Our trust-minimized bridge in turn leverages the Inter-Blockchain Communication (IBC) Protocol. We have already connected Polkadot, Kusama, and Cosmos/the Interchain to this bridge, with expansion to Solana and Ethereum in the works.

### Multi-Domain Auctions
User intents are scored based on volume cleared, with solutions being screened for MEV and bundled into a block for each domain. Searchers can tip for priority, and finalized blocks are embedded with validity predicates and sent to builders. This is depicted below:

![mda](../mantis/mda.png)

### Language for Execution: The Composable Virtual Machine
When the best solution is found, it is turned into a Composable Virtual Machine (CVM) program, which:
- Specifies which hops need to happen
- Specifies which calls to virtual wallet need to occur
- If a solution has multiple hops - routed back to Centauri chain
- Ex. Transferring to a CEX
  - Problem defined as location to send funds to
  - User funds transferred to virtual wallet
  - CVM instruction set defines the necessary hops to the required network able to accept the assets
  - Transfers occur over IBC

This is depicted below:
![CVM](../mantis/lfe-cvm.png)

### Verifiable Settlement
Settlement of transactions resolving user intents must be verifiable. We also believe that these transactions must be partial block aware; To improve cross-domain censorship-resistance and enforce searcher conditioning for cross-domain transactions, partial block auctions are a must.

Examples of this can be seen in Cosmos, but Ethereum requires additional work regarding commitments to allow for a differentiation between top-of-block and rest-of-block.