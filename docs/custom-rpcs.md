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
sp-api = { default-features = false, git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.18" }
codec = { default-features = false, features = ["derive"], package = "parity-scale-codec", version = "3.0.0" }

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

        // as many RPCs as are needed for the pallet can all be defined here in the same trait
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
sp-api = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.18" }
sp-blockchain = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.18" }
sp-runtime = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.18" }
sp-std = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v0.9.18" }

# SCALE
scale-info = { version = "2.1.1", features = ["derive"] }
codec = { version = "3.0.0", package = "parity-scale-codec", features = ["derive"] }

# local
pallet-name-runtime-api = { path = "../runtime-api" }

# rpc
jsonrpc-core = "18.0.0"
jsonrpc-core-client = "18.0.0"
jsonrpc-derive = "18.0.0"
```

Note that this crate will only be included in the node and not the runtime, so there is no need for a `std` feature.

### In `lib.rs`

Required imports:

```rust
use pallet_name_runtime_api::PalletNameRuntimeApi;
use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::{sync::Arc, marker::PhantomData};
```

This defines the RPC itself. The name of the RPC needs to follow the pattern of `moduleName_functionName`.

**Note**: Any types that either are or are a wrapper type around `u128/i128` will need to be wrapped in `composable_support::rpc_helpers::SafeRpcWrapper`.

```rust
#[rpc]
pub trait PalletNameApi<BlockHash, /* ...any generic parameters... */>
where
    GENERIC_PARAMETER: Codec, 
{
    // the name of the rpc must be moduleName_functionName, where both module
    // and function are camelCase and are separated by an underscore.
    #[rpc(name = "palletName_rpcFunctionName")]
    fn rpc_function_name(
        &self,
        // any additional parameters here
        // if the type is or wraps a 128 bit integer, it should be declared as follows:
        u128_ish: SafeRpcWrapper</* whatever the type is */>
        at: Option<BlockHash>, // `at` should be last
    ) -> RpcResult<ReturnType>;
}
```

This is a struct that will implement the above API. It contains the client to make the RPC calls.

If there are more generics, instead of adding more parameters (`PalletName<C, M, N, P, ...>`), just use a tuple instead: `PalletName<C, (M, N, P, ...)`

```rust
pub struct PalletName<C, Block> {
    client: Arc<C>,
    _marker: PhantomData<Block>,
}

impl<C, M> PalletName<C, M> {
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _marker: Default::default() }
    }
}
```

```rust
impl<C, Block, /* ...any generic parameters... */>
    PalletNameApi<<Block as BlockT>::Hash, /* ...any generic parameters... */>
    for PalletName<C, /* ...any generic parameters, enclosed in a tuple... */>
where
    Block: BlockT,
    // all generic parameters must have at least these bounds
    GENERIC_PARAMETER: Codec + Send + Sync + 'static + FromStr + Display,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: PalletNameRuntimeApi<Block, AssetId, AccountId, Balance>,
{
    fn rpc_function_name(
        &self,
        // any additional parameters here
        // if the type is or wraps a 128 bit integer, it should be declared as follows:
        u128_ish: SafeRpcWrapper</* whatever the type is */>
        at: Option<<Block as BlockT>::Hash>, // `at` should be last
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

/home/ben/codeprojects/composable/integration-tests/runtime-tests/src/types/interfaces/definitions.ts

### Type Definitions

Create a folder here: `integration-tests/runtime-tests/src/types/interfaces/pallet-name`

And then within that folder, create a file `defintions.ts` with the following structure:

```typescript
export default {
  rpc: {
    // the functionName part of the RPC call as defined in the `#[rpc(name="")]` annotation on the rust definition
    rpcFunctionName: {
      description: "Provide a short description of the RPC here.",
      params: [
        // define the parameters in the same order as defined in the rust RPC
        {
          name: "parameter_name",
          type: "ParameterType"
        },
        // see note below
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "ReturnType"
    },
    // if there are multiple RPCs, they can all be defined here
  },
  types: {
      // define any custom types for the pallet here
      // see the note below for more information
  },
};
```

Then, in `integration-tests/runtime-tests/src/types/interfaces/definitions.ts`, add the following line:

```typescript
export { default as palletName } from "./palletName/definitions";
```

Notes:

* `at` is mandatory, and is defined as the last parameter in the rust RPC definition for a reason:
  Most of the time when calling an RPC the block hash can be omitted, and the best hash will be assumed if one is not provided.

  Having it as the last parameter makes calling the RPC simpler:

  ```typescript
  palletName.rpcFunctionName(param1, param2)
  ```

  Instead of:

  ```typescript
  palletName.rpcFunctionName(null, param1, param2)
  ```

  If `at` were defined first.

  Technically, it is possible to define `at` anywhere in the RPC definition, but putting it last for all of them makes the RPCs simpler and more consistent.

* If this is a preexisting pallet, the types for it are most likely already defined in the type definitions for `crowdloanRewards` (for reasons that don't need to be covered in this document) and can just be moved over to this file.

  Even if there are no types to declare, still define an empty object or else everything will explode.

### Tests

Create a folder here (if it doesn't already exist): `integration-tests/runtime-tests/test/tests/pallet-name`

And then within that folder, create a file `rpcPalletNameTests.ts` with the following structure:

```typescript
/* eslint-disable no-trailing-spaces */
import { /* any custom defined types that are needed for the RPC */ } from '@composable/types/interfaces';
import { expect } from 'chai';


describe('query.palletName.account Tests', function() {
  // Set timeout to 1 minute.
  this.timeout(60*1000); // <- increase this if tests are timing out

  // repeat this block as needed for every test case defined in the class below.
  it('rpc.palletName.functionName Tests', async function() {
    await RpcPalletNameTests.rpcPalletNameFunctionNameTest();
  });
});


export class RpcPalletNameTests {
    /**
     * 
     */
    public static async rpcPalletNameFunctionNameTest() {
        // api is a global variable
        const result = await api.rpc.palletName.functionName(/* parameters */);

        // see note below about bignumbers
        // (this is just an example assertion)
        expect(result).to.be.a["bignumber"].that.equals('0');
    }
}
```

Notes:

* If the type being compared against is a `u128`/`i128` on the rust side and has been wrapped in `SafeRpcWrapper`, it will be a bn.js `BN` (big number) here. `chai-bn` is used for asssertions with `BN`s but typescript can't quite figure out that it's being used; using `["bignumber"]` instead of `.bignumber` circumvents the typechecker a bit and allows it to pass without a `@ts-ignore` comment.

  Thanks Dominik for figuring this one out!

## Additional Resources

For a good overview of how custom RPCs work, see <https://core.tetcoin.org/recipes/custom-rpc.html>.
