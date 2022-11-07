# IBC Integration for XCVM

Composable XCVM is a cross-chain virtual machine that is transport and execution
layer agnostic. One of it's execution layers is CosmWasm and one of it's
transport layers is IBC. CosmWasm is IBC native and provide facilities
(functions, metadata) to leverage IBC in cross-chain communications. We support
CosmWasm as smart contract layering on our Picasso parachain.

In the context of the parachain, we decided to split the CosmWasm implementation
between two components. First, an abstract host agnostic VM that can be
implemented by any infrastructure (as long as it satisfies the host interface).
The second component is a Substrate (polkadot framework) Pallet (or module) that
implements the host functions and instantiates the abstract VM to execute
contracts.

## A. CosmWasm VM extension for IBC

Composable is building a CosmWasm VM in an incremental way. To do so, we
initially choose not to implement the IBC/Staking/Distribution features.

Cosmoverse demo has been a success, but as everyone know, the IBC infrastructure
was not yet ready. Now that the Composable bridging team has successfully
connected two parachains via Centauri (trustless bridge on top of the IBC
protocol), XCVM can leverage IBC itself.

The need for IBC to be supported by the CosmWasm VM is now becoming important
for this purpose.

#### 1. Enabling CosmWasm contracts to submit IBC packets

Our initial CosmWasm VM implementation would fault whenever a contract would
submit an IBC packet. Similarly, the IBC callbacks defined by a contract on
channel opening, packet acknowledgement would never be called (as per the
previous statement).

To enable this feature, we should extend the
[`CosmosMsg`](https://github.com/CosmWasm/cosmwasm/blob/531ecc3d942af2040a3a2ce57db9a449110349c7/packages/std/src/results/cosmos_msg.rs#L25)
data structures to support the
[`IbcMsg`](https://github.com/CosmWasm/cosmwasm/blob/ded2c78d57b40ac050892b6253ec0a9235246ea5/packages/std/src/ibc.rs#L23)
variant. This will allow contracts to submit packets and transfer assets via the
host, assuming it supports IBC.

#### 2. Enabling CosmWasm VM host to call back contracts on IBC events

Similarly, we need to extend the VM abstract host to allow it to call back
contracts on IBC events. The list of events a contract can listen to is [defined
in the original CosmWasm
specification](https://github.com/CosmWasm/cosmwasm/blob/531ecc3d942af2040a3a2ce57db9a449110349c7/IBC.md)
This ability will be implemented by extending the [`VMBase`
interface](https://github.com/ComposableFi/cosmwasm-vm/blob/24d22367af7602aecc84da390f6f22f88f35b6bb/vm/src/vm.rs#L141) for IBC operations.

## B. Pallet CosmWasm extensions for IBC

Now that the abstract CosmWasm VM is IBC capable, we still need to upgrade the
host to implement the extra functions required to keep satisfying the interface.
And finally, we need to consume the IBC Pallet from the CosmWasm Pallet to
forward IBC events from the underlying VM to the IBC Pallet and calls backs
contracts whenever required.

#### 1. Upgrading the CosmWasm Pallet Host to the IBC capable VM

After having upgraded the CosmWasm VM to IBC. We need to extend the host
functions implemented by the CosmWasm Pallet to stay compatible with the VM. The
first thing to do will be to upgrade the CosmWasm VM reference (dependency in
Cargo) and then follow the missing functions to implement them. They can
initially be stub functions to only satisfy the interface.

#### 2. Integrating IBC Pallet in the CosmWasm Pallet

Parallelly, the actual IBC implementation (held by the IBC Pallet made by our
bridging team) must be consumed by the CosmWasm Pallet. To do so, we need to add
the IBC Pallet as dependency of the CosmWasm Pallet and try interacting with the
interface to execute IBC operations. If the interface is satisfying and comply
to the CosmWasm Pallet requirements, we can tie the IBC Pallet interface with
the CosmWasm VM host interface (the CosmWasm Pallet would act as a router
between the underlying VM and the IBC pallet).

#### 3. Tying IBC Pallet with the CosmWasm VM

Once the IBC Pallet is a dependency of the CosWasm Pallet and it's interface is
compatible with our expectations, we need to tie the IBC Pallet interface with
the CosmWasm Pallet Host implementation, effectively making the functions
defined in section **B.1.** not stub anymore. Consequently, we will also be able
to extend the CosmWasm Pallet benchmarking to cover the weights of this
functions.

#### 4. IBC callbacks for contracts

The IBC specification state that the IBC module is only consumed. It does not
mention how modules consuming IBC can be called back when events occur (channel
opened, closed, packet sent...).

It is unclear how we are going to poll events from the IBC Pallet to dispatch
them on the according contracts. It can be probably done with transactions, but
also via block hooks or off-chain workers. This is the final piece that
will tie everything together and make our CosmWasm Pallet IBC capable.

