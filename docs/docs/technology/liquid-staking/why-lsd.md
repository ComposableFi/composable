# Why Liquid Staking

## The Problems

### Siloed and Reduced Security

Security is siloed between most chains. While many large chains use PoS to secure the main chain, this security largely does not extend to middleware and apps built atop these chains using a separate consensus protocol or execution layer. To secure themselves, middleware must implement their own trust and security networks. This is risky and incredibly intensive. 

As a result, many middleware providers are suboptimally secure. This makes them more likely targets of attack as the “weakest links” compared to the main chain. Moreover, dApps powered by these middlewares inherit the lack of security of the middleware. dApps often use a number of middleware services like oracles and bridges. Therefore, dApps often may have their security being compromised by a number of external factors that are not immediately apparent to users.

### Censorship and Lost Agency

Censorship concerns are another issue on a number of blockchains. This is particularly true on Ethereum since its merge to proof-of-stake (PoS) and the creation of MEV-Boost by Flashbots. Ethereum validators use MEV-Boost to earn a share of the MEV extracted from the blocks they propose to block builders, made possible by proposer-builder separation (PBS) in Ethereum. Using MEV-Boost, validators access blocks from a marketplace of builders who produce blocks with information on the transaction orderflow and fee for the validator who proposes the block. 

A significant downside of MEV-Boost is that it only allows MEV extraction for full blocks, as proposers attest to block headers only when selecting the highest bids. Thus, proposers cannot customize block contents. This makes room for censorship; builders could collude to censor transactions for extortion. Moreover, the full-block nature of MEV-Bost means there is limited slashing capability; slashing can only occur when a block proposer proposes a different block than the committed block (e.g. block proposers sign two block headers of the same height).

## Composable’s Solution

### Liquid Staked DOT: LSDOT

Our solution leverages liquid staking tokens, which represent a nearly $20 billion market on Ethereum alone. In particular, we are creating a new means of liquid staking DOT, the native token of the Polkadot Network. Users will be able to liquid stake their DOT on Composable’s ecosystem, in return receiving the new liquid staked token, LSDOT.

Liquid staking is important in PoS networks as it allows users to stake tokens while still being able to use a form of that token (an LST) for other purposes (hence why these tokens are referred to as being liquid). These LSTs can then be used for liquidity provisioning, lending, borrowing, restaking, or any other purpose that other tokens can be used for in DeFi. As a result, protocols with liquid staking are able to incentivize users to stake their tokens with them, as users can compound the yield of staking with that of other uses of their LSTs.

Components of this solution are detailed below:

### Accounting Service

This initiative is underpinned by an accounting service coordinating protocol functions. This is a middleware service using the design of the present initiative to bootstrap its own security.

### Synchronization Mechanism & Queue System

The present protocol synchronizes and executes operations. This involves a queue system, made possible by the fact that restaking activities do not need high performance and are mostly not time-sensitive. As a result, we do not have any challenges relating to concurrency.

The queuing system performs batch operations on an epoch basis. Epochs will be a constant length (likely around 6 hours), and long enough to reduce on-chain transaction cost while being short enough to deliver a reasonable transaction deposit and withdrawal time. Within each epoch, deposits and withdrawals are queued. These transactions are processed after Composable triggers these operations. As all system-level operations are triggered via Composable, concurrency issues are eliminated.

### Risk Management

Restakers enabling pooled security generate a detailed risk management framework at the core of the present protocol. This protocol makes recommendations to middleware services (with an option to override). Restaking participants are provided with details of associated risks including systematic risk at the time of deposit. As this initiative leverages itself for restaking, genesis risk parameters are very conservative.

### Guest Blockchain

Restaking will happen on our guest blockchain and on Polkadot for other trust assumptions. The purpose of the guest blockchain is to enable a service that is compatible with the IBC Protocol to function on top of a chain that is not IBC compatible. This will initially be deployed on Solana, facilitating a light client there, and thus making Solana IBC compatible. The guest blockchain is described in detail in this Composable Research forum post. 

### Restaked Validator Layer for Partial Block Building

Our solution also leverages restaking, in which a previously staked token (like an LST) is staked again. This process allows for pooled security from the underlying chain to be applied to other chains, applications, or middlewares.

Composable is working to support LSDOT restaking in our restaking layer. We created the restaking layer to address 1) siloed and reduced security and 2) censorship and lost agency. As a result, we create a use case for LSDOT in the Cosmos, while delivering pooled security (via EigenLayer) to the Composable ecosystem.

