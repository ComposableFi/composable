# Technical Architecture

Smart contracts utilized on Picasso Cosmos for the Restaking Layer are written in CosmWasm. Contracts in domain specific PoS Blockchains are written in the smart contracting framework of the respective execution layer.


![architecture](../restaking/architecture.png)
### Restaking Vaults 
A smart contract is deployed on each PoS chain in the system to accept restaked assets to power the restaking layer, along with staking vaults for the Cosmos ecosystem assets on Picasso Cosmos.

Vault Contracts are designed to receive Liquid Staked Tokens (LSTs). The assets that will be initially accepted are: 

1. Solana LSTs 
2. ETH and ETH LSTs
3. Monad LSTs and Native Withdrawal 
4. TRX LSTs 
5. lsDOT
6. BNB LSTs 
7. Cosmos assets via Picasso: 
   - stTIA/milkTIA 
   - stDYDX 
   - stATOM
   - SEI LSTs 
   - PICA LSTs 
   - Berachain LSTs 

### Operators 
An actor responsible for executing off-chain software logic restaked from the AVS. These operators function between Restaking Layer and the AVS. Operators retrieve their instructions from the AVS Registration Contract, which informs them of the validation tasks.

Upon registration, operators are stored in the Operator Assignment. Within this contract, operators also register for the AVSes they intend to validate. Subsequently, they must obtain delegations to interact with these protocols.

Operators receive delegations from the Delegation Management contract in the following manner: 

1. Users delegate their assets to particular operators to the Vault Contract via the Delegation Management contract.
2. The total value of the user’s stake is derived from the Accounting Contract. 
3. Through this delegation, users can select which Actively Validated Services (AVS) they wish to validate.
4. Operators have the option to accept or decline these delegations. 
5. The Operator Assignment Contract specifies conditions that delegators must adhere to, including the percentage fee charged by operators.
6. The Delegation Management Contract facilitates asset unstake calls for users. 

:::note
When selecting an AVS, Operators and Stakers should be aware of the associated slashing conditions that are set by the AVS.
:::

### Actively Validated Services (AVSes) 
AVSes are decentralized applications that require economic security such as roll-ups, L2s, data availability layers, sequencers, dApps, cross-chain bridges, and virtual machines. AVSes define what logic they would like validated by interacting with the AVS Registration contract. In this contract, they will parameterize: 

- Amount of stake i.e. how much security they require.
- The type of asset or assets accepted.
- The set of operators they’d like to interact with.
- Slashing parameters. 
  - Slashing parameters and proofs of successful behaviour must adhere to a specific framework.

Once the parameters are established, AVSes proceed to engage with the Rewards Distribution contract to allocate rewards corresponding to the desired distribution period. This process defines the rewards rate per unit of security, wherein, for instance, an AVS may pay $50 of their native token for 1 unit of security (e.g., 1 sol of security). Additionally, a length of time must be specified for each epoch by the AVS. A fraction of the rewards (20%) is automatically allocated to PICA stakers.

### Orchestrator 

This Orchestator is a smart contract deployed on Picasso designed to execute fundamental operations including:

- Interfacing with the AVS Registration contract which facilitates registration and unregistration processes.
- Updating the Accounting Management contract to reflect staked and unstaked amounts.
- Updating the Accounting Management contract to record the amount slashed.
  - Sending a notification to the vault to transfer slashed assets to the community pool.
- Updating the Delegation Management contract with user delegations.
- Facilitating interactions between the Delegation Management contract and the Operator Assignment contract.
- Slashing Manager:
  - Adding contracts authorized to perform slashing.
  - Revoking slashing permissions from specified contracts.
  - Monitoring historical stake updates to ensure that withdrawals are only permitted once no middlewares possess slashing rights over the withdrawn funds.

### Verifier 
The Verifier contract is responsible for verifying that operators execute the consensus of the Actively Validated Services (AVS) correctly. It dispatches slashing operations to the orchestrator and receives IBC proofs of slashing conditions from the AVS. Additionally, it forwards the slashing operations to the orchestrator for the AVS, ensuring thorough validation of the AVS's performance.

### Accounting contract 
This contract is responsible for updating the state of vaults deployed on various chains based on actions such as {stake, un-stake, slash}.

### Fishermen Protocol
These are actors who ensure operators are being honest and signal if an operator is misbehaving and needs to be slashed. Anyone is allowed to become a Fisherman and rewards are provided to any misbehaviour reports via Slashing. 

### CVM
Send stake and un-stake messages from PoS chains to Picasso Cosmos. Users can originate these requests from the PoS chains they restake assets. Additionally, users can (un)delegate their stake to operators of AVSes. These are all operations that are executed on the chain where the user assets live, and are propagated using CVM.

### Slashing Contract
The process involves detecting malicious behavior, initiating slashing requests, and executing specific steps based on the type of slashing chosen by the AVS.

1. Monitors detect malicious/anomalous behavior and begin a slashing request to the involved chain.
2. A cross-chain message is sent along Picasso Cosmos, with the stakes frozen for the affected customer and deposit and withdrawals disabled. Pending deposits and withdrawals in the present epoch become invalid.
3. Proofs are given to the slashing contract
4. Dependent upon the type of slashing required, a number of steps can then happen:
   - If a cryptographically provable offense occurs, it is easily proven and implemented into the smart contract.
   - If consensus-driven slashing is involved, more time/additional proofs are needed.
   - If a slashing veto is triggered, a lengthier sequence occurs to finalize slashing.
5. Taking into account the slashing outcome, Picasso Cosmos reaches consensus and the staked amount is updated. The resulting data is sent via Picasso Cosmos to all impacted chains.
6. User funds are slashed.

### Fee Distribution
The fee distribution process is carried out in the following manner:

1. The AVS sends a reward proof to the Accounting contract to begin the fee distribution process.
2. Picasso Cosmos triggers fee distribution process that are set by the AVS.
3. When consensus is reached, a fee distribution request is sent to appropriate blockchains.
4. A cross-chain message is sent over Picasso Cosmos to the restaking vault located on the network where the user has deposited their stake. 
5. This bulk operation further includes fees that must be sent to PICA stakers. 
6. Restakers on other networks who may have also restaked on the same AVS can claim their rewards.
