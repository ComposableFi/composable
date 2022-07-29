// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IInvestmentStrategy.sol";
import "./IMosaicVaultConfig.sol";

/**
 * @dev Interface of L1Vault.
 */
interface IMosaicVault {
    event TransferInitiated(
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

    event WithdrawalCompleted(
        address indexed accountTo,
        uint256 amount,
        uint256 netAmount,
        address indexed tokenAddress,
        bytes32 indexed uniqueId,
        bool swapToNative
    );

    event TransferFundsRefunded(
        address indexed tokenAddress,
        address indexed user,
        uint256 amount,
        uint256 fullAmount,
        bytes32 indexed uniqueId
    );
    event FundsDigested(address indexed tokenAddress, uint256 amount);

    event FeeTaken(
        address indexed owner,
        address indexed user,
        address indexed token,
        uint256 amount,
        uint256 fee,
        uint256 baseFee,
        uint256 totalFee,
        bytes32 uniqueId
    );

    event DepositActiveLiquidity(
        address indexed tokenAddress,
        address indexed provider,
        uint256 amount,
        uint256 blocks
    );

    event DepositPassiveLiquidity(
        address indexed tokenAddress,
        address indexed provider,
        uint256 amount
    );

    event LiquidityWithdrawn(
        address indexed user,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 requestedAmountIn,
        uint256 amountOut,
        uint256 baseFee,
        uint256 mosaicFee,
        bool swapToNative,
        bytes32 id
    );

    event LiquidityRefunded(
        address indexed tokenAddress,
        address indexed receiptAddress,
        address indexed user,
        uint256 amount,
        bytes32 uniqueId
    );

    event WithdrawRequest(
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

    event RelayerSet(address indexed _old, address indexed _new);
    event ConfigSet(address indexed _old, address indexed _new);

    enum TransferState {
        UNKNOWN, // Default state
        SUCCESS,
        REFUNDED
    }

    struct WithdrawRequestData {
        uint256 amountOutMin;
        uint256 maxDelay;
        bool _swapToNative;
    }

    struct WithdrawData {
        uint256 feePercentage;
        uint256 baseFee;
        address[] investmentStrategies;
        bytes[] investmentStrategiesData;
        uint256 ammId;
        bytes32 id;
        uint256 amountToSwapToNative;
        uint256 minAmountOutNative;
        uint256 nativeSwapperId;
    }

    function setRelayer(address _relayer) external;

    function setVaultConfig(address _vaultConfig) external;

    function vaultConfig() external view returns (IMosaicVaultConfig);

    /**
     * @dev used to provide active liquidity.
     * @param _amount amount of tokens to deposit
     * @param _tokenAddress  SC address of the ERC20 token to deposit
     * @param _blocksForActiveLiquidity users choice of active liquidity
     */
    function provideActiveLiquidity(
        uint256 _amount,
        address _tokenAddress,
        uint256 _blocksForActiveLiquidity
    ) external payable;

    /**
     * @dev used to provide passive liquidity.
     * @param _amount amount of tokens to deposit
     * @param _tokenAddress  SC address of the ERC20 token to deposit
     */
    function providePassiveLiquidity(uint256 _amount, address _tokenAddress) external payable;

    /// @notice External function called to add withdraw liquidity request
    /// @param _receiptToken Address of the iou token provider have
    /// @param _amountIn Amount of tokens provider want to withdraw
    /// @param _tokenOut Address of the token which LP wants to receive
    /// @param _ammID the amm to use for swapping
    /// @param _data extra call data
    /// @param _destinationNetworkId networkId of the _receiptToken's underlying token
    /// @param _withdrawRequestData set of data for withdraw
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

    /**
     *  @notice Called by relayer to withdraw liquidity from the vault
     *  @param _accountTo eth address to send the withdrawal tokens
     *  @param _amount requested by user + rewards for token in
     *  @param _requestedAmount amount requested by user for token in
     *  @param _tokenIn address of the token in
     *  @param _tokenOut address of the token out
     *  @param _amountOutMin minimum amount out user wants
     *  @param _withdrawData set of data for withdraw
     *  @param _data additional _data required for each AMM implementation
     */
    function withdrawLiquidity(
        address _accountTo,
        uint256 _amount,
        uint256 _requestedAmount,
        address _tokenIn,
        address _tokenOut,
        uint256 _amountOutMin,
        WithdrawData calldata _withdrawData,
        bytes calldata _data
    ) external;

    /**
     * @dev used to release funds
     * @param _accountTo eth address to send the withdrawal tokens
     * @param _amount amount of token in
     * @param _tokenIn address of the token in
     * @param _tokenOut address of the token out
     * @param _amountOutMin minimum amount out user want
     * @param _data additional data required for each AMM implementation
     * @param _withdrawData set of data for withdraw
     */
    function withdrawTo(
        address _accountTo,
        uint256 _amount,
        address _tokenIn,
        address _tokenOut,
        uint256 _amountOutMin,
        WithdrawData calldata _withdrawData,
        bytes calldata _data
    ) external;

    /**
     * @dev used by the relayer or by the owner to refund a failed withdrawal request
     * @param _user user's address
     * @param _amount amount to be refunded out of which the transaction cost was substracted
     * @param _receiptToken receipt's address
     * @param _id withdrawal id generated by the relayer.
     */
    function revertLiquidityWithdrawalRequest(
        address _user,
        uint256 _amount,
        address _receiptToken,
        bytes32 _id
    ) external;

    /**
     * @dev transfer ERC20 token to another Mosaic vault.
     * @param _amount amount of tokens to deposit
     * @param _tokenAddress  SC address of the ERC20 token to deposit
     * @param _remoteDestinationAddress SC address of the ERC20 supported tokens in a diff network
     * @param _remoteNetworkID  network ID of remote token
     * @param _maxTransferDelay delay in seconds for the relayer to execute the transaction
     * @param _swapToNative true if a part will be swapped to native token in destination
     * @return transferId - transfer unique identifier
     */
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

    /**
     * @dev transfer ETH to another Mosaic vault
     * @param _remoteDestinationAddress SC address of the ERC20 supported tokens in a diff network
     * @param _remoteNetworkID  network ID of remote token
     * @param _maxTransferDelay delay in seconds for the relayer to execute the transaction
     * @param _swapToNative true if a part will be swapped to native token in destination
     * @return transferId - transfer unique identifier
     */
    function transferETHToLayer(
        address _remoteDestinationAddress,
        uint256 _remoteNetworkID,
        uint256 _maxTransferDelay,
        address _tokenOut,
        uint256 _remoteAmmId,
        uint256 _amountOutMin,
        bool _swapToNative
    ) external payable returns (bytes32 transferId);

    /**
     * @dev called by the relayer or by the owner to refund a failed transfer transaction
     * @param _token address of token user want to withdraw.
     * @param _user user's address.
     * @param _amount amount of tokens to be refunded.
     * @param _originalAmount amount of tokens user initiated a transfer for.
     * @param _id id generated by the relayer.
     * @param _investmentStrategies list of addresses of the investment strategies
     * @param _investmentStrategiesData list of extra data for investment strategies
     */
    function refundTransferFunds(
        address _token,
        address _user,
        uint256 _amount,
        uint256 _originalAmount,
        bytes32 _id,
        address[] calldata _investmentStrategies,
        bytes[] calldata _investmentStrategiesData
    ) external;

    /**
     * @dev used to send random tokens to the holding.
     * @param _token Address of the ERC20 token
     */
    function digestFunds(address _token) external;

    /**
     * @dev used to pause the contract.
     */

    function pause() external;

    /**
     * @dev used to unpause the contract.
     */

    function unpause() external;
}