Our restaking layer consists of a network of restaked validators who are off-chain entities. Validators on this network could be existing validators, who are able to opt into the restaking layer, and are also able to select which protocol(s) to restake with. While staking thus increases the potential amount slashed for validators performing malicious actions, it also greatly increases the amount of rewards that validators can earn. 

In this network, validation is powered by restaked tokens contributed by users (restake LSDOT, etc.). Validators are selected according to a Byzantine Fault Tolerance (BFT) mechanism. Selected validators check the smart contract on-chain and use these inputs to construct a block. The block is finalized when signed by ⅔ of validators. Restaked funds are sent to Composable and encoded on the block as part of the header data, which we create a proof for. The block is stored on our internal ledger in addition to being encoded on the respective chain.

The rationale behind creating a restaking layer is that it allows Composable to build partial blocks in every domain. This addresses the censorship and block proposer agency issues outlined previously. 

Enabling partial block building works as follows:

Block proposers can restake LSDOT. Block builders then construct all or a portion of the block as they wish. Even with this flexibility, blocks will always have space, as the price of continually building full blocks exponentially increases. Block builders assemble this portion of the block and compute a merkle root of the transactions therein. The transactions, merkle root, and bid are sent to the relay. Then, the relay enables data availability by storing the transactions and sending the merkle root and bid to the block proposer. From there, the proposer selects the highest bid and assembles an alternative block. They send an attestation to the merkle root of the winning bid that is linked to a commitment other than the header (i.e. the transaction root) to the proposer’s alternative block. Then, the relay sends the proposer the underlying transactions of the block builder-constructed portion of the winning bid’s merkle root. From here, the proposer assembles a new block with these transactions in the first portion, filling the remainder of the block with whatever transactions they desire. If the relay fails to release underlying transactions, the proposer then proposes their alternative block.

To penalize builders for malicious actions, we implement two requirements. First, builders must restake LSDOT with Composable to participate in this initiative. This addresses concerns about proposers stealing the transaction released from the relay to rob all of the builder’s MEV. This is mitigated because a cryptoeconomic cost is put on this theft; as proposers are restaked with the present initiative, they would be slashed if they propose blocks without transactions from the builder-constructed portion of the block (as proven by on-chain proofs comparing the block’s transaction root and the merkle root of the builder-constructed block portion) or if they did not propose the alternate block they attested to. Second, dispute raising on invalid transaction bundles will be done via fraud proofs in interactive challenges. As a result, if builders propose invalid bundles, proposers can raise a fraud proof and the builder will be penalized.

### Monitors

Monitors opt to run our provided monitoring software. This software syncs to the latest state of the middleware network, listens for network messages, monitors for malicious events, and reports such events with proofs to Composable to contribute to the slashing process.

Monitoring is part of the protocol for middleware. Monitors can be the same entities as those operating middleware clients, thus enabling our customers to monitor each other. Third-party operators can also be monitors, depending upon the design of the middleware involved. 

Composable will work with customers to customize the monitoring software to each middleware. This is needed as malicious behaviors differ between middlewares. 

## How it will work

### Deposits

1. The user obtains LSDOT from another protocol.
2. The user navigates to the deposit/withdrawal interface from Composable.
3. The user deposits LSDOT, with the deposit being queued for processing, and tokens being locked upon deposit.
4. Composable triggers a deposit accounting request to all supported chains (e.g. Cosmos chains). This signals to the chain that future deposits will be queued for the next epoch.
5. New deposit information is sent along the Composable Cosmos chain and IBC back to Composable. 
6. New restaked tokens are accounted for for each given middleware service, with these stakes being updated via consensus and logged on an immutable shared ledger.

### Withdrawals

1. When staking on the current initiative, the user must input a withdrawal address.
2. The user navigates to the deposit/withdrawal interface from Composable.
3. The user inputs their withdrawal request, which is received by Composable’s smart contract deployed on connected chains. The withdrawal is queued for processing. 
4. Composable triggers a withdrawal accounting request to all supported chains . This signals to the chain that future withdrawals will be queued for the next epoch.
5. New withdrawal information is sent along the Composable Cosmos chain and IBC back to Composable. 
6. Stakes are updated for each project using our product via consensus and logged on an immutable shared ledger.
7. Composable sends a message to each of the supported chains with withdrawal requests.
8. Pending withdrawals are unlocked after the appropriate chain receives the message.
9. Restakers are eligible to claim unlocked coins on their withdrawal address, as specified in step 1. If this withdrawal address is delegated to a smart contract in the present initiative, the slashed tokens will be deducted from the original principal deposit.

