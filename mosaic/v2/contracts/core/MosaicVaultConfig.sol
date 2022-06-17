// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "../interfaces/IMosaicVaultConfig.sol";
import "../interfaces/IMosaicHolding.sol";
import "../interfaces/ITokenFactory.sol";
import "../libraries/FeeOperations.sol";
import "./VaultConfigBase.sol";

contract MosaicVaultConfig is IMosaicVaultConfig, VaultConfigBase {
    /// @notice ratio for token transfers. 1000 -> 1:1 transfer
    uint256 public constant TOKEN_RATIO = 1000;
    uint256 private nonce;
    string private constant ACTIVE_TOKEN_NAME = "IOU-";
    string private constant PASSIVE_TOKEN_NAME = "R-";

    /// @notice lock time for passive liquidity
    uint256 public override passiveLiquidityLocktime;
    /// @notice min fee per transfer
    uint256 public override minFee;
    /// @notice max fee per transfer
    uint256 public override maxFee;
    /// @notice minimum number of blocks to provide liquidity
    uint256 public override minLimitLiquidityBlocks;
    /// @notice maximum number of blocks to provide liquidity
    uint256 public override maxLimitLiquidityBlocks;

    /// @notice Address of ERC20 wrap eth
    address public override wethAddress;

    /// @notice Store address of a token in another network
    // @dev remoteTokenAddress[networkID][addressHere] = addressThere
    mapping(uint256 => mapping(address => address)) public override remoteTokenAddress;
    /// @notice Ratio of token in another network
    mapping(uint256 => mapping(address => uint256)) public override remoteTokenRatio;

    /*
    UNISWAP = 2
    SUSHISWAP = 3
    CURVE = 4
    */
    mapping(uint256 => address) public override supportedAMMs;

    /// @notice Public function to query the whitelisted tokens list
    /// @dev token address => WhitelistedToken struct
    mapping(address => WhitelistedToken) public whitelistedTokens;

    /// @notice Public reference to the paused networks
    mapping(uint256 => bool) public override pausedNetwork;

    /// @notice flag that indicates if part of a Mosaic transfer or liquidity withdrawal
    ///         can be swapped into the native token of the destination network
    bool public override ableToPerformSmallBalanceSwap;

    /// @notice Public reference to the addresses of the NativeSwapper contracts used to swap
    ///         a part of a transfer or liquidity withdrawal to native token
    mapping(uint256 => address) public override supportedMosaicNativeSwappers;

    /// @notice Initialize function to set up the contract
    /// @dev it should be called immediately after deploy
    /// @param _mosaicHolding Address of the MosaicHolding contract
    function initialize(address _mosaicHolding) public initializer {
        require(_mosaicHolding != address(0), "ERR: HOLDING ADDRESS");

        __Ownable_init();

        nonce = 0;
        // 0%
        minFee = 0;
        // 5%
        maxFee = 500;
        // 1 day
        minLimitLiquidityBlocks = 1;
        maxLimitLiquidityBlocks = 100;
        passiveLiquidityLocktime = 1 days;
        mosaicHolding = IMosaicHolding(_mosaicHolding);
    }

    /// @notice sets the lock time for passive liquidity
    /// @param _locktime new lock time for passive liquidity
    function setPassiveLiquidityLocktime(uint256 _locktime) external override onlyOwner {
        passiveLiquidityLocktime = _locktime;
    }

    /// @notice External function called by owner to set the ERC20 WETH address
    /// @param _weth address of the WETH
    /// @param _minTransferAmount min amount of ETH that can be transfered
    /// @param _maxTransferAmount max amount of ETH that can be transfered
    function setWethAddress(
        address _weth,
        uint256 _minTransferAmount,
        uint256 _maxTransferAmount
    ) external override onlyOwner validAddress(_weth) {
        if (wethAddress != address(0)) {
            _removeWhitelistedToken(wethAddress);
        }

        _addWhitelistedToken(_weth, _minTransferAmount, _maxTransferAmount);
        wethAddress = _weth;
    }

    /// @notice Get IOU address of an ERC20 token
    /// @param _token address of the token whose underlying IOU we are requesting
    function getUnderlyingIOUAddress(address _token) external view override returns (address) {
        return whitelistedTokens[_token].underlyingIOUAddress;
    }

    /// @notice Get Receipt address of an ERC20 token
    /// @param _token address of the token whose underlying Receipt we are requesting
    function getUnderlyingReceiptAddress(address _token) external view override returns (address) {
        return whitelistedTokens[_token].underlyingReceiptAddress;
    }

    /// @notice Public function to add address of the AMM used to swap tokens
    /// @param _ammID the integer constant for the AMM
    /// @param _ammAddress Address of the AMM
    /// @dev AMM should be a wrapper created by us over the AMM implementation
    function addSupportedAMM(uint256 _ammID, address _ammAddress)
        public
        override
        onlyOwner
        validAddress(_ammAddress)
    {
        supportedAMMs[_ammID] = _ammAddress;
        emit AMMAdded(_ammID, _ammAddress);
    }

    /// @notice Public function to remove address of the AMM
    /// @param _ammID the integer constant for the AMM
    function removeSupportedAMM(uint256 _ammID) public override onlyOwner {
        delete supportedAMMs[_ammID];
        emit AMMRemoved(_ammID);
    }

    /// @notice External function called by the owner in order to change remote token ration
    /// @param _tokenAddress Address of the token in this network
    /// @param _remoteNetworkID Network Id
    /// @param _remoteTokenRatio New token ratio
    function changeRemoteTokenRatio(
        address _tokenAddress,
        uint256 _remoteNetworkID,
        uint256 _remoteTokenRatio
    ) external override onlyOwner validAmount(remoteTokenRatio[_remoteNetworkID][_tokenAddress]) {
        remoteTokenRatio[_remoteNetworkID][_tokenAddress] = _remoteTokenRatio;
    }

    /// @notice Adds a whitelisted token to the contract, allowing for anyone to deposit their tokens.
    /// @param _tokenAddress SC address of the ERC20 token to add to whitelisted tokens
    /// @param _minTransferAmount min amount of tokens that can be transfered
    /// @param _maxTransferAmount max amount of tokens that can be transfered
    function addWhitelistedToken(
        address _tokenAddress,
        uint256 _minTransferAmount,
        uint256 _maxTransferAmount
    ) external override onlyOwner validAddress(_tokenAddress) {
        _addWhitelistedToken(_tokenAddress, _minTransferAmount, _maxTransferAmount);
    }

    /// @notice Updates a whitelisted token limit.
    /// @param _tokenAddress SC address of the ERC20 token to add to whitelisted tokens
    /// @param _minTransferAmount min amount of tokens that can be transfered
    function updateWhitelistedTokenMinLimit(address _tokenAddress, uint256 _minTransferAmount)
        external
        onlyOwner
        validAddress(_tokenAddress)
    {
        require(
            whitelistedTokens[_tokenAddress].underlyingIOUAddress != address(0),
            "ERR: NOT WHITELISTED"
        );
        require(
            whitelistedTokens[_tokenAddress].maxTransferAllowed > _minTransferAmount,
            "ERR: MAX > MIN"
        );
        whitelistedTokens[_tokenAddress].minTransferAllowed = _minTransferAmount;
    }

    /// @notice Updates a whitelisted token limit.
    /// @param _tokenAddress SC address of the ERC20 token to add to whitelisted tokens
    /// @param _maxTransferAmount max amount of tokens that can be transfered
    function updateWhitelistedTokenMaxLimit(address _tokenAddress, uint256 _maxTransferAmount)
        external
        onlyOwner
        validAddress(_tokenAddress)
    {
        require(
            whitelistedTokens[_tokenAddress].underlyingIOUAddress != address(0),
            "ERR: NOT WHITELISTED"
        );

        require(
            _maxTransferAmount > whitelistedTokens[_tokenAddress].minTransferAllowed,
            "ERR: MAX > MIN"
        );
        whitelistedTokens[_tokenAddress].maxTransferAllowed = _maxTransferAmount;
    }

    /// @notice Internal function that adds a whitelisted token to the contract, allowing for anyone to deposit their tokens
    /// @param _tokenAddress SC address of the ERC20 token to add to whitelisted tokens
    /// @param _minTransferAmount min amount of tokens that can be transfered
    /// @param _maxTransferAmount max amount of tokens that can be transfered
    function _addWhitelistedToken(
        address _tokenAddress,
        uint256 _minTransferAmount,
        uint256 _maxTransferAmount
    ) private nonReentrant {
        require(_maxTransferAmount > _minTransferAmount, "ERR: MAX > MIN");

        require(
            whitelistedTokens[_tokenAddress].underlyingIOUAddress == address(0),
            "ERR: ALREADY WHITELISTED"
        );

        (address newIou, address newReceipt) = _deployLiquidityTokens(_tokenAddress);

        whitelistedTokens[_tokenAddress].minTransferAllowed = _minTransferAmount;

        whitelistedTokens[_tokenAddress].maxTransferAllowed = _maxTransferAmount;

        emit TokenWhitelisted(_tokenAddress, newIou, newReceipt);
    }

    /// @dev Private function called when deploy a receipt IOU token based on already deployed ERC20 token
    /// @param _underlyingToken address of the underlying token
    function _deployLiquidityTokens(address _underlyingToken) private returns (address, address) {
        require(address(tokenFactory) != address(0), "ERR: FACTORY INIT");
        require(address(vault) != address(0), "ERR: VAULT INIT");

        address newIou = tokenFactory.createIOU(_underlyingToken, ACTIVE_TOKEN_NAME, vault);

        address newReceipt = tokenFactory.createReceipt(
            _underlyingToken,
            PASSIVE_TOKEN_NAME,
            vault
        );

        whitelistedTokens[_underlyingToken].underlyingIOUAddress = newIou;
        whitelistedTokens[_underlyingToken].underlyingReceiptAddress = newReceipt;

        emit TokenCreated(_underlyingToken);
        return (newIou, newReceipt);
    }

    /// @notice removes whitelisted token from the contract, avoiding new deposits and withdrawals.
    /// @param _tokenAddress SC address of the ERC20 token to remove from whitelisted tokens
    function removeWhitelistedToken(address _tokenAddress) external override onlyOwner {
        _removeWhitelistedToken(_tokenAddress);
    }

    /// @notice private function that removes whitelisted token from the contract, avoiding new deposits and withdrawals.
    /// @param _tokenAddress SC address of the ERC20 token to remove from whitelisted tokens
    function _removeWhitelistedToken(address _tokenAddress) private {
        require(
            whitelistedTokens[_tokenAddress].underlyingIOUAddress != address(0),
            "ERR: NOT WHITELISTED"
        );
        emit TokenWhitelistRemoved(_tokenAddress);
        delete whitelistedTokens[_tokenAddress];
    }

    /// @notice External function called by the owner to add whitelisted token in network
    /// @param _tokenAddress Address of the token in this network
    /// @param _tokenAddressRemote Address of the token in destination network
    /// @param _remoteNetworkID Network Id
    /// @param _remoteTokenRatio New token ratio
    function addTokenInNetwork(
        address _tokenAddress,
        address _tokenAddressRemote,
        uint256 _remoteNetworkID,
        uint256 _remoteTokenRatio
    )
        external
        override
        onlyOwner
        validAddress(_tokenAddress)
        validAddress(_tokenAddressRemote)
        notZero(_remoteNetworkID)
    {
        require(
            whitelistedTokens[_tokenAddress].underlyingIOUAddress != address(0),
            "ERR: NOT WHITELISTED"
        );

        remoteTokenAddress[_remoteNetworkID][_tokenAddress] = _tokenAddressRemote;
        remoteTokenRatio[_remoteNetworkID][_tokenAddress] = _remoteTokenRatio;

        emit RemoteTokenAdded(
            _tokenAddress,
            _tokenAddressRemote,
            _remoteNetworkID,
            _remoteTokenRatio
        );
    }

    /// @notice Called only by the owner to remove whitelisted token from remote network
    /// @param _tokenAddress address of the token to remove
    /// @param _remoteNetworkID id of the remote network
    function removeTokenInNetwork(address _tokenAddress, uint256 _remoteNetworkID)
        external
        override
        onlyOwner
        notZero(_remoteNetworkID)
        validAddress(_tokenAddress)
    {
        require(
            remoteTokenAddress[_remoteNetworkID][_tokenAddress] != address(0),
            "ERR: NOT WHITELISTED NETWORK"
        );

        delete remoteTokenAddress[_remoteNetworkID][_tokenAddress];
        delete remoteTokenRatio[_remoteNetworkID][_tokenAddress];

        emit RemoteTokenRemoved(_tokenAddress, _remoteNetworkID);
    }

    /// @notice Updates the minimum fee
    /// @param _newMinFee new minimum fee value
    function setMinFee(uint256 _newMinFee) external override onlyOwner {
        require(_newMinFee < FeeOperations.FEE_FACTOR, "ERR: MIN > FACTOR");
        require(_newMinFee < maxFee, "ERR: MIN > MAX");

        minFee = _newMinFee;
        emit MinFeeChanged(_newMinFee);
    }

    /// @notice Updates the maximum fee
    /// @param _newMaxFee new maximum fee value
    function setMaxFee(uint256 _newMaxFee) external override onlyOwner {
        require(_newMaxFee < FeeOperations.FEE_FACTOR, "ERR: MAX > FACTOR");
        require(_newMaxFee > minFee, "ERR: MIN > MAX");

        maxFee = _newMaxFee;
        emit MaxFeeChanged(_newMaxFee);
    }

    /// @notice Updates the minimum limit liquidity block
    /// @param _newMinLimitLiquidityBlocks new minimum limit liquidity block value
    function setMinLimitLiquidityBlocks(uint256 _newMinLimitLiquidityBlocks)
        external
        override
        onlyOwner
    {
        require(_newMinLimitLiquidityBlocks < maxLimitLiquidityBlocks, "ERR: MIN > MAX");

        minLimitLiquidityBlocks = _newMinLimitLiquidityBlocks;
        emit MinLiquidityBlockChanged(_newMinLimitLiquidityBlocks);
    }

    /// @notice Updates the maximum limit liquidity block
    /// @param _newMaxLimitLiquidityBlocks new maximum limit liquidity block value
    function setMaxLimitLiquidityBlocks(uint256 _newMaxLimitLiquidityBlocks)
        external
        override
        onlyOwner
    {
        require(_newMaxLimitLiquidityBlocks > minLimitLiquidityBlocks, "ERR: MIN > MAX");

        maxLimitLiquidityBlocks = _newMaxLimitLiquidityBlocks;
        emit MaxLiquidityBlockChanged(_newMaxLimitLiquidityBlocks);
    }

    /// @notice External function called by the vault to generate new ID
    /// @dev Nonce variable is incremented on each call
    function generateId() external override onlyVault(msg.sender) returns (bytes32) {
        nonce = nonce + 1;
        return keccak256(abi.encodePacked(block.chainid, block.number, vault, nonce));
    }

    /// @notice Check if amount is in token transfer limits
    function inTokenTransferLimits(address _token, uint256 _amount)
        external
        view
        override
        returns (bool)
    {
        return (whitelistedTokens[_token].minTransferAllowed <= _amount &&
            whitelistedTokens[_token].maxTransferAllowed >= _amount);
    }

    /// @notice External callable function to pause the contract
    function pauseNetwork(uint256 _networkID) external override onlyOwner {
        pausedNetwork[_networkID] = true;
        emit PauseNetwork(msg.sender, _networkID);
    }

    /// @notice External callable function to unpause the contract
    function unpauseNetwork(uint256 _networkID) external override onlyOwner {
        pausedNetwork[_networkID] = false;
        emit UnpauseNetwork(msg.sender, _networkID);
    }

    /// @notice Sets the value of the flag that controls if part of a Mosaic transfer
    ///         can be swapped into the native token of the destination network
    function setAbleToPerformSmallBalanceSwap(bool _flag) external override onlyOwner {
        ableToPerformSmallBalanceSwap = _flag;
    }

    /// @notice Public function to add address of the MosaicNativeSwapper used to swap into native tokens
    /// @param _mosaicNativeSwapperID the integer constant for the MosaicNativeSwapper
    /// @param _mosaicNativeSwapperAddress Address of the MosaicNativeSwapper
    function addSupportedMosaicNativeSwapper(
        uint256 _mosaicNativeSwapperID,
        address _mosaicNativeSwapperAddress
    ) public override onlyOwner validAddress(_mosaicNativeSwapperAddress) {
        supportedMosaicNativeSwappers[_mosaicNativeSwapperID] = _mosaicNativeSwapperAddress;
        ableToPerformSmallBalanceSwap = true;
    }

    /// @notice Public function to remove address of the MosaicNativeSwapper
    /// @param _mosaicNativeSwapperID the integer constant for the MosaicNativeSwapper
    function removeSupportedMosaicNativeSwapper(uint256 _mosaicNativeSwapperID)
        public
        override
        onlyOwner
    {
        delete supportedMosaicNativeSwappers[_mosaicNativeSwapperID];
    }

    modifier onlyOwnerOrVault(address _address) {
        require(_address == owner() || _address == vault, "ERR: PERMISSIONS O-V");
        _;
    }

    modifier onlyVault(address _address) {
        require(_address == vault, "ERR: PERMISSIONS VAULT");
        _;
    }

    modifier onlyWhitelistedRemoteTokens(uint256 _networkID, address _tokenAddress) {
        require(
            whitelistedTokens[_tokenAddress].underlyingIOUAddress != address(0),
            "ERR: NOT WHITELISTED"
        );
        require(
            remoteTokenAddress[_networkID][_tokenAddress] != address(0),
            "ERR: NOT WHITELISTED NETWORK"
        );
        _;
    }

    modifier notZero(uint256 _value) {
        require(_value > 0, "ERR: ZERO");
        _;
    }
}
