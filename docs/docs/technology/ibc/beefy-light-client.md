# 11-BEEFY IBC Light Client

:::note

**The IBC implementation on Picasso and Composable utilizes the grandpa light client, as beefy has not yet been deployed on Kusama. The content of this section pertains to a beefy light client which is not used in production.**

:::

[Parity](https://www.parity.io/)’s `Bridge Efficiency Enabling Finality Yielder (BEEFY)` and its novel consensus gadget that enables DotSama to be bridged to additional chains via very efficient finality proofs. Parachains get their finality from the Kusama relay chain, and thus BEEFY’s ability to create finality proofs provides finality for IBC on Picasso and an essential gateway for the bridge infrastructure.

We are also developing a BEEFY light client implementation for Cosmos-IBC (11-BEEFY, spec pending. This product will enable Cosmos chains to follow the finality of the Kusama relay chain (and thus, the finality of Picasso). A single instance of this light client on any Cosmos chain can prove finality for any Kusama parachain, allowing Cosmos chains to verify IBC commitment packets (IBC consensus proofs). 

## BEEFY Finality Gadget 

With the [BEEFY protocol](https://www.youtube.com/watch?v=ZmIa_4hPRZ8&t=2378s), the authority set produces an extra finality proof for light clients which consists of the MMR root hash of all blocks finalized by [GRANDPA](https://polkadot.network/tag/grandpa/) (the finality gadget implemented for the Polkadot relay chain) at a given height. With the introduction of this protocol, light clients no longer need to be aware of all the headers in a chain for them to be convinced about finality. This drastically reduces the size of the data that light clients must store to follow the chain’s consensus to exactly 124 bytes.

A preliminary [specification](https://github.com/paritytech/grandpa-bridge-gadget/blob/td-docs/docs/beefy.md) for BEEFY is already available and is largely implemented, barring a few kinks that need ironing out. At a high level, this is a new protocol that will be added to Polkadot without the need for a hard fork. Thanks to the [WebAssembly (Wasm)](https://webassembly.org/) runtime and the on-chain governance protocol, this new protocol will produce significantly lighter finality proofs for light clients for both on-chain and off-chain uses. It will achieve this by having the existing GRANDPA authority set periodically vote on the Merkle Mountain Range root hash of all blocks that have been considered final by the network.

This proof is shown below:

```markdown
pub struct BEEFYNextAuthoritySet {
	/// Id of the next set.
	///
	/// Id is required to correlate BEEFY signed commitments with the validator set.
	/// Light Client can easily verify that the commitment witness it is getting is
	/// produced by the latest validator set.
	pub id: u64, // 8bytes
	/// Number of validators in the set.
	///
	/// Some BEEFY Light Clients may use an interactive protocol to verify only subset
	/// of signatures. We put set length here, so that these clients can verify the minimal
	/// number of required signatures.
	pub len: u32, // 4bytes
	/// Merkle Root Hash build from BEEFY AuthorityIds.
	///
	/// This is used by Light Clients to confirm that the commitments are signed by the correct
	/// validator set. Light Clients using interactive protocol, might verify only subset of
	/// signatures, hence don't require the full list here (will receive inclusion proofs).
	pub root: H256, // 32 bytes
	}
```

```markdown
/// Data that light clients need to follow relay chain consensus
pub struct BEEFYLightClient{
	pub latest_BEEFY_height: u32, // 4bytes
	pub mmr_root_hash: H256, // 32bytes
	pub current_authorities: BEEFYNextAuthoritySet<H256>, // 44bytes
	pub next_authorities: BEEFYNextAuthoritySet<H256>, // 44bytes
	}	
```


## 11-BEEFY COSMOS-IBC Light Client

Connecting to IBC requires both chains to embed a light client for proof of validation. In order to connect to IBC using Cosmos and Picasso, Composable is working to develop a Bridge Efficiency Enabling Finality Yielder (BEEFY) light client onto Picasso and Cosmos. 

To support Substrate-based chains on the Cosmos side, Composable will need a BEEFY-IBC client merged into IBC-Go; therefore, the first step in the process is to create a BEEFY-Go followed by a BEEFY-IBC. Once this is set, Composable will work on updating the relayer before launching the product.

Composable has completed the development of this [BEEFY light client](https://github.com/ComposableFi/ibc-go/blob/main/modules/light-clients/11-beefy/README.md) in Go for the Cosmos ecosystem, with the product being called the 11-BEEFY COSMOS-IBC light client. Pending further audits, this light client will be merged upstream into the IBC-Go repo which hosts the canonical implementation of the [Tendermint](https://tendermint.com/) light client.

Composable’s intent is that this light client will serve as the canonical light client for Cosmos chains to communicate directly with DotSama parachains. A single instance of the light client can track either the Kusama or Polkadot relay chain’s finality and be used to prove the finality of any of the connected parachains’ states. In the spirit of trustlessness, Composable has published a demo with [instructions](https://github.com/ComposableFi/ibc-go/blob/main/modules/light-clients/11-beefy/README.md) for everyone to run a test to verify the operation of the light client. The draft spec is available [here.](https://github.com/ComposableFi/ibc-go/blob/main/modules/light-clients/11-beefy/spec.md)
