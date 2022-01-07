# Adding an RPC interface to a pallet

Create 2 new crates beside the `src/` folder of the pallet you want to add the RPC to:

```plaintext
frame
  pallet
    src
    Cargo.toml
    rpc
      src
      Cargo.toml
    runtime-api
      src
      Cargo.toml
```

Add the two new crates to the composable workspace file (top level Cargo.toml):

```toml
members = [
    # add these lines
    "frame/pallet-name/rpc",
    "frame/pallet-name/runtime-api",
]
```

## Runtime API Crate

### In `Cargo.toml`

```toml
[package]
name = "pallet-name-runtime-api"
version = "0.0.1"
authors = ["Composable Developers"]
homepage = "https://composable.finance"
edition = "2021"
rust-version = "1.56"

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
sp-api = { default-features = false, git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.16" }
codec = { default-features = false, features = [
    "derive",
], package = "parity-scale-codec", version = "2.0.0" }

# ...any other dependencies, as per usual

[features]
default = ["std"]
std = ["sp-api/std"]

```

### In `lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)] // REVIEW: I'm not sure if these are actually necessary
#![allow(clippy::unnecessary_mut_passed)] // REVIEW: I'm not sure if these are actually necessary

sp_api::decl_runtime_apis! {
    pub trait PalletNameRuntimeApi</* ...any generic parameters... */>
    where
        GENERIC_PARAMETER: codec::Codec, // all parameters must implement `Codec`
    {
        fn rpc_function_name(/* ...parameters... */) -> ReturnType;

        // as many rpcs as are needed for the pallet can all be defined here in the same trait
    }
}
```

## RPC Crate

### In `Cargo.toml`

```toml
[package]
name = "PALLET-NAME-rpc"
version = "0.0.1"
authors = ["Composable Developers"]
homepage = "https://composable.finance"
edition = "2021"
rust-version = "1.56"

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
# substrate primitives
sp-api = { default-features = false, git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.16" }
sp-blockchain = { default-features = false, git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.16" }
sp-runtime = { default-features = false, git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.16" }
sp-std = { default-features = false, git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.16" }

scale-info = { version = "1.0", default-features = false, features = ["derive"] }
codec = { default-features = false, features = ["derive"], package = "parity-scale-codec", version = "2.0.0" }

pallet-name-runtime-api = { path = "../runtime-api", default-features = false }

# rpc
jsonrpc-core = "18.0.0"
jsonrpc-core-client = "18.0.0"
jsonrpc-derive = "18.0.0"

[features]
default = ["std"]
std = [
    "pallet-name-runtime-api/std",
    "codec/std",
    "sp-runtime/std",
    "sp-api/std",
]
```

### In `lib.rs`

Required imports:

```rust
use PALLET_NAME_runtime_api::PALLET_NAME_RuntimeApi;
use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::sync::Arc;
```

This defines the RPC itself; the name of the RPC needs to follow the pattern of `moduleName_functionName`.

```rust
#[rpc]
pub trait PALLET_NAME_Api<BlockHash, /* ...any generic parameters... */>
where
    GENERIC_PARAMATER: Codec,
{
    // the name of the rpc must be module_function, where both module
    // and function are camelCase and are seperated by an underscore.
    #[rpc(name = "palletName_rpcFunctionName")]
    fn rpc_function_name(
        &self,
        // any additional parameters here
        at: Option<BlockHash>, // `at` should be last
    ) -> RpcResult<ReturnType>;
}
```

This is a struct that will inplement the above API. It contains the client to make the RPC calls.

If there are more generics, instead of adding more parameters (`Assets<C, M, N, P, ...>`), just use a tuple instead: `Assets<C, (M, N, P, ...)`

```rust
pub struct PalletName<C, Block> {
    client: Arc<C>,
    _marker: sp_std::marker::PhantomData<Block>,
}

impl<C, M> PalletName<C, M> {
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _marker: Default::default() }
    }
}
```

```rust
impl<C, Block, /* ...any generic parameters... */>
    AssetsApi<<Block as BlockT>::Hash, /* ...any generic parameters... */>
    for Assets<C, /* ...any generic parameters, enclosed in a tuple... */>
where
    Block: BlockT,
    GENERIC_PARAMETER: Codec + Send + Sync + 'static, // all generic parameters must have at least these bounds
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: AssetsRuntimeApi<Block, AssetId, AccountId, Balance>,
{
    fn rpc_function_name(
        &self,
        // any additional parameters here
        at: Option<<Block as BlockT>::Hash>, // `at` must be last
    ) -> RpcResult<ReturnType> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| {
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        }));

        let runtime_api_result = api.rpc_function_name(&at, asset_id, account_id);
        // TODO(benluelo): Review what error message & code to use
        runtime_api_result.map_err(|e| {
            RpcError {
                code: ErrorCode::ServerError(9876), // No real reason for this value
                message: "Something wrong".into(),
                data: Some(format!("{:?}", e).into()),
            }
        })
    }
}
```

## In `node/`

The RPC needs to be added to the node in order to be called. First, add both the above crates to `node/Cargo.toml`:

```toml
[dependencies]
# ...stub...
pallet-name-rpc = { path = "../frame/pallet-name/rpc" }
pallet-name-runtime-api = { path = "../frame/pallet-name/runtime-api" }
```

Then, in `node/src/runtime.rs`, add a bound in both the definition and blanket impl for `HostRuntimeApis`:

```rust
pub trait HostRuntimeApis:
    // ...stub...
    + pallet_name_runtime_api::PalletNameRuntimeApi<Block, /* ...any generic parameters... */>,
    // ...stub...

impl<Api> HostRuntimeApis for Api
where
    Api: // ...stub...
        + pallet_name_runtime_api::PalletNameRuntimeApi<Block, /* ...any generic parameters... */>,
        // ...stub...
```

Then, in `node/src/rpc.rs`, in  `fn create(...)`, add a bound to `C::Api`, and then within the function, add the RPC to the runtime:

```rust
pub fn create<C, P, B>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    // ...stub...
    C::Api: pallet_name_runtime_api::PalletNameRuntimeApi<B, /* ...any generic parameters... */>,
    // ...stub...
{
    // ...stub...
    io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(client)));

    io
}
```

## Add to runtimes

Within each runtime's `Cargo.toml`, add the runtime-api dependency:

```toml
# ...stub...
[dependencies]
pallet-name-runtime-api = { path = '../../frame/pallet-name/runtime-api', default-features = false }
# ...stub...
```

And then in each runtime's `lib.rs`, in the `impl_runtime_apis!` macro, implement the API:

```rust
impl_runtime_apis! {
    impl pallet_name_runtime_api::PalletNameRuntimeApi<Block, /* ...any generic parameters... */> for Runtime {
        fn rpc_function_name(/* ...rpc parameters... */) -> ReturnType {
            // actual implementation here
        }
    }
    // ...stub...
}
```

Note that this assumes that the pallet has already been added to the runtime and the pallet's `Config` already implemented.

## Integration Tests

## Also see

For a good overview of how custom RPCs work, see <https://core.tetcoin.org/recipes/custom-rpc.html>.
