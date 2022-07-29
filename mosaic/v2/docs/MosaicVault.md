## MosaicVault

MosaicVault is deployed on each layer and is the primary contract for interaction. Each layer has a vault which is used
to control the funds deposited as liquidity.

Components of MosaicVault:

- [MosaicHolding](/docs/MosaicHolding.md)
- [MosaicVaultConfig](/docs/MosaicVaultConfig.md)

[list of commands to deploy](/docs/configurations/mosaic-vault.md)

### EOA callable methods

#### Transfer funds

```solidity
function transferERC20ToLayer(
  uint256 _amount,
  address _tokenAddress,
  address _remoteDestinationAddress,
  uint256 _remoteNetworkID,
  uint256 _maxTransferDelay,
  address _tokenOut,
  uint256 _remoteAmmId,
  uint256 _amountOutMin,
  bool _swapToNative
) external returns (bytes32 transferId);

```

This method is used to transfer funds to another layer. The relayer reads the event emitted by this method and then gives the funds to the user on the destination network in the desired `_tokenOut`. A fee is deducted from the funds given to the user.

The corresponding method to transfer ETH is `transferETHToLayer`

```solidity
function transferETHToLayer(
  address _remoteDestinationAddress,
  uint256 _remoteNetworkID,
  uint256 _maxTransferDelay,
  address _tokenOut,
  uint256 _remoteAmmId,
  uint256 _amountOutMin,
  bool _swapToNative
) external payable returns (bytes32 transferId);

```

Event emitted:

```solidity
TransferInitiated(
  address indexed owner,
  address indexed erc20,
  address remoteTokenAddress,
  uint256 indexed remoteNetworkID,
  uint256 value,
  address remoteDestinationAddress,
  bytes32 uniqueId,
  uint256 maxTransferDelay,
  address tokenOut,
  uint256 ammID,
  uint256 amountOutMin,
  bool _swapToNative
);
```

```solidity
function withdrawTo(
  address _accountTo,
  uint256 _amount,
  address _tokenIn,
  address _tokenOut,
  bytes32 _id,
  uint256 _baseFee,
  uint256 _amountOutMin,
  uint256 _ammID,
  WithdrawData calldata _withdrawData,
  bytes calldata _data
) external;

struct WithdrawData {
  uint256 feePercentage;
  uint256 baseFee;
  address investmentStrategy;
}

```

The relayer listens to `TransferInitiated` event on the source layer. Then calls the `withdrawTo` method on the
destination layer vault. This method deducts the fee for the transfer and sends the requested funds to the user.

The `_baseFee` param is used by the relayer to deduct a fee for itself to fund the gas used in the transaction.

Diagram for a successful transfer

![transfer-flow](/docs/images/transfer-erc20-to-layer.png)

#### Provide liquidity

```solidity
function provideActiveLiquidity(
  uint256 _amount,
  address _tokenAddress,
  uint256 _blocksForActiveLiquidity
) external;

```

This method is used to provide active liquidity in the vault and receive back `IOUToken` in return.

`_blocksForActiveLiquidity` is the number of blocks the user wants the liquidity to be active. At the end of the period
the liquidity becomes passive.

The value of `_blocksForActiveLiquidity` should be within the range set by the contract.

Event emitted:

```solidity
DepositActiveLiquidity(
  address indexed tokenAddress,
  address indexed provider,
  uint256 amount,
  uint256 blocks
);
```

```solidity
function providePassiveLiquidity(uint256 _amount, address _tokenAddress) external;

```

This method is used to provide passive liquidity in the vault and receive back `ReceiptToken` in return.

Event emitted:

```solidity
DepositPassiveLiquidity(
  address indexed tokenAddress,
  address indexed provider,
  uint256 amount
);
```

![provideLiquidity](/docs/images/provide-liquidity.png)

#### Withdraw Liquidity

```solidity
function withdrawLiquidityRequest(
  address _receiptToken,
  uint256 _amountIn,
  address _tokenOut,
  address _destinationAddress,
  uint256 _ammID,
  bytes calldata _data,
  uint256 _destinationNetworkId,
  WithdrawRequestData calldata _withdrawRequestData
) external returns (bytes32 transferId);

struct WithdrawRequestData {
  uint256 amountOutMin;
  uint256 maxDelay;
  bool _swapToNative;
}

```

`WithdrawRequestData` is a struct that keep the minimum requirements set by the user for the transfer to succeed

1. `amountOutMin` - Minimum amount of token user receive in case of a swap
2. `maxDelay` - Maximum delay between requesting a withdrawal and receiving the tokens
3. `_swapToNative` - `true` if a part will be swapped to native token in destination

The user calls this method to register a request to withdraw the liquidity. The `_receiptToken` can be either IOU or
Receipt token.

If the `_receiptToken` is a IOUToken then the liquidity provided by the user was `active` and the contract will check if
the `_blocksForActiveLiquidity` period is over.

The user can use the params in the function to withdraw liquidity on any whitelisted network and token of choice.

If the request is valid the contract emits the `WithdrawRequest` event and `_receiptToken` are burned.

```solidity
WithdrawRequest(
  address indexed user,
  address receiptToken,
  address indexed tokenIn,
  uint256 amountIn,
  address indexed tokenOut,
  address remoteTokenInAddress,
  address destinationAddress,
  uint256 ammId,
  uint256 destinationNetworkID,
  bytes data,
  bytes32 uniqueId,
  WithdrawRequestData _withdrawData,
);
```

The relayer reads `WithdrawRequest` event and calls `withdrawLiquidity` on the `_destinationNetworkId` passed in the
request. If there is enough liquidity on the `_destinationNetworkId` for `tokenIn` the liquidity is withdrawn and given
back to the user.

```solidity
function withdrawLiquidity(
  address _receiver,
  address _tokenIn,
  address _tokenOut,
  uint256 _amountIn,
  uint256 _baseFee,
  uint256 _amountOutMin,
  uint256 _ammID,
  WithdrawData calldata _withdrawData,
  bytes calldata _data,
  bytes32 _id
) external;

struct WithdrawData {
  uint256 feePercentage;
  uint256 baseFee;
  address[] investmentStrategies;
  uint256 ammId;
  bytes32 id;
  uint256 amountToSwapToNative;
  uint256 nativeSwapperId;
}

```

In case of low liquidity on the vault, funds are automatically withdrawn from investment strategy

![withdrawLiquidity](/docs/images/withdraw-liquidity.png)

#### Digest funds

```solidity
function digestFunds(address _token) external;

```

This method is used to transfer randomly sent tokens to this contract to the Mosaic holding

[home](/readme.md)
