Passing this proposal gives the address add OSMO address the ability to upload CosmWasm contracts to Osmosis without seeking further governance approval for each upload. This address is administered by Composable core contributors.


## **About Composable VM**

[https://docs.composable.finance/products/xcvm](https://docs.composable.finance/products/xcvm)

Composable VM (CVM) is a set of smart contracts that allow orchestration between protocols over IBC. Similar to generalized message passing, CVM can be used to abstract away individual actions from the end user to deliver a more streamlined UX. CVM operates on top of Composable’s IBC-based transfer protocol, Centauri. 


## **Details**

The end-goal of CVM will be to allow a user to interact with multiple chains and platforms while operating out of a single wallet, from a single chain. In its simplest form, this may be exemplified by a user swapping tokens between DEX’s on different chains. That is, a Polkadot user may swap DOT with a token on Osmosis DEX by having their DOT transferred across Centauri to Osmosis, swapped for a given token, and having that token delivered back to their wallet on Polkadot. This creates an experience that abstracts away the complexities of asking a user to perform multiple bridge transfers, in addition to a swap within a separate ecosystem.

By deploying the CVM contract on Osmosis, users would be able to swap any DotSama token with any Cosmos based token with liquidity on Osmosis. 


## Forward Looking

In addition to swap operations, actions such as staking act as another strong example of operations CVM could perform for a user. Taking the last example, a user on Polkadot could transfer assets to Osmosis, swap to OSMO, perform a staking operation on Stride to receive stOSMO, and have this stOSMO transferred back to their Polkadot wallet all with only having to sign for one transaction.

Once additional features are added to CVM, new contract versions will need to be uploaded to Osmosis. In the context of this proposal, will only be uploading the contract for routing, transfers, and swaps. Once a new version of the contract becomes available, an additional governance post will be created in order to upload the new contract. 


## **Mechanism**

The CVM contract to be deployed on Osmosis is responsible for handling two things:



1. Receive swap path and minimum output amount and execute it. 
    * Path is a sequence of pools that will be used to go from token A to token B, the same way as in the Uniswap V2 router.
2. In case of a successful swap execute specified ‘after swap action’ which can be either bank send or contract call or ibc transfer.

Since the only responsibility of this contract is to perform swaps it is stateless and does not require any ownership or pausable functions.

The contract also handles fallback scenarios for IBC transfers, in case of packet failure or timeout contract will transfer swapped funds to the specified fallback_address.


## **Contract information:**



* The release for the contract is available at - `nix build github:ComposableFi/composable/d4d01f19d8fbe4eafa81f9f2dfd0fd4899998ce6#xc-cw-contracts`
* The git commit Id - d4d01f19d8fbe4eafa81f9f2dfd0fd4899998ce6
* The code can be found at - https://github.com/ComposableFi/composable/tree/d4d01f19d8fbe4eafa81f9f2dfd0fd4899998ce6/code/xcvm/cosmwasm/contracts/
* Compiler Version - https://github.com/ComposableFi/composable/blob/d4d01f19d8fbe4eafa81f9f2dfd0fd4899998ce6/flake.lock
* Checksum - 


## **Contact**

Twitter: [https://twitter.com/](https://twitter.com/squidrouter)composablefi

Discord: [https://discord.gg/](https://discord.gg/squidrouter)composablefi

Website: [https://composable.](https://squidrouter.com/)finance
