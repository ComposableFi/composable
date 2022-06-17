// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IMosaicHolding.sol";
import "./IVaultConfigBase.sol";

interface IMosaicVaultConfig is IVaultConfigBase {
    event MinFeeChanged(uint256 newMinFee);
    event MaxFeeChanged(uint256 newMaxFee);
    event MinLiquidityBlockChanged(uint256 newMinLimitLiquidityBlocks);
    event MaxLiquidityBlockChanged(uint256 newMaxLimitLiquidityBlocks);
    event LockupTimeChanged(address indexed owner, uint256 oldVal, uint256 newVal, string valType);
    event TokenWhitelisted(
        address indexed erc20,
        address indexed newIou,
        address indexed newReceipt
    );
    event TokenWhitelistRemoved(address indexed erc20);
    event RemoteTokenAdded(
        address indexed erc20,
        address indexed remoteErc20,
        uint256 indexed remoteNetworkID,
        uint256 remoteTokenRatio
    );
    event RemoteTokenRemoved(address indexed erc20, uint256 indexed remoteNetworkID);

    event PauseNetwork(address admin, uint256 networkID);

    event UnpauseNetwork(address admin, uint256 networkID);

    event AMMAdded(uint256 _id, address indexed _address);

    event AMMRemoved(uint256 _id);

    struct WhitelistedToken {
        uint256 minTransferAllowed;
        uint256 maxTransferAllowed;
        address underlyingIOUAddress;
        address underlyingReceiptAddress;
    }

    function passiveLiquidityLocktime() external view returns (uint256);

    function minFee() external view returns (uint256);

    function maxFee() external view returns (uint256);

    function maxLimitLiquidityBlocks() external view returns (uint256);

    function minLimitLiquidityBlocks() external view returns (uint256);

    function wethAddress() external view returns (address);

    function remoteTokenAddress(uint256 _id, address _token) external view returns (address);

    function remoteTokenRatio(uint256 _id, address _token) external view returns (uint256);

    function supportedAMMs(uint256 _ammID) external view returns (address);

    function pausedNetwork(uint256) external view returns (bool);

    function ableToPerformSmallBalanceSwap() external view returns (bool);

    function supportedMosaicNativeSwappers(uint256 _mosaicNativeSwapperID)
        external
        view
        returns (address);

    /**
     * @dev used to set the passive liquidity lock time
     * @param _locktime  Lock time in seconds until the passive liquidity withdrawal is unavailable
     */
    function setPassiveLiquidityLocktime(uint256 _locktime) external;

    /**
     * @dev used to set WETH address
     * @param _weth  Address of WETH token
     * @param _minTransferAmount Minimum transfer allowed amount
     * @param _maxTransferAmount Maximum transfer allowed amount
     */
    function setWethAddress(
        address _weth,
        uint256 _minTransferAmount,
        uint256 _maxTransferAmount
    ) external;

    function getUnderlyingIOUAddress(address _token) external view returns (address);

    function getUnderlyingReceiptAddress(address _token) external view returns (address);

    /**
     * @dev used to add address of the AMM used to swap tokens.
     * @param _ammID the integer constant for the AMM
     * @param _ammAddress Address of the AMM
     */
    function addSupportedAMM(uint256 _ammID, address _ammAddress) external;

    /**
     * @dev used to remove address of the AMM.
     * @param _ammID the integer constant for the AMM
     */
    function removeSupportedAMM(uint256 _ammID) external;

    function changeRemoteTokenRatio(
        address _tokenAddress,
        uint256 _remoteNetworkID,
        uint256 _remoteTokenRatio
    ) external;

    /**
     * @dev used to adds a whitelisted token to the contract.
     * @param _tokenAddress  SC address of the ERC20 token to add to supported tokens
     * @param _minTransferAmount Minimum amount of token can be transferred
     * @param _maxTransferAmount  Maximum amount of token can be transferred
     */
    function addWhitelistedToken(
        address _tokenAddress,
        uint256 _minTransferAmount,
        uint256 _maxTransferAmount
    ) external;

    /**
     * @dev used to removes whitelisted token from the contract.
     * @param _token  SC address of the ERC20 token to remove from supported tokens
     */
    function removeWhitelistedToken(address _token) external;

    function addTokenInNetwork(
        address _tokenAddress,
        address _tokenAddressRemote,
        uint256 _remoteNetworkID,
        uint256 _remoteTokenRatio
    ) external;

    function removeTokenInNetwork(address _tokenAddress, uint256 _remoteNetworkID) external;

    /**
     * @dev updates the minimum fee.
     * @param _newMinFee  value to be set as new minimum fee
     */
    function setMinFee(uint256 _newMinFee) external;

    /**
     * @dev updates the maximum fee.
     * @param _newMaxFee  value to be set as new minimum fee
     */
    function setMaxFee(uint256 _newMaxFee) external;

    /**
     * @dev updates the minimum limit liquidity block.
     * @param _newMinLimitLiquidityBlocks value to be set as new minimum limit liquidity block
     */
    function setMinLimitLiquidityBlocks(uint256 _newMinLimitLiquidityBlocks) external;

    /**
     * @dev updates the maximum limit liquidity block.
     * @param _newMaxLimitLiquidityBlocks value to be set as new maximum limit liquidity block
     */
    function setMaxLimitLiquidityBlocks(uint256 _newMaxLimitLiquidityBlocks) external;

    function generateId() external returns (bytes32);

    function inTokenTransferLimits(address, uint256) external view returns (bool);

    /**
     * @dev used to pause a network.
     * @param _networkID  network ID of remote token
     */
    function pauseNetwork(uint256 _networkID) external;

    /**
     * @dev used to unpause a network.
     * @param _networkID  network ID of remote token
     */
    function unpauseNetwork(uint256 _networkID) external;

    function setAbleToPerformSmallBalanceSwap(bool _flag) external;

    function addSupportedMosaicNativeSwapper(
        uint256 _mosaicNativeSwapperID,
        address _mosaicNativeSwapperAddress
    ) external;

    function removeSupportedMosaicNativeSwapper(uint256 _mosaicNativeSwapperID) external;
}
