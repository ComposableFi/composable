## MosaicVaultConfig

This contract is used to store configuration data for the MosaicVault contract.

Some of the methods are documented here. For a complete list of available methods check the interface `IMosaicVaultConfig`.

```solidity
function getUnderlyingIOUAddress(address _token) external view returns (address);

function getUnderlyingReceiptAddress(address _token) external view returns (address);

```

These methods are used to get the ReceiptToken address for the given ERC20 token address. This method will return a valid value only if the provided token address is whitelisted.

```solidity
function remoteTokenAddress(uint256 _id, address _token) external view returns (address);

```

This method returns the token address of the particular ERC20 on the remote network id. For example on the mainnet you can use this method to get the USDC address on Matic chain.

```solidity
function maxLimitLiquidityBlocks() external view returns (uint256);

function minLimitLiquidityBlocks() external view returns (uint256);

```

These function return the max and min number of blocks which can be specified for providing active liquidity.

[home](/readme.md) > [MosaicVault](/docs/MosaicVault.md)
