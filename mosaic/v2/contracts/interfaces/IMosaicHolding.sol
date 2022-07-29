// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "../interfaces/IInvestmentStrategy.sol";

interface IMosaicHolding {
    event FoundsInvested(address indexed strategy, address indexed admin, uint256 cTokensReceived);

    event InvestmentWithdrawn(address indexed strategy, address indexed admin);

    event RebalancingThresholdChanged(
        address indexed admin,
        address indexed token,
        uint256 oldAmount,
        uint256 newAmount
    );

    event RebalancingInitiated(
        address indexed by,
        address indexed token,
        address indexed receiver,
        uint256 amount
    );

    event TokenClaimed(address indexed strategy, address indexed rewardTokenAddress);

    event SaveFundsStarted(address owner, address token, address receiver);

    event LiquidityMoved(address indexed to, address indexed tokenAddress, uint256 amount);

    event SaveFundsLockUpTimerStarted(address owner, uint256 time, uint256 durationToChangeTime);

    event SaveFundsLockUpTimeSet(address owner, uint256 time, uint256 durationToChangeTime);

    event ETHTransfered(address receiver, uint256 amount);

    function getTokenLiquidity(address _token, address[] calldata _investmentStrategies)
        external
        view
        returns (uint256);

    function saveFundsLockupTime() external view returns (uint256);

    function newSaveFundsLockUpTime() external view returns (uint256);

    function durationToChangeTimer() external view returns (uint256);

    function transfer(
        address _token,
        address _receiver,
        uint256 _amount
    ) external;

    function transferETH(address _receiver, uint256 _amount) external;

    function setUniqRole(bytes32 _role, address _address) external;

    function approve(
        address _spender,
        address _token,
        uint256 _amount
    ) external;

    /**
     * @dev starts save funds transfer.
     * @param _token Token's balance the owner wants to withdraw
     * @param _to Receiver address
     */
    function startSaveFunds(address _token, address _to) external;

    /**
     * @dev manually moves funds back to L1.
     */
    function executeSaveFunds() external;

    /**
     * @dev starts save funds lockup timer change.
     * @param _time lock up time duration
     */
    function startSaveFundsLockUpTimerChange(uint256 _time) external;

    /**
     * @dev set save funds lock up time.
     */
    function setSaveFundsLockUpTime() external;

    function invest(
        IInvestmentStrategy.Investment[] calldata _investments,
        address _investmentStrategy,
        bytes calldata _data
    ) external;

    function withdrawInvestment(
        IInvestmentStrategy.Investment[] calldata _investments,
        address _investmentStrategy,
        bytes calldata _data
    ) external;

    function coverWithdrawRequest(
        address[] calldata _investmentStrategies,
        bytes[] calldata _data,
        address _token,
        uint256 _amount
    ) external;

    function claim(address _investmentStrategy, bytes calldata _data) external;
}