### Slashing 

1. Monitors detect malicious/anomalous behavior and begin a slashing request to the involved chain.
2. A cross-chain message is sent along the Composable Cosmos chain, with the stakes frozen for the affected customer and deposit and withdrawals disabled. Pending deposits and withdrawals in the present epoch become invalid.
3. Proofs are given to the slashing contract
4. Dependent upon the type of slashing required, a number of steps can then happen:
   - If a cryptographically provable offense occurs, it is easily proven and implemented into the smart contract.
   - If consensus-driven slashing is involved, more time/additional proofs are needed.
   - If a slashing veto is triggered, a lengthier sequence occurs to finalize slashing.
5. Taking into account the slashing outcome, Composable reaches consensus and the staked amount is updated. The resulting data is sent via the Composable Cosmos chain to all impacted chains.
6. User funds are slashed.

### Fee Distribution

1. The customer sends a reward proof to Composable to begin the fee distribution process.
2. Composable triggers fee distribution requests at set intervals of time. When consensus is reached, a fee distribution request is sent to appropriate blockchains.
3. A cross-chain message is sent over the Composable Cosmos chain to the involved chains, releasing the funds. This bulk operation further includes fees that must be sent to other chains. As a result, restakers on other chains who may have also restaked the the present customer only receive one payment contract per chain.
4. The Composable Cosmos chain sends funds to other chains for claiming by restakers.
5. Once funds are received by these other chains, they can be claimed by restakers.

## Use Cases

## Enabling Partial Block Building

Composable leverages [EigenLayer](https://www.eigenlayer.xyz/) along with the present solution to facilitate partial block building. EigenLayer is a protocol that enables staked ETH to be used as cryptoeconomic security for protocols other than Ethereum, in exchange for protocol fees and rewards.

EigenLayer has further introduced a solution to the limitations of MEV-Boost with [MEV-Boost++](https://hackmd.io/@layr/SkBRqvdC5). This protocol preserves block proposer agency while maintaining the benefits of MEV-Boost by enabling partial block building. MEV-Boost++  accomplishes this through the following steps:

1. Block proposers restake ETH with EigenLayer
2. Block builders assemble partial blocks
3. Only centralized relay provisioning data availability (DA) is permitted
4. Block proposers complete commitments
5. Data is revealed

This is depicted below:

![image-sourced](../liquid-staking/why-lsd-1.png)

Further building upon the MEV-Boost++ model from EigenLayer and incorporating it with our LSDOT and restaking layer solutions, we are thus able to implement partial block building in our ecosystem. 

### Securing other Cosmos Chain Middleware/Apps

We later plan to enable this solution for apps and middleware providers in the Interchain through our IBC bridge. As a result, middleware protocols such as oracles and sidechains will not need to devote time and resources to their own security models, and can instead focus on their respective roles/services. Importantly, the security delivered is significant, so that the cost of corrupting a middleware service equals the aggregate amount of all restaked assets via Composable. This means middleware services using our solution are no longer the “weakest link” when it comes to security, making them less likely targets.

### Connecting to Secure Middlewares on Other Chains

In a similar manner to the above, we will also be able to extend this service to be able to secure middleware and applications on chains outside of the Cosmos/Interchain. These additional chains must first be connected to the IBC via Composable IBC. For example, we are currently in the testnet phases of connecting Ethereum to our bridge. Once this is on mainnet, the present initiative can be expanded to Ethereum-based protocols.

Perhaps even more excitingly, we are working to connect previously IBC-incompatible chains to the IBC, and apps on these chains can also take advantage of our service. This connection will happen via our novel guest blockchain solution. The purpose of the guest blockchain is to enable a service that is compatible with the IBC Protocol to function on top of a chain that is not IBC compatible. This will initially be deployed on Solana, facilitating a light client there, and thus making Solana IBC-compatible. The guest blockchain is described in detail in this [Composable Research forum post](https://research.composable.finance/t/crossing-the-cross-blockchain-interoperability-chasm/33). 

**Thus, any chain connected to Composable IBC can benefit from the shared security of LSDOT restaking.**