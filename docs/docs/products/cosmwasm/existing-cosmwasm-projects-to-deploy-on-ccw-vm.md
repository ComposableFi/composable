# How existing CW projects can deploy on the ccw-vm

This section explains how CosmWasm developers can test their contracts on the ccw-vm.
Generally, the entire codebase of a CosmWasm project can be compiled and uploaded to the ccw-vm on Picasso.
However, there is a minor difference between ccw-vm and the Cosmos chains' VM
which is the `data` field of the `Response` type.
Normally, it should be up to the users to use this `data` field and format it however they want.
But currently in Cosmos chains, after the instantiation of the contract, 
the `data` field is formatted with protobuf and the SDK adds the instantiated contract's address in that field.
The ccw-vm leaves this field up to the user and doesn't touch it at all.

Some contracts make use of this field, for example; 
contract **A** might instantiate an instance from contract **B** and can use the 
`data` field to read the address that is instantiated. 
However, ccw-vm does not interfere with the `data` field, 
so any contract that tries to read the contract address from this `data` field will fail. 
Instead of reading the address from the `data` field, contracts can read this address from the emitted events. 
A default `instantiate` event with the `contract_address` attribute key is emitted on every instantiate call.

Developers can test the deployment of their projects on Picasso through a standard Command Line Interface (CLI) 
that interacts with Pallet CosmWasm. At Composable, we utilize Nix to reproducibly build all necessary packages. 
**To get started testing out CW contracts on the ccw-vm, check out our guide [here].** 

[here]: ../../developer-guides/cosmwasm/walkthrough
