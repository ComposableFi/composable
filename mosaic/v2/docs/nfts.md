# Summoner NFT vault

[list of commands to deploy](/docs/configurations/nft-summoner.md)

```solidity
function transferERC721ToLayer(
        address _sourceNFTAddress,
        uint256 _sourceNFTId,
        address _destinationAddress,
        uint256 _destinationNetworkID,
        uint256 _transferDelay,
        address _feeToken
)
```

This function emits `TransferInitiated` with `isRelease == true` if the NFT is to be released on the destination network. This is a special case when original network of the NFT is equal to the destination network. On receiving this event the relayer has to call the `releaseSeal` method on the destination layer.

This function emits `TransferInitiated` with `isRelease == false` when the NFT is being transferred to a destination network other than the original network of the NFT. `TransferInitiated` event contains the information about the original NFT which is needed in the `summonNFT` method. On receiving this event the relayer has to call the `summonNFT` method on the destination layer.

```solidity
function releaseSeal(
        address _nftOwner,
        address _nftContract,
        uint256 _nftId,
        bytes32 _id,
        bool _isFailure
)
```

This method is called in 2 cases:

1. to release the original nft on the original network. The boolean `isFailure` is false.
2. to release the source nft on the source network in case of a failed transfer. The boolean `isFailure` is true.

```solidity
function summonNFT(
        string memory _nftUri,
        address _destinationAddress,
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId,
        bytes32 _id
)
```

this method is called to create/transfer a MosaicNFT on the destination network

## NFT transfer flow:

### Transfer to destination layer

- user calls `transferERC721ToLayer` on source layer -> `TransferInitiated` event with `isRelease == false`
- relayer reads the event on source layer and calls `summonNFT` on destination layer
- MosaicNFT is minted/transferred on destination layer

### transfer back to original layer

- user calls `transferERC721ToLayer` on the original layer with minted NFT -> `TransferInitiated` event with `isRelease == true`
- relayer reads the event and calls `releaseSeal` on original layer with `isFailure == false`
- original NFT is transferred back to the user

### Failed transfer to destination layer

- user calls `transferERC721ToLayer` on source layer -> `TransferInitiated` event
- relayer fails to transfer the NFT to destination layer
- relayer calls `releaseSeal` on the source with `isFailure == true`

## Fee operations

`feeAmounts`

- A mapping of remote network id => token address => fee amount.
- If the fee amount for a token is not greater than 0 it is considered invalid.
- The fee amount for the native token is mapped to `address(0)`.

Why use a dict?
Some layer 2 do not support a native token. If we want to deploy on such layers we need a separate contract for them
which supports only ERC20 for fees. So I had designed this contract for that situation.

**Flow**:

0. the user has to approve the `Summoner` for `feeAmounts[feeToken]` for ERC20 or supply the native
1. the user has to approve the ERC721 to the `Summoner` for the contract to be able to transfer it
2. user calls `transferERC721ToLayer` the fee is taken in the feeToken selected by the user and an event is emitted
3. successful transfer - no further action needed
4. failed transfer - the relayer calls `releaseSeal` and the fee will be refunded to the user

**How to withdraw fees from the vault**

```solidity
function withdrawFees(address feeToken, address withdrawTo);

```

An onlyOwner method to withdraw the fees collected in various tokens.

## How to manage fee between layers

Summoner i.e. the NFT vault collects fees on the source layer when a transfer transaction is done. The fees is collected
within the vault and needs to withdrawn.

To withdraw the fees use this:

```javascript
// assuming USDC is a allowed fee token
await summonerContract.connect(owner).withdrawFees(USDC_ADDRESS, owner.address);
// to withdraw native fee
await summonerContract.connect(owner).withdrawFees(ethers.constants.AddressZero, owner.address);
```

Since we are taking the fee on the origin network, but we are paying for the transaction on the destination network, if
the overall system is compensated we will be able to more or less keep both networks with enough balance.

But if the system is having more transfers from network A to B than the other way around, network B will need more funds
that should be taken from the fees earned on network A.

Those fees on network A then should be transferred and converted in a way that the owner of the vault on network B can
use them to pay for transactions.

**how to transfer fees collected from layer A to B**

First step is to transfer funds to the destination address on the remote layer using the composable l2 vaults. Other bridging options are of course posible, but we are using the potential of our own system to move this funds.

To transfer funds to another layer use this:

```javascript
// here owner is the address holding the funds on source layer
// remoteNetworkId is the destination chain id
await l2Vault.connect(owner).transferERC20ToLayer(
  amount,
  tokenAddress,
  remoteDestinationAddress,
  remoteNetworkID,
  0, //maxTransferDelay
  tokenAddress, //tokenOut
  0, //remoteAmmId
  0, //amountOutMin
  false //swapToNative
);
```

The above code will transfer the desired amount of tokens to the `remoteDestinationAddress` on destination layer.

Once the tokens are received on the destination layer you can swap them to the native tokens of that layer using an AMM.

For example, use this code to swap using sushiswap on polygon:

```js
await sushiSwapRouter
  .connect(owner)
  .swapExactTokensForETH(
    amountIn,
    amountOutMin,
    [tokenAddress, WETH_ADDRESS],
    destinationAddress,
    deadline
  );
```

## Aleph integration guide

You need an Ethereum account to write to aleph storage. You need aleph tokens to write anything.

Ethereum guide on how to create a aleph account:
https://aleph-im.github.io/aleph-js/guide/getting-started.html#ethereum

Guide on how to use key value storage for NFT metadata:
https://aleph-im.github.io/aleph-js/guide/getting-started.html#aggregates-key-value-storage

In the above guide instead of `account.address` you will use the `nftAddress` because we are writing metadata against the particular NFT.

The key will be the `nftId`.

Here is a code sample:

```js
// We update the 'mykey' key:
await aggregates.submit(
  nftContract.address,
  nftId,
  { a: 1, b: 2 }, // metadata
  {
    account: account, // the writing aleph account
    channel: "TEST", // a constant value for channel name
  }
);
```

[home](/readme.md)
