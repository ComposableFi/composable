# Definitions

### Actively Validated Services (AVSes) 
A project or protocol that needs distributed validation and acquires said validation via restaking. Examples of AVSes are middleware services, layer 2 networks, bridges, data layers, and dApps.

### Coincidence of Wants (CoWs)
An occurrence where two (or more) parties coincidentally hold an item or asset that the other wants, and thus are able to exchange directly without the need for an intermediary exchange; in the case of intents, this principle means that user intents can coincidentally be the opposite of other user intents (i.e. one intent to swap A for B and another to swap B for A form a CoW).

### Composable Virtual Machine (CVM) 
An orchestration language and execution runtime for cross-chain program execution and intents settlement that operates, specifically over IBC.

### Cross-Domain Maximal Extractible Value (MEV) 
The maximum value that can be captured from arbitrage transactions executed in a specified order across multiple domains. See: Maximal Extractible Value.

### Cryptoeconomic Security 
A model for securing a network via economic incentives and cryptography.

### Generalized Restaking 
A mechanism for restaking an asset from a starting location on any chain, such that the cryptoeconomic security provided can be used by Actively Validated Services on any other chain. See: Actively Validated Services, Cryptoeconomic Security, and Restaking.

### Intent 
An expression of what a user wants to achieve whenever they interact with a blockchain protocol, for instance “transfer X asset from blockchain A to blockchain B” or “trade X asset for Y asset”. Practically, an intent is an off-chain signed message that encodes which state transitions a user wants to achieve. Unlike transactions, intents are partial. Thus, one can think of intents as parts of transactions that require other direct or indirect parts as complements in order to form a final balanced transaction that satisfies all of a user's constraints.

### Inter-Blockchain Communication (IBC) Protocol
A cross-chain messaging protocol for trust-minimized communication between different blockchains; website here.

### Light Clients 
Lightweight, trustless mechanisms for verifying the state of the counterparty blockchain; these are essential for IBC communication.

### Maximal Extractible Value (MEV)
The maximal value extractable between one or more blocks, given any arbitrary re-ordering, insertion or censorship of pending or existing transactions (as defined by Obadia et al., 2021).

### Multichain-Agnostic Normalized Trust-Minimized Intent Settlement (MANTIS) 
A vertically integrated, optimized intents settlement framework with expression, execution, and settlement. 

### Operators 
Entities responsible for executing off-chain software logic restaked from an AVS. See: Actively Validated Services.

### Restaking
A new primitive in crypto-economic security that enables the rehypothecation of a token on the consensus layer.

### Solvers 
Entities that compete to determine an optimal solution (in the form of a transaction execution pathway) for a user's intent. See: intents.

### Sync Committee
A committee of 512 validators that is randomly selected every sync committee period (~1 day). While a validator is part of the currently active sync committee, they are expected to continually sign the block header that is the new head of the chain at each slot.