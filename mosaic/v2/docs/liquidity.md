## Receipt Token - passive liquidity

### Deposit

Receipt token is issued for the user who deposit passive liquidity

#### Steps to add passive liquidity:

#### ERC20

1. Approve MosaicVault address in token contract
2. Call `providePassiveLiquidity` function in MosaicVault

**providePassiveLiquidity** function:

```solidity
function providePassiveLiquidity(uint256 _amount, address _tokenAddress) external;

```

- **\_amount** Amount of token user want to provide as liquidity
- **\_tokenAddress** Address of the ERC20 compatible token

#### ETH

1. User called `providePassiveLiquidity` with desired ETH value, ignoring (setting to 0) the `_amount` and `_tokenAddress` params

```solidity
function providePassiveLiquidity(uint256 _amount, address _tokenAddress) external;

```

#### Event emitted:

```solidity
DepositPassiveLiquidity(address indexed tokenAddress, address indexed provider, uint256 amount);
```

- **tokenAddress** Address of the token
- **provider** Address of the user who provide liquidity in the vault
- **amount** Amount of token user deposit

_Only whitelisted tokens allowed_

Equal amount of **Receipt** tokens will be minted for the user address and the initial tokens will be transferred in
MosaicHolding address

### Withdraw

Passive liquidity can be withdrawn anytime. In order to withdraw the passive liquidity, Receipt token will be burned

_If user deposit ETH as active liquidity, he will get back WETH token_

#### Steps to withdraw passive liquidity:

1. User add a withdrawal liquidity request
2. Relayer will call the **withdrawLiquidity** function and send the token to the user

withdrawLiquidity function:

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

```

- **\_receiptToken** Address of the corresponding receipt token
- **\_amountIn** Amount of token user want to withdraw
- **\_tokenOut** User have the possibility to withdraw the liquidity in different token than the one he deposited
- **\_destinationAddress** Address of the receiver of the liquidity and rewards
- **\_ammID** Id of the exchange that should be used if user opted to get back the liquidity in other token
- **\_data** Additional data required by the exchange
- **\_destinationNetworkId** ID of the network user will receive the liquidity back
- **\_withdrawRequestData** Consist of `amountOutMin`, `maxDelay` and `_swapToNative`

Event emitted

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
  WithdrawRequestData _withdrawData
);
```

Relayer listen for this event, calculate the rewards and send the liquidity to the user

## IOU Token - active liquidity

### Deposit

IOU token is issued for the user who deposit active liquidity

#### Steps to add active liquidity:

#### ERC20

1. Approve MosaicVault address in token contract
2. Call `provideActiveLiquidity` on the MosaicVault contract

```solidity
function provideActiveLiquidity(
  uint256 _amount,
  address _tokenAddress,
  uint256 _blocksForActiveLiquidity
) external;

```

- **\_amount** Amount of token user deposit
- **\_tokenAddress** Address of the token
- **\_blocksForActiveLiquidity** Number of block liquidity is available

#### ETH

1. User called `provideActiveLiquidity` with desired ETH value and number of blocks he wants to be available for, ignoring (setting to 0) the \_amount and \_tokenAddress params

```solidity
function provideActiveLiquidity(
  uint256 _amount,
  address _tokenAddress,
  uint256 _blocksForActiveLiquidity
) external;

```

- **\_blocksForActiveLiquidity** Number of block liquidity is available

#### Event emitted:

```solidity
DepositActiveLiquidity(
  address indexed tokenAddress,
  address indexed provider,
  uint256 amount,
  uint256 blocks
);
```

### Withdraw

Active liquidity can be withdrawn after number of block user selected passed. In order to withdraw the active liquidity,
IOU token will be burned

_If user deposit ETH as active liquidity, he will get back WETH token_

#### Steps to withdraw active liquidity:

1. User add a withdrawal liquidity request
2. Relayer will call the **withdrawLiquidity** function and send the token to the user

withdrawLiquidity function:

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

```
