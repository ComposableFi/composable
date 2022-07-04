// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../core/InvestmentStrategyBase.sol";
import "./ILendingPool.sol";
import "./ILendingPoolAddressesProvider.sol";
import "./IAaveIncentivesController.sol";

contract AaveInvestmentStrategy is InvestmentStrategyBase {
    using SafeERC20Upgradeable for IERC20Upgradeable;
    ILendingPoolAddressesProvider public lendingPoolAddressesProvider;
    IAaveIncentivesController public incentivesController;

    /// @notice initializes the strategy
    function initialize(
        address _admin,
        address _mosaicHolding,
        address _addressProvider,
        address _incentivesController
    ) public initializer {
        initializeBase(_admin, _mosaicHolding);
        lendingPoolAddressesProvider = ILendingPoolAddressesProvider(_addressProvider);
        incentivesController = IAaveIncentivesController(_incentivesController);
    }

    /// @notice makes an investment into AAVE lending pool
    /// @param investments the investment details
    function makeInvestment(Investment[] calldata investments, bytes calldata)
        external
        override
        onlyInvestor
        nonReentrant
        oneTokenAllowed(investments)
        returns (uint256)
    {
        Investment memory investment = investments[0];
        ILendingPool lendingPool = ILendingPool(lendingPoolAddressesProvider.getLendingPool());

        IERC20Upgradeable(investment.token).safeTransferFrom(
            msg.sender,
            address(this),
            investment.amount
        );
        IERC20Upgradeable(investment.token).safeApprove(address(lendingPool), investment.amount);

        lendingPool.deposit(investment.token, investment.amount, address(this), 0);

        return investment.amount;
    }

    /// @notice withdraws deposited assets from AAVE lending pool
    /// @param investments withdraw details
    function withdrawInvestment(Investment[] calldata investments, bytes calldata)
        external
        override
        onlyInvestor
        nonReentrant
        oneTokenAllowed(investments)
    {
        Investment memory investment = investments[0];
        ILendingPool lendingPool = ILendingPool(lendingPoolAddressesProvider.getLendingPool());
        lendingPool.withdraw(investment.token, investment.amount, mosaicHolding);
    }

    function claimTokens(bytes calldata data) external override onlyInvestor returns (address) {
        address token = abi.decode(data, (address));
        ILendingPool lendingPool = ILendingPool(lendingPoolAddressesProvider.getLendingPool());
        DataTypes.ReserveData memory reserve = lendingPool.getReserveData(token);

        IERC20Upgradeable(reserve.aTokenAddress).safeTransfer(
            mosaicHolding,
            IERC20Upgradeable(reserve.aTokenAddress).balanceOf(address(this))
        );
        return reserve.aTokenAddress;
    }

    /// @notice claim AAVE rewards to holding
    /// @dev callable by the admin only
    /// @param _tokenOut token out to claim for
    function claimRewards(address _tokenOut) external onlyAdmin {
        address[] memory tokens = new address[](1);
        tokens[0] = _tokenOut;
        incentivesController.claimRewards(tokens, type(uint256).max, mosaicHolding);
    }

    /*
    returns the invested amount for the token
    */
    function investmentAmount(address token)
        external
        view
        override
        validAddress(token)
        returns (uint256)
    {
        ILendingPool lendingPool = ILendingPool(lendingPoolAddressesProvider.getLendingPool());
        DataTypes.ReserveData memory reserve = lendingPool.getReserveData(token);
        return IERC20Upgradeable(reserve.aTokenAddress).balanceOf(address(this));
    }
}
