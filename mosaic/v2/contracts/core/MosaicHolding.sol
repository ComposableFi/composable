// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

import "@openzeppelin/contracts/interfaces/IERC20.sol";
import "../interfaces/IMosaicHolding.sol";

/// @title MosaicHolding
/// @notice Mosaic contract that holds all the liquidity
contract MosaicHolding is
    IMosaicHolding,
    ReentrancyGuardUpgradeable,
    AccessControlEnumerableUpgradeable,
    PausableUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;
    /// @notice Role name for the MosaicVault
    bytes32 public constant MOSAIC_VAULT = keccak256("MOSAIC_VAULT");
    /// @notice Role name for the rebalancing bot
    bytes32 public constant REBALANCING_BOT = keccak256("REBALANCING_BOT");

    /// @notice Address of the token funds should be saved from
    address public tokenAddressToSaveFunds;
    /// @notice Address of the EOA where founds should be saved
    address public userAddressToSaveFundsTo;
    /// @notice Funds can be saved until this timer. Is defined relative to block.timestamp
    uint256 public saveFundsTimer;
    /// @notice Variable that define how long admin is allowed to save funds
    uint256 public override saveFundsLockupTime;
    /// @notice New save funds lockup time value
    uint256 public override newSaveFundsLockUpTime;
    /// @notice Save token duration
    uint256 public override durationToChangeTimer;

    /// @notice Public mapping to track the investment strategies available
    /// @dev address => bool
    mapping(address => bool) public investmentStrategies;
    /// @notice Public mapping to track the rebalancing interval per token
    mapping(address => uint256) public rebalancingThresholds;

    // solhint-disable-next-line no-empty-blocks
    receive() external payable {}

    /// @notice Initialize function to set up the contract
    /// @dev it should be called immediately after deploy
    /// @param _admin Address of the contract admin
    function initialize(address _admin) public initializer {
        __ReentrancyGuard_init();
        __Pausable_init();
        __AccessControl_init();

        _setupRole(DEFAULT_ADMIN_ROLE, _admin);
        _setRoleAdmin(MOSAIC_VAULT, DEFAULT_ADMIN_ROLE);

        saveFundsLockupTime = 12 hours;
    }

    /// @notice External function to return available liquidity
    /// @param _token token address for which we want query liquidity for
    /// @param _investmentStrategies array of investments strategies to query for liquidity
    function getTokenLiquidity(address _token, address[] calldata _investmentStrategies)
        external
        view
        override
        returns (uint256)
    {
        uint256 sum = IERC20(_token).balanceOf(address(this));
        if (_investmentStrategies.length > 0) {
            for (uint256 i = 0; i < _investmentStrategies.length; i++) {
                sum = sum + IInvestmentStrategy(_investmentStrategies[i]).investmentAmount(_token);
            }
        }
        return sum;
    }

    /// @notice External function used by admin of the contract to define rebalancing max amounts per token
    /// @param _token token address for which we want to set the threshold
    /// @param _amount the max rebalancing amount for that token (max withdrawable amount)
    function setRebalancingThreshold(address _token, uint256 _amount)
        external
        validAddress(_token)
        validAmount(_amount)
        onlyAdmin
    {
        emit RebalancingThresholdChanged(
            msg.sender,
            _token,
            rebalancingThresholds[_token],
            _amount
        );

        rebalancingThresholds[_token] = _amount;
    }

    /// @notice External function used by rebalancing bots to extract liquidity that will be transferred to another layer
    /// @param _token token address for which we want to rebalance
    /// @param _amount the amount that's being extracted
    /// @param _receiver receiver address; has to be whitelisted
    function extractLiquidityForRebalancing(
        address _token,
        uint256 _amount,
        address _receiver
    ) external validAddress(_token) validAmount(_amount) onlyRebalancingOrAdmin {
        require(_amount <= rebalancingThresholds[_token], "ERR: AMOUNT");
        require(IERC20Upgradeable(_token).balanceOf(address(this)) >= _amount, "ERR: BALANCE");

        IERC20Upgradeable(_token).safeTransfer(_receiver, _amount);

        emit RebalancingInitiated(msg.sender, _token, _receiver, _amount);
    }

    /// @notice External function used by admin of the contract to add uniq role address
    /// @param _role role of the actor
    /// @param _actor address of the actor
    function setUniqRole(bytes32 _role, address _actor)
        external
        override
        validAddress(_actor)
        onlyAdmin
    {
        uint256 rolesCount = getRoleMemberCount(_role);
        for (uint256 i = 0; i < rolesCount; i++) {
            address _oldRoleAddress = getRoleMember(_role, 0);
            revokeRole(_role, _oldRoleAddress);
        }
        grantRole(_role, _actor);
    }

    // @notice External function to transfer tokens by the vault or by admins
    // @param _token ERC20 token address
    // @param _receiver Address of the receiver, vault or EOA
    // @param _amount Amount to transfer
    function transfer(
        address _token,
        address _receiver,
        uint256 _amount
    ) public override validAddress(_token) validAddress(_receiver) onlyVaultOrAdmin whenNotPaused {
        require(IERC20Upgradeable(_token).balanceOf(address(this)) >= _amount, "ERR: BALANCE");
        IERC20Upgradeable(_token).safeTransfer(_receiver, _amount);
    }

    // @notice External function to transfer ETH by the vault or by admins
    // @param _receiver Address of the receiver, vault or EOA
    // @param _amount Amount to transfer
    function transferETH(address _receiver, uint256 _amount)
        public
        override
        validAddress(_receiver)
        onlyVaultOrAdmin
        whenNotPaused
    {
        require(address(this).balance >= _amount, "ERR: BALANCE");
        // solhint-disable-next-line avoid-low-level-calls
        (bool sent, ) = payable(_receiver).call{value: _amount}("");
        require(sent, "ERR: TRANSFER ETH FAILED");
        emit ETHTransfered(_receiver, _amount);
    }

    /// @notice External function called in order to allow other party to spend from this contract
    /// @param _spender Address of the spender
    /// @param _token Address of the ERC20 token
    /// @param _amount Amount allow to spend
    function approve(
        address _spender,
        address _token,
        uint256 _amount
    ) external override whenNotPaused validAddress(_spender) onlyVaultOrAdmin {
        IERC20Upgradeable(_token).safeApprove(_spender, _amount);
    }

    /// @notice External function called only by the admin to add IInvestmentStrategy supported contracts
    /// @param _strategyAddress IInvestmentStrategy contract address
    function addInvestmentStrategy(address _strategyAddress)
        external
        onlyAdmin
        validAddress(_strategyAddress)
    {
        require(investmentStrategies[_strategyAddress] == false, "ERR: ALREADY ADDED");
        investmentStrategies[_strategyAddress] = true;
    }

    /// @notice External function called by the admin to invest founds in one of the IInvestmentStrategy from the contract
    /// @param _investments Array of Investment struct (token address, amount)
    /// @param _investmentStrategy Address of the IInvestmentStrategy admin want to use
    /// @param _data dynamic data that strategy required
    function invest(
        IInvestmentStrategy.Investment[] calldata _investments,
        address _investmentStrategy,
        bytes calldata _data
    ) external override onlyAdmin validAddress(_investmentStrategy) {
        require(investmentStrategies[_investmentStrategy], "ERR: STRATEGY NOT SET");
        uint256 investmentsLength = _investments.length;
        address contractAddress = address(this);
        for (uint256 i; i < investmentsLength; i++) {
            IInvestmentStrategy.Investment memory investment = _investments[i];
            require(investment.amount != 0 && investment.token != address(0), "ERR: TOKEN AMOUNT");
            IERC20Upgradeable token = IERC20Upgradeable(investment.token);
            require(token.balanceOf(contractAddress) >= investment.amount, "ERR: BALANCE");
            token.safeApprove(_investmentStrategy, investment.amount);
        }

        uint256 mintedTokens = IInvestmentStrategy(_investmentStrategy).makeInvestment(
            _investments,
            _data
        );
        emit FoundsInvested(_investmentStrategy, msg.sender, mintedTokens);
    }

    /// @notice withdraws tokens from investments to have available liquidity
    /// @param _investmentStrategies list of addresses of the investment strategies
    /// @param _data investment strategies extra data
    /// @param _token address of the token to withdraw
    /// @param _amount amount of tokens desired to withdraw
    function coverWithdrawRequest(
        address[] calldata _investmentStrategies,
        bytes[] calldata _data,
        address _token,
        uint256 _amount
    ) external override {
        require(hasRole(MOSAIC_VAULT, msg.sender), "ERR: PERMISSIONS A-V");
        uint256 balance = IERC20(_token).balanceOf(address(this));
        if (balance >= _amount) return;

        uint256 requiredAmount = _amount - balance;
        uint8 index;
        while (requiredAmount > 0) {
            address strategy = _investmentStrategies[index];
            IInvestmentStrategy investment = IInvestmentStrategy(strategy);
            uint256 investmentAmount = investment.investmentAmount(_token);
            uint256 amountToWithdraw = 0;
            if (investmentAmount >= requiredAmount) {
                amountToWithdraw = requiredAmount;
                requiredAmount = 0;
            } else {
                amountToWithdraw = investmentAmount;
                requiredAmount = requiredAmount - investmentAmount;
            }
            IInvestmentStrategy.Investment[]
                memory investments = new IInvestmentStrategy.Investment[](1);
            investments[0] = IInvestmentStrategy.Investment(_token, amountToWithdraw);
            IInvestmentStrategy(investment).withdrawInvestment(investments, _data[index]);

            emit InvestmentWithdrawn(strategy, msg.sender);

            index++;
        }

        require(IERC20(_token).balanceOf(address(this)) >= _amount, "ERR: VAULT BAL");
    }

    /// @notice External function called by the admin to withdraw investment
    /// @param _investments Array of Investment struct (token address, amount)
    /// @param _investmentStrategy address of the strategy
    /// @param _data dynamic data that strategy required
    function withdrawInvestment(
        IInvestmentStrategy.Investment[] calldata _investments,
        address _investmentStrategy,
        bytes calldata _data
    ) external override onlyVaultOrAdmin validAddress(_investmentStrategy) {
        require(investmentStrategies[_investmentStrategy], "ERR: STRATEGY NOT SET");
        IInvestmentStrategy(_investmentStrategy).withdrawInvestment(_investments, _data);
        emit InvestmentWithdrawn(_investmentStrategy, msg.sender);
    }

    /// @notice External function used to claim tokens that different DAO issues for the investors
    /// @param _investmentStrategy address of the strategy
    /// @param _data dynamic data that strategy required
    function claim(address _investmentStrategy, bytes calldata _data)
        external
        override
        onlyAdmin
        validAddress(_investmentStrategy)
    {
        require(investmentStrategies[_investmentStrategy], "ERR: STRATEGY NOT SET");
        address rewardTokenAddress = IInvestmentStrategy(_investmentStrategy).claimTokens(_data);
        emit TokenClaimed(_investmentStrategy, rewardTokenAddress);
    }

    /**
     * @notice Starts save funds transfer
     * @param _token Token's balance the owner wants to withdraw
     * @param _to Receiver address
     */
    function startSaveFunds(address _token, address _to)
        external
        override
        onlyAdmin
        whenPaused
        validAddress(_token)
        validAddress(_to)
    {
        tokenAddressToSaveFunds = _token;
        userAddressToSaveFundsTo = _to;

        saveFundsTimer = block.timestamp + saveFundsLockupTime;

        emit SaveFundsStarted(msg.sender, _token, _to);
    }

    /**
     * @notice Will be called once the contract is paused and token's available liquidity will be manually moved
     */
    function executeSaveFunds() external override onlyAdmin nonReentrant {
        require(saveFundsTimer <= block.timestamp && saveFundsTimer != 0, "ERR: TIMELOCK");

        uint256 balance = IERC20Upgradeable(tokenAddressToSaveFunds).balanceOf(address(this));
        if (balance > 0) {
            IERC20Upgradeable(tokenAddressToSaveFunds).safeTransfer(
                userAddressToSaveFundsTo,
                balance
            );
            emit LiquidityMoved(userAddressToSaveFundsTo, tokenAddressToSaveFunds, balance);
        }
        saveFundsTimer = 0;
    }

    /**
     * @notice sets durationToChangeTimer
     * @param _time the new value
     */
    function setDurationToChangeTimer(uint256 _time) external onlyAdmin validAmount(_time) {
        durationToChangeTimer = _time;
    }

    /**
     * @notice starts save funds lockup timer change.
     * @param _time lock up time duration
     */
    function startSaveFundsLockUpTimerChange(uint256 _time)
        external
        override
        onlyAdmin
        validAmount(_time)
    {
        newSaveFundsLockUpTime = _time;
        durationToChangeTimer = saveFundsLockupTime + block.timestamp;

        emit SaveFundsLockUpTimerStarted(msg.sender, _time, durationToChangeTimer);
    }

    /**
     * @notice set save funds lockup time.
     */
    function setSaveFundsLockUpTime() external override onlyAdmin {
        require(
            durationToChangeTimer <= block.timestamp && durationToChangeTimer != 0,
            "ERR: TIMELOCK"
        );

        saveFundsLockupTime = newSaveFundsLockUpTime;
        durationToChangeTimer = 0;

        emit SaveFundsLockUpTimeSet(msg.sender, saveFundsLockupTime, durationToChangeTimer);
    }

    /// @notice External function to pause the contract
    /// @dev only when contract is unpaused
    function pause() external whenNotPaused onlyAdmin {
        _pause();
    }

    /// @notice External function to unpause the contract
    /// @dev only when contract is paused
    function unpause() external whenPaused onlyAdmin {
        _unpause();
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "ERR: ADDRESS");
        _;
    }

    modifier validAmount(uint256 _value) {
        require(_value > 0, "ERR: AMOUNT");
        _;
    }

    modifier onlyAdmin() {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "ERR: PERMISSIONS A");
        _;
    }

    modifier onlyRebalancingOrAdmin() {
        require(
            hasRole(REBALANCING_BOT, msg.sender) || hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ERR: PERMISSIONS A-R"
        );
        _;
    }

    modifier onlyVaultOrAdmin() {
        require(
            hasRole(MOSAIC_VAULT, msg.sender) || hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ERR: PERMISSIONS A-V"
        );
        _;
    }
}
