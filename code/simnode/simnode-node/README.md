# Simnode Binary
This is a special binary that can be run without being connected to a relay chain and exposes an rpc method `engine_createBlock` that you can call in order to seal a block. Its method signature is:

```rust

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreatedBlock<Hash> {
    /// hash of the created block.
    pub hash: Hash,
    /// some extra details about the import operation
    pub aux: ImportedAux,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedAux {
    /// Only the header has been imported. Block body verification was skipped.
    pub header_only: bool,
    /// Clear all pending justification requests.
    pub clear_justification_requests: bool,
    /// Request a justification for the given block.
    pub needs_justification: bool,
    /// Received a bad justification.
    pub bad_justification: bool,
    /// Whether the block that was imported is the new best block.
    pub is_new_best: bool,
}

#[rpc(name = "engine_createBlock")]
fn create_block(
	&self,
	create_empty: bool,
	finalize: bool,
	parent_hash: Option<Hash>,
) -> FutureResult<CreatedBlock<Hash>>;
```

Because you are in control of block production, the node will not create blocks until you call this rpc method.

# Build Instructions

```bash
cargo build --release -p simnode-node
# run like so
./target/release/simnode-node --chain=picasso # pass all the args you would normally pass to the collator
```