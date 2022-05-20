// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

import "../interfaces/IMosaicHolding.sol";
import "../interfaces/IMosaicExchange.sol";
import "../interfaces/IReceiptBase.sol";
import "../interfaces/ITokenFactory.sol";
import "../interfaces/IMosaicVaultConfig.sol";
import "../interfaces/IWETH.sol";
import "../interfaces/IMosaicVault.sol";
import "../interfaces/IInvestmentStrategy.sol";
import "../interfaces/IMosaicNativeSwapper.sol";

import "../libraries/FeeOperations.sol";

//@title: Composable Finance Mosaic ERC20 Vault
contract MosaicVault is
    IMosaicVault,
    OwnableUpgradeable,
    ReentrancyGuardUpgradeable,
    PausableUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;

    struct DepositInfo {
        address token;
        uint256 amount;
    }

    struct TemporaryWithdrawData {
        address tokenIn;
        address remoteTokenIn;
        bytes32 id;
    }

    /// @notice Public mapping to keep track of all the withdrawn funds
    mapping(bytes32 => bool) public hasBeenWithdrawn;

    /// @notice Public mapping to keep track of all the refunded funds
    mapping(bytes32 => bool) public hasBeenRefunded;

    /// @dev mapping userAddress => tokenAddress => activeLiquidityAvailableAfterBlock (block number)
    mapping(address => mapping(address => uint256)) private activeLiquidityAvailableAfterBlock;

    /// @dev mapping userAddress => tokenAddress => passiveLiquidityAvailableAfterTimestamp (block timestamp)
    mapping(address => mapping(address => uint256)) public passiveLiquidityAvailableAfterTimestamp;

    /// @notice Public mapping to track the user deposits
    /// @dev bytes32 => DepositInfo struct (token address, amount)
    mapping(bytes32 => DepositInfo) public deposits;

    /// @notice Store the last withdrawn ID
    bytes32 public lastWithdrawID;

    /// @notice Store the last refunded ID
    bytes32 public lastRefundedID;

    /// @notice Relayer address
    address public relayer;

    IMosaicVaultConfig public override vaultConfig;

    /// @notice Initialize function to set up the contract
    /// @dev it should be called immediately after deploy
    /// @param _mosaicVaultConfig Address of the MosaicVaultConfig
    function initialize(address _mosaicVaultConfig) public initializer {
        __Ownable_init();
        __Pausable_init();
        __ReentrancyGuard_init();

        vaultConfig = IMosaicVaultConfig(_mosaicVaultConfig);
    }

    /// @notice External callable function to set the relayer address
    function setRelayer(address _relayer) external override onlyOwner {
        emit RelayerSet(relayer, _relayer);
        relayer = _relayer;
    }

    /// @notice External callable function to set the vault config address
    function setVaultConfig(address _vaultConfig) external override onlyOwner {
        emit ConfigSet(address(vaultConfig), _vaultConfig);
        vaultConfig = IMosaicVaultConfig(_vaultConfig);
    }

    /// @notice External function used by the user to provide active liquidity to the vault
    ///         User will receive equal amount of IOU tokens
    /// @param _amount Amount of tokens he want to deposit; 0 for ETH
    /// @param _tokenAddress Address of the token he want to deposit; 0x0 for ETH
    /// @param _blocksForActiveLiquidity For how many blocks the liquidity is locked
    function provideActiveLiquidity(
        uint256 _amount,
        address _tokenAddress,
        uint256 _blocksForActiveLiquidity
    )
        public
        payable
        override
        nonReentrant
        whenNotPaused
        inBlockApproveRange(_blocksForActiveLiquidity)
    {
        require(_amount > 0 || msg.value > 0, "ERR: AMOUNT");
        if (msg.value > 0) {
            require(
                vaultConfig.getUnderlyingIOUAddress(vaultConfig.wethAddress()) != address(0),
                "ERR: WETH"
            );
            _provideLiquidity(msg.value, vaultConfig.wethAddress(), _blocksForActiveLiquidity);
        } else {
            require(_tokenAddress != address(0), "ERR: INVALID");
            require(vaultConfig.getUnderlyingIOUAddress(_tokenAddress) != address(0), "ERR: TOKEN");
            _provideLiquidity(_amount, _tokenAddress, _blocksForActiveLiquidity);
        }
    }

    /// @notice External function used by the user to provide passive liquidity to the vault
    ///         User will receive equal amount of Receipt tokens
    /// @param _amount Deposited token's amount; 0 for ETH
    /// @param _tokenAddress Deposited token's address; 0x0 for ETH
    function providePassiveLiquidity(uint256 _amount, address _tokenAddress)
        external
        payable
        override
        nonReentrant
        whenNotPaused
    {
        require(_amount > 0 || msg.value > 0, "ERR: AMOUNT");
        if (msg.value > 0) {
            require(
                vaultConfig.getUnderlyingReceiptAddress(vaultConfig.wethAddress()) != address(0),
                "ERR: WETH "
            );
            _provideLiquidity(msg.value, vaultConfig.wethAddress(), 0);
        } else {
            require(_tokenAddress != address(0), "ERR: INVALID");
            require(
                vaultConfig.getUnderlyingReceiptAddress(_tokenAddress) != address(0),
                "ERR: TOKEN"
            );
            _provideLiquidity(_amount, _tokenAddress, 0);
        }
    }

    /// @dev Internal function called to deposit liquidity, both passive and active liquidity
    /// @param _amount Deposited token's amount; 0 for ETH
    /// @param _tokenAddress Deposited token's address; WETH address for ETH
    /// @param _blocksForActiveLiquidity For how many blocks the liquidity is locked.
    ///                                  Should be 0 if liquidity is passive
    function _provideLiquidity(
        uint256 _amount,
        address _tokenAddress,
        uint256 _blocksForActiveLiquidity
    ) private {
        uint256 finalAmount = _amount;
        //ETH
        if (msg.value > 0) {
            uint256 previousWethAmount = IWETH(_tokenAddress).balanceOf(address(this));
            IWETH(_tokenAddress).deposit{value: msg.value}();
            uint256 currentWethAmount = IWETH(_tokenAddress).balanceOf(address(this));
            finalAmount = currentWethAmount - previousWethAmount;
            require(finalAmount >= msg.value, "ERR: WRAP");

            IERC20Upgradeable(_tokenAddress).safeTransfer(
                vaultConfig.getMosaicHolding(),
                finalAmount
            );
        } else {
            IERC20Upgradeable(_tokenAddress).safeTransferFrom(
                msg.sender,
                vaultConfig.getMosaicHolding(),
                finalAmount
            );
        }
        if (_blocksForActiveLiquidity > 0) {
            //active liquidity
            IReceiptBase(vaultConfig.getUnderlyingIOUAddress(_tokenAddress)).mint(
                msg.sender,
                finalAmount
            );
            _updateActiveLiquidityAvailableAfterBlock(_tokenAddress, _blocksForActiveLiquidity);
            emit DepositActiveLiquidity(
                _tokenAddress,
                msg.sender,
                finalAmount,
                _blocksForActiveLiquidity
            );
        } else {
            //passive liquidity
            IReceiptBase(vaultConfig.getUnderlyingReceiptAddress(_tokenAddress)).mint(
                msg.sender,
                finalAmount
            );
            _updatePassiveLiquidityAvailableAfterTimestamp(_tokenAddress);
            emit DepositPassiveLiquidity(_tokenAddress, msg.sender, finalAmount);
        }
    }

    /// @notice Internal function called to update the availability of the active liquidity for a token and user
    /// @param _token address of the token which availability will be updated
    /// @param _blocksForActiveLiquidity number of blocks the active liquidity will last
    function _updateActiveLiquidityAvailableAfterBlock(
        address _token,
        uint256 _blocksForActiveLiquidity
    ) private {
        uint256 _availableAfter = activeLiquidityAvailableAfterBlock[msg.sender][_token];
        uint256 _newAvailability = block.number + _blocksForActiveLiquidity;
        if (_availableAfter < _newAvailability) {
            activeLiquidityAvailableAfterBlock[msg.sender][_token] = _newAvailability;
        }
    }

    /// @notice Internal function called to update the availability of the passive liquidity for a token and user
    /// @param _token address of the token which availability will be updated
    function _updatePassiveLiquidityAvailableAfterTimestamp(address _token) private {
        uint256 _availableAfter = passiveLiquidityAvailableAfterTimestamp[msg.sender][_token];
        uint256 _newAvailability = block.timestamp + vaultConfig.passiveLiquidityLocktime();
        if (_availableAfter < _newAvailability) {
            passiveLiquidityAvailableAfterTimestamp[msg.sender][_token] = _newAvailability;
        }
    }

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
    ) external override nonReentrant returns (bytes32) {
        require(_amountIn > 0, "ERR: AMOUNT");
        require(paused() == false, "ERR: PAUSED");
        require(vaultConfig.pausedNetwork(_destinationNetworkId) == false, "ERR: PAUSED NETWORK");

        TemporaryWithdrawData memory tempData = _getTemporaryWithdrawData(
            _receiptToken,
            _tokenOut,
            _destinationNetworkId
        );

        require(IReceiptBase(_receiptToken).balanceOf(msg.sender) >= _amountIn, "ERR: BALANCE");
        IReceiptBase(_receiptToken).burn(msg.sender, _amountIn);

        emit WithdrawRequest(
            msg.sender,
            _receiptToken,
            tempData.tokenIn,
            _amountIn,
            _tokenOut,
            tempData.remoteTokenIn,
            _destinationAddress,
            _ammID,
            _destinationNetworkId,
            _data,
            tempData.id,
            _withdrawRequestData
        );

        return tempData.id;
    }

    /// @dev for solving stack too deep error
    /// @param _receiptToken Address of the iou token provider have
    /// @param _tokenOut Address of the token which LP wants to receive
    /// @param _networkId networkId of the _receiptToken's underlying token
    function _getTemporaryWithdrawData(
        address _receiptToken,
        address _tokenOut,
        uint256 _networkId
    ) private validAddress(_tokenOut) returns (TemporaryWithdrawData memory) {
        address tokenIn = IReceiptBase(_receiptToken).underlyingToken();
        address remoteTokenIn;
        // this condition is needed when the withdraw liquidity is requested on another network
        if (_networkId != block.chainid) {
            remoteTokenIn = vaultConfig.remoteTokenAddress(_networkId, tokenIn);
            require(remoteTokenIn != address(0), "ERR: TOKEN");
        } else {
            remoteTokenIn = tokenIn;
        }

        if (vaultConfig.getUnderlyingIOUAddress(tokenIn) == _receiptToken) {
            // active liquidity
            // check if active liquidity is still locked
            require(
                activeLiquidityAvailableAfterBlock[msg.sender][tokenIn] <= block.number,
                "ERR: LIQUIDITY"
            );
        } else if (vaultConfig.getUnderlyingReceiptAddress(tokenIn) == _receiptToken) {
            // passive liquidity
            // check if passive liquidity is still locked
            require(
                passiveLiquidityAvailableAfterTimestamp[msg.sender][tokenIn] <= block.timestamp,
                "ERR: LIQUIDITY"
            );
            // passive liquidity can only be withdrawn in the same token
            require(remoteTokenIn == _tokenOut, "ERR: TOKEN OUT");
        } else {
            revert("ERR: TOKEN NOT WHITELISTED");
        }
        return TemporaryWithdrawData(tokenIn, remoteTokenIn, vaultConfig.generateId());
    }

    /// @notice Called by relayer to withdraw liquidity from the vault
    /// @param _accountTo eth address to send the withdrawal tokens
    /// @param _amount amount requested by user + rewards for token in
    /// @param _requestedAmount amount requested by user for token in
    /// @param _tokenIn address of the token in
    /// @param _tokenOut address of the token out
    /// @param _amountOutMin minimum amount out user wants
    /// @param _withdrawData set of data for withdraw
    /// @param _data additional data required for each AMM implementation
    function withdrawLiquidity(
        address _accountTo,
        uint256 _amount,
        uint256 _requestedAmount,
        address _tokenIn,
        address _tokenOut,
        uint256 _amountOutMin,
        WithdrawData calldata _withdrawData,
        bytes calldata _data
    ) external override {
        uint256 withdrawAmount = _withdraw(
            _accountTo,
            _amount,
            _tokenIn,
            _tokenOut,
            _amountOutMin,
            _withdrawData,
            _data
        );

        emit LiquidityWithdrawn(
            _accountTo,
            _tokenIn,
            _tokenOut,
            _amount,
            _requestedAmount,
            withdrawAmount,
            _withdrawData.baseFee,
            _withdrawData.feePercentage,
            _withdrawData.amountToSwapToNative > 0,
            _withdrawData.id
        );
    }

    /// @notice method called by the relayer to release transfer funds
    /// @param _accountTo eth address to send the withdrawal tokens
    /// @param _amount amount of token in
    /// @param _tokenIn address of the token in
    /// @param _tokenOut address of the token out
    /// @param _amountOutMin minimum amount out user wants
    /// @param _withdrawData set of data for withdraw
    /// @param _data additional data required for each AMM implementation
    function withdrawTo(
        address _accountTo,
        uint256 _amount,
        address _tokenIn,
        address _tokenOut,
        uint256 _amountOutMin,
        WithdrawData calldata _withdrawData,
        bytes calldata _data
    ) external override {
        uint256 withdrawAmount = _withdraw(
            _accountTo,
            _amount,
            _tokenIn,
            _tokenOut,
            _amountOutMin,
            _withdrawData,
            _data
        );

        emit WithdrawalCompleted(
            _accountTo,
            _amount,
            withdrawAmount,
            _tokenOut,
            _withdrawData.id,
            _withdrawData.amountToSwapToNative > 0
        );
    }

    /// @dev internal function called by `withdrawLiquidity` and `withdrawTo`
    /// @param _accountTo eth address to send the withdrawal tokens
    /// @param _amount amount of token in
    /// @param _tokenIn address of the token in
    /// @param _tokenOut address of the token out
    /// @param _amountOutMin minimum amount out user wants
    /// @param _withdrawData set of data for withdraw
    /// @param _data additional data required for each AMM implementation
    function _withdraw(
        address _accountTo,
        uint256 _amount,
        address _tokenIn,
        address _tokenOut,
        uint256 _amountOutMin,
        WithdrawData memory _withdrawData,
        bytes calldata _data
    )
        internal
        onlyWhitelistedToken(_tokenIn)
        validAddress(_tokenOut)
        nonReentrant
        onlyOwnerOrRelayer
        whenNotPaused
        returns (uint256 withdrawAmount)
    {
        IMosaicHolding mosaicHolding = IMosaicHolding(vaultConfig.getMosaicHolding());
        require(hasBeenWithdrawn[_withdrawData.id] == false, "ERR: WITHDRAWN");
        if (_tokenOut == _tokenIn) {
            require(
                mosaicHolding.getTokenLiquidity(_tokenIn, _withdrawData.investmentStrategies) >=
                    _amount,
                "ERR: VAULT BAL"
            );
        }
        if (_withdrawData.amountToSwapToNative > 0) {
            require(vaultConfig.ableToPerformSmallBalanceSwap(), "ERR: UNABLE");
        }
        require(_withdrawData.amountToSwapToNative <= _amount, "ERR: TOO HIGH");
        hasBeenWithdrawn[_withdrawData.id] = true;
        lastWithdrawID = _withdrawData.id;

        withdrawAmount = _takeFees(
            _tokenIn,
            _amount,
            _accountTo,
            _withdrawData.id,
            _withdrawData.baseFee,
            _withdrawData.feePercentage
        );

        mosaicHolding.coverWithdrawRequest(
            _withdrawData.investmentStrategies,
            _withdrawData.investmentStrategiesData,
            _tokenIn,
            withdrawAmount
        );
        if (_withdrawData.amountToSwapToNative > 0) {
            if (_withdrawData.amountToSwapToNative > withdrawAmount) {
                _withdrawData.amountToSwapToNative = withdrawAmount;
            }
            swapToNativeAndTransfer(
                _tokenIn,
                _accountTo,
                _withdrawData.amountToSwapToNative,
                _withdrawData.minAmountOutNative,
                _withdrawData.nativeSwapperId
            );
            withdrawAmount -= _withdrawData.amountToSwapToNative;
        }

        if (_tokenOut != _tokenIn) {
            withdrawAmount = _swap(
                withdrawAmount,
                _amountOutMin,
                _tokenIn,
                _tokenOut,
                _withdrawData.ammId,
                _data
            );
        }

        mosaicHolding.transfer(_tokenOut, _accountTo, withdrawAmount);
    }

    /// @notice Swaps `_amountToNative` from `_tokenIn` to native token and sends it to `_accountTo`
    /// @dev moved to function to avoid Stack too deep
    /// @param _tokenIn address of the token to be swapped
    /// @param _accountTo address which will receive the native tokens
    /// @param _amountToNative amount in `tokenIn` tokens that will be converted to native tokens
    function swapToNativeAndTransfer(
        address _tokenIn,
        address _accountTo,
        uint256 _amountToNative,
        uint256 _minAmountOutNative,
        uint256 _nativeSwapperId
    ) internal {
        address mosaicNativeSwapperAddress = vaultConfig.supportedMosaicNativeSwappers(
            _nativeSwapperId
        );
        require(mosaicNativeSwapperAddress != address(0), "ERR: NOT SET");
        IMosaicNativeSwapper mosaicNativeSwapper = IMosaicNativeSwapper(mosaicNativeSwapperAddress);

        // holding => vault
        IMosaicHolding(vaultConfig.getMosaicHolding()).transfer(
            _tokenIn,
            address(this),
            _amountToNative
        );

        // vault => nativeSwapper
        IERC20Upgradeable(_tokenIn).safeIncreaseAllowance(
            mosaicNativeSwapperAddress,
            _amountToNative
        );
        mosaicNativeSwapper.swapToNative(
            _tokenIn,
            _amountToNative,
            _minAmountOutNative,
            _accountTo,
            ""
        );
    }

    /// @notice in case of liquidity withdrawal request fails, the owner or the relayer calls this method to refund the user with his receipt tokens
    /// @param _user user's address
    /// @param _amount refunded amount which should be a bit smaller than the original one to cover the transaction cost
    /// @param _receiptToken receipt token user had
    /// @param _id request's id
    function revertLiquidityWithdrawalRequest(
        address _user,
        uint256 _amount,
        address _receiptToken,
        bytes32 _id
    ) external override onlyOwnerOrRelayer nonReentrant {
        require(_amount > 0, "ERR: AMOUNT");
        require(hasBeenRefunded[_id] == false, "REFUNDED");

        address tokenAddress = IReceiptBase(_receiptToken).underlyingToken();
        hasBeenRefunded[_id] = true;
        lastRefundedID = _id;

        require(
            tokenAddress != address(0) &&
                (vaultConfig.getUnderlyingReceiptAddress(tokenAddress) != address(0) ||
                    vaultConfig.getUnderlyingIOUAddress(tokenAddress) != address(0)),
            "ERR: TOKEN NOT WHITELISTED"
        );

        if (vaultConfig.getUnderlyingIOUAddress(tokenAddress) == _receiptToken) {
            IReceiptBase(vaultConfig.getUnderlyingIOUAddress(tokenAddress)).mint(_user, _amount);
        } else if (vaultConfig.getUnderlyingReceiptAddress(tokenAddress) == _receiptToken) {
            IReceiptBase(vaultConfig.getUnderlyingReceiptAddress(tokenAddress)).mint(
                _user,
                _amount
            );
        }

        emit LiquidityRefunded(tokenAddress, _receiptToken, _user, _amount, _id);
    }

    /// @dev internal function called to calculate the on-chain fees
    /// @param _token address of the token in
    /// @param _amount amount of token in
    /// @param _accountTo address who will receive the withdrawn liquidity
    /// @param _withdrawRequestId id of the withdraw request
    /// @param _baseFee base fee of the withdraw
    /// @param _feePercentage fee percentage of the withdraw
    function _takeFees(
        address _token,
        uint256 _amount,
        address _accountTo,
        bytes32 _withdrawRequestId,
        uint256 _baseFee,
        uint256 _feePercentage
    ) private returns (uint256) {
        if (_baseFee > 0) {
            IMosaicHolding(vaultConfig.getMosaicHolding()).transfer(_token, relayer, _baseFee);
        }
        uint256 fee = 0;
        if (_feePercentage > 0) {
            require(
                _feePercentage >= vaultConfig.minFee() && _feePercentage <= vaultConfig.maxFee(),
                "ERR: OUT OF RANGE"
            );

            fee = FeeOperations.getFeeAbsolute(_amount, _feePercentage);

            IMosaicHolding(vaultConfig.getMosaicHolding()).transfer(
                _token,
                vaultConfig.getMosaicHolding(),
                fee
            );
        }

        uint256 totalFee = _baseFee + fee;
        require(totalFee < _amount, "ERR: FEE AMOUNT");
        if (totalFee > 0) {
            emit FeeTaken(
                msg.sender,
                _accountTo,
                _token,
                _amount,
                fee,
                _baseFee,
                fee + _baseFee,
                _withdrawRequestId
            );
        }
        return _amount - totalFee;
    }

    /// @dev internal function used to swap tokens
    /// _amountIn amount of the token in
    /// _amountOutMin min amount expected of the token out after the swap
    /// _tokenIn address of the token in
    /// _tokenOut address of the token out
    /// _ammID the amm to use for swapping
    /// _data extra call data for the swap
    function _swap(
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _tokenIn,
        address _tokenOut,
        uint256 _ammID,
        bytes memory _data
    ) private returns (uint256) {
        address mosaicHolding = vaultConfig.getMosaicHolding();
        IMosaicHolding(mosaicHolding).transfer(_tokenIn, address(this), _amountIn);
        address ammAddress = vaultConfig.supportedAMMs(_ammID);
        require(ammAddress != address(0), "ERR: AMM");

        IERC20Upgradeable(_tokenIn).safeApprove(ammAddress, _amountIn);

        uint256 amountToSend = IMosaicExchange(ammAddress).swap(
            _tokenIn,
            _tokenOut,
            _amountIn,
            _amountOutMin,
            _data
        );
        require(amountToSend >= _amountOutMin, "ERR: PRICE");
        IERC20Upgradeable(_tokenOut).safeTransfer(mosaicHolding, amountToSend);
        return amountToSend;
    }

    /// @notice transfer ERC20 token to another Mosaic vault
    /// @param _amount amount of tokens to transfer
    /// @param _tokenAddress SC address of the ERC20 token to transfer
    /// @param _remoteDestinationAddress address that will receive the transfer on destination
    /// @param _remoteNetworkID destination network
    /// @param _maxTransferDelay delay in seconds for the relayer to execute the transaction
    /// @param _tokenOut SC address of the ERC20 token that will be received in the destination network
    /// @param _remoteAmmId id of the AMM that will be used in the destination network
    /// @param _amountOutMin min amount of the token out the user expects to receive
    /// @param _swapToNative true if a part will be swapped to native token in destination
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
    )
        external
        override
        onlyWhitelistedRemoteTokens(_remoteNetworkID, _tokenAddress)
        nonReentrant
        whenNotPausedNetwork(_remoteNetworkID)
        returns (bytes32 transferId)
    {
        require(_amount > 0, "ERR: AMOUNT");

        transferId = vaultConfig.generateId();

        uint256[3] memory ammConfig;
        ammConfig[0] = _remoteAmmId;
        ammConfig[1] = _maxTransferDelay;
        ammConfig[2] = _amountOutMin;

        _transferERC20ToLayer(
            _amount,
            _tokenAddress,
            _remoteDestinationAddress,
            _remoteNetworkID,
            _tokenOut,
            ammConfig,
            transferId,
            _swapToNative
        );
    }

    /// @notice transfer ETH converted to WETH to another Mosaic vault
    /// @param _remoteDestinationAddress address that will receive the transfer on destination
    /// @param _remoteNetworkID destination network
    /// @param _maxTransferDelay delay in seconds for the relayer to execute the transaction
    /// @param _tokenOut SC address of the ERC20 token that will be received in the destination network
    /// @param _remoteAmmId id of the AMM that will be used in the destination network
    /// @param _amountOutMin min amount of the token out the user expects to receive
    /// @param _swapToNative true if a part will be swapped to native token in destination
    function transferETHToLayer(
        address _remoteDestinationAddress,
        uint256 _remoteNetworkID,
        uint256 _maxTransferDelay,
        address _tokenOut,
        uint256 _remoteAmmId,
        uint256 _amountOutMin,
        bool _swapToNative
    )
        external
        payable
        override
        nonReentrant
        whenNotPausedNetwork(_remoteNetworkID)
        returns (bytes32 transferId)
    {
        require(msg.value > 0, "ERR: AMOUNT");
        address weth = vaultConfig.wethAddress();
        require(weth != address(0), "ERR: WETH");
        require(
            vaultConfig.getUnderlyingIOUAddress(weth) != address(0),
            "ERR: WETH NOT WHITELISTED"
        );
        // check if ETH transfer is possible
        require(
            vaultConfig.remoteTokenAddress(_remoteNetworkID, weth) != address(0),
            "ERR: ETH NOT WHITELISTED REMOTE"
        );

        uint256[3] memory ammConfig;
        ammConfig[0] = _remoteAmmId;
        ammConfig[1] = _maxTransferDelay;
        ammConfig[2] = _amountOutMin;

        transferId = vaultConfig.generateId();
        _transferERC20ToLayer(
            msg.value,
            vaultConfig.wethAddress(),
            _remoteDestinationAddress,
            _remoteNetworkID,
            _tokenOut,
            ammConfig,
            transferId,
            _swapToNative
        );
    }

    /// @dev Internal function called to transfer ERC20 to another Mosaic vault
    /// @param _amount amount of tokens to transfer
    /// @param _tokenAddress SC address of the ERC20 token to transfer
    /// @param _remoteDestinationAddress address that will receive the transfer on destination
    /// @param _remoteNetworkID destination network
    /// @param _tokenOut SC address of the ERC20 token that will be received in the destination network
    /// @param _ammConfig => 0 - amm id , 1 - delay, 2, amount out
    /// @param _id id of the transfer
    /// @param _swapToNative true if a part will be swapped to native token in destination
    function _transferERC20ToLayer(
        uint256 _amount,
        address _tokenAddress,
        address _remoteDestinationAddress,
        uint256 _remoteNetworkID,
        address _tokenOut,
        uint256[3] memory _ammConfig, // 0 - amm id , 1 - delay, 2, amount out
        bytes32 _id,
        bool _swapToNative
    ) private inTokenTransferLimits(_tokenAddress, _amount) {
        if (_tokenAddress == vaultConfig.wethAddress()) {
            // convert to WETH
            IWETH(_tokenAddress).deposit{value: _amount}();
            // transfer funds to holding
            IERC20Upgradeable(_tokenAddress).safeTransfer(vaultConfig.getMosaicHolding(), _amount);
        } else {
            // transfer funds to holding
            IERC20Upgradeable(_tokenAddress).safeTransferFrom(
                msg.sender,
                vaultConfig.getMosaicHolding(),
                _amount
            );
        }

        deposits[_id] = DepositInfo({token: _tokenAddress, amount: _amount});

        // NOTE: _tokenOut == address(0) is reserved for
        //       the native token of the destination layer
        //       for eg: MATIC for Polygon
        emit TransferInitiated(
            msg.sender,
            _tokenAddress,
            vaultConfig.remoteTokenAddress(_remoteNetworkID, _tokenAddress),
            _remoteNetworkID,
            _amount,
            _remoteDestinationAddress,
            _id,
            _ammConfig[1],
            _tokenOut,
            _ammConfig[0],
            _ammConfig[2],
            _swapToNative
        );
    }

    /// @notice called by the owner of the contract or by the relayer to return funds back to the user in case of a failed transfer
    ///         This method will mark the `id` as used and emit the event that funds have been refunded
    /// @param _token address of the ERC20 token
    /// @param _user address of the user that will receive the tokens
    /// @param _amount amount of tokens to be refunded
    /// @param _originalAmount amount of tokens transfered in the first place
    /// @param _id id of the original transfer
    /// @param _investmentStrategies list of addresses of the investment strategies
    /// @param _investmentStrategiesData list of extra data for investment strategies
    function refundTransferFunds(
        address _token,
        address _user,
        uint256 _amount,
        uint256 _originalAmount,
        bytes32 _id,
        address[] calldata _investmentStrategies,
        bytes[] calldata _investmentStrategiesData
    ) external override nonReentrant onlyOwnerOrRelayer {
        // should not be refunded
        require(hasBeenRefunded[_id] == false, "ERR: REFUNDED");

        // check if the vault has enough locked balance
        require(
            IMosaicHolding(vaultConfig.getMosaicHolding()).getTokenLiquidity(
                _token,
                _investmentStrategies
            ) >= _amount,
            "ERR: BAL"
        );

        // check if the deposit data matches
        require(
            deposits[_id].token == _token && deposits[_id].amount == _originalAmount,
            "ERR: DEPOSIT"
        );

        hasBeenRefunded[_id] = true;
        lastRefundedID = _id;

        IMosaicHolding mosaicHolding = IMosaicHolding(vaultConfig.getMosaicHolding());
        mosaicHolding.coverWithdrawRequest(
            _investmentStrategies,
            _investmentStrategiesData,
            _token,
            _amount
        );
        mosaicHolding.transfer(_token, _user, _amount);

        delete deposits[_id];

        emit TransferFundsRefunded(_token, _user, _amount, _originalAmount, _id);
    }

    /**
     * @notice Used to transfer randomly sent tokens to this contract to the Mosaic holding
     * @param _token Token's address
     */
    function digestFunds(address _token) external override onlyOwner validAddress(_token) {
        uint256 balance = IERC20Upgradeable(_token).balanceOf(address(this));
        require(balance > 0, "ERR: BAL");
        IERC20Upgradeable(_token).safeTransfer(vaultConfig.getMosaicHolding(), balance);
        emit FundsDigested(_token, balance);
    }

    /// @notice External payable function called when ether is sent to the contract
    ///         Receiving ether is considered an active liquidity
    receive() external payable {
        provideActiveLiquidity(0, address(0), vaultConfig.maxLimitLiquidityBlocks());
    }

    /// @notice External callable function to pause the contract
    function pause() external override whenNotPaused onlyOwner {
        _pause();
    }

    /// @notice External callable function to unpause the contract
    function unpause() external override whenPaused onlyOwner {
        _unpause();
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "ERR: INVALID ADDRESS");
        _;
    }

    modifier onlyWhitelistedToken(address _tokenAddress) {
        require(
            vaultConfig.getUnderlyingIOUAddress(_tokenAddress) != address(0),
            "ERR: TOKEN NOT WHITELISTED"
        );
        _;
    }

    modifier onlyWhitelistedRemoteTokens(uint256 _networkID, address _tokenAddress) {
        require(
            vaultConfig.remoteTokenAddress(_networkID, _tokenAddress) != address(0),
            "ERR: TOKEN NOT WHITELISTED DESTINATION"
        );
        _;
    }

    modifier whenNotPausedNetwork(uint256 _networkID) {
        require(paused() == false, "ERR: PAUSED");
        require(vaultConfig.pausedNetwork(_networkID) == false, "ERR: PAUSED NETWORK");
        _;
    }

    modifier onlyOwnerOrRelayer() {
        require(_msgSender() == owner() || _msgSender() == relayer, "ERR: PERMISSIONS");
        _;
    }

    modifier inTokenTransferLimits(address _token, uint256 _amount) {
        require(vaultConfig.inTokenTransferLimits(_token, _amount), "ERR: TRANSFER LIMITS");
        _;
    }

    modifier inBlockApproveRange(uint256 _blocksForActiveLiquidity) {
        require(
            _blocksForActiveLiquidity >= vaultConfig.minLimitLiquidityBlocks() &&
                _blocksForActiveLiquidity <= vaultConfig.maxLimitLiquidityBlocks(),
            "ERR: BLOCK RANGE"
        );
        _;
    }
}
