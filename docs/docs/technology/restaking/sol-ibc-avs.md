# Technical Innovation: the AVS for Solana IBC

The AVS for Solana IBC is a system developed by Composable and collaborators on the research team at INESC-ID Distributed Systems Group, associated with the University of Lisbon. We are particularly grateful for the contributions of Professor Miguel Matos to this collaboration.

The AVS for Solana IBC concept acts as a foundational bridge, connecting previously isolated blockchains through the broader IBC. The benefits extend to increased liquidity, cross-chain DeFi opportunities, and a streamlined user experience. As we adapt and deploy this solution on additional blockchains, the vision of trust-minimized cross-chain interoperability becomes a reality, pushing blockchain towards mass adoption.

:::info
Some blockchains (for example NEAR, Solana and TRON) do not meet IBC’s technical criteria (e.g. see [ICS-23 specification](https://github.com/cosmos/ibc/tree/main/spec/core/ics-023-vector-commitments)), preventing the implementation of IBC on their networks. In response, we created a novel AVS for Solana IBC solution which runs on top of otherwise unsupported ledgers, providing all features necessary for IBC integration. This will be deployed on Solana to retrieve provable storage and supply Solana state proofs to enable IBC. This allows for trust-minimised bridging to chains that otherwise cannot support the IBC.
:::

At the time of writing, [over 392 million SOL are staked](https://solanacompass.com/statistics/staking), representing a staking market capitalization of over $25 billion dollars. This is a staggering 92% of the total circulating supply of SOL. Therefore, there is an incredibly large market for restaking these assets that are already staked in Solana. Yet, there has been very little use for these receipt tokens - until now. The Restaking Vault on Solana serves as the validation layer of the AVS for Solana IBC. This network needs to be validated like any other chain using the proof-of-stake (PoS) model.

## Architecture
Our innovative approach involves creating a AVS for Solana IBC layered atop the host blockchain (Solana). This AVS for Solana IBC extends capabilities, providing state proofs for blockchains lacking IBC compatibility. This solution is not only generic, applicable to any smart-contract-supporting blockchain, but also imposes no changes on the underlying host—a significant advantage. 


This novel AVS for Solana IBC design enables IBC Protocol communication between Solana and other IBC-supported ledgers (such as ones based on the Cosmos SDK). To operate, the system requires participation of validators who take part in block generation. The overall flow of the AVS for Solana IBC is depicted below:

![guest_blockchain](../restaking/gb.png)

*This figure depicts the sequence of events when successfully sending a message from a blockchain using the AVS for Solana IBC solution and an IBC-enabled counterparty blockchain. Like in any trustless example, anyone can run a relayer to pass messages between the host blockchain and the counterparty blockchain. However, to be able to provide a proof to the counterparty blockchain, a AVS for Solana IBC with provable storage is necessary.*

### Storage Challenges and Solutions
Provable key-value storage in the AVS for Solana IBC is crucial for providing proofs to the counterparty blockchain. To manage data generation and storage costs, our solution implements sealing or pruning of subtries, ensuring efficient use of on-chain storage. The commitment of storage is represented as a hash of the Merkle trie’s root node.

### Epoch Change: Ensuring Consistency and Syncing
Blocks are organized into epochs with specified validators. Epoch changes occur when a majority of stake signers finalize a block. Syncing to the latest state is facilitated by requesting blocks belonging to unknown epochs, ensuring efficient blockchain synchronization without fetching all blocks.

### Validation and Validator Set Dynamics
Validators produce blocks with state proofs, relayed to the counterparty chain. Validator selection, set changes, and slashing mechanisms are seamlessly integrated. Validators stake assets with the AVS contract, ensuring commitment to block validation for at least one epoch.

The validator network will be directed by majority where it is the responsibility of active validators to maintain uptime and sign corresponding payloads of transactions.

#### Bonding
Joining as a validator will require a bonded stake to keep participation gated from malicious actors easily onboarding. The size of the bond will be 25 SOL.

#### (Re) Staking
The validator set will be able to utilize liquid staking derivatives (LSD) of SOL, as well as LP tokens for this initiative. The following assets will be deemed acceptable collateral for staking as a validator: SOL, mSOL, jitoSO and bSOL.
The Pyth oracle will be used to access pricing feeds for assets staked to the platform.

#### Slashing & Jailing: Safeguarding Integrity
Slashing is implemented as a means of disincentivizing malicious or erroneous behavior amongst the network of validators. Namely, slashing occurs under conditions where a transaction is improperly signed, or a validator fails to maintain uptime of their node. The AVS for Solana IBC system ensures validators wait for host blocks to finalize before signing virtual blocks, maintaining the integrity of the process.

Slashing parameters are as follows:

| Condition | Penalty |
| --------- | -------- |
| Validator in active set has 50% downtime over the the previous 100 blocks  | 1% of stake slashed + Jailed      |
| Validator leaves set and does not continue validating during the unbonding period    | 1% of stake slashed         |
| Signing incorrect validator set or transaction/batch      | 95% of stake slashed     |

A validator becomes jailed as a penalty for excessive downtime, or double signing transactions.
Jailing parameters are as follows:

| Condition | Penalty |
| --------- | -------- |
| Excessive Downtime | Slashed and jailed. May rejoin the validator set after replenishing stake      |
| Double Sign   | Slashed and jailed. May not rejoin validator set        |

### Fishermen
The role of fishermen in the network is to monitor the validator set for misbehavior. Fishermen may submit an attestation in the event that a validator does not perform its responsibilities or acts maliciously. 

Fishermen must put forth a stake in order to submit an attestation. The stake must be large enough to promote acting with a relative amount of certainty before challenging validator behavior. For simplicity sake, **a Fisherman must stake 50 SOL to submit an attestation.**

The window to submit an attestation will last 1 epoch ( approximately 2-3 days ). A fisherman is rewarded or penalized based on the correctness of their attestation as follows:

- If an attestation is deemed valid, the fisherman receives 33% of the slashed amount with the remaining 66% being distributed to the active validator set
- If an attestation is deemed erroneous, the fisherman forfeits his stake which is then distributed proportionally across the validator set
- If an attestation was correct, but a validator has already been penalized (slashed) for the same attestation submitted via multiple fisherman, the fisherman who had submitted the stale attestation will not lose their stake, but will also receive 0 reward.

#### Validator Rewards
Validators signing transactions over the bridge will be rewarded for their participation in the form of a revenue split. These rewards will total 20% of revenue earned via bridging fees and will distribute proportionally across the active set. Validator rewards will be weighted based on the speed in which they sign a transaction.

### Future Exploration and Optimization
While the AVS for Solana IBC concept represents a significant leap in cross-blockchain interoperability, further optimization is needed. Challenges, such as the departure of the last validator and security measures beyond the IBC trust model, warrant future exploration.
