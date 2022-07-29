// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "./CTokenInterfaces.sol";
import "./IComptroller.sol";
import "../../core/InvestmentStrategyBase.sol";

contract CompoundInvestmentStrategy is InvestmentStrategyBase {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IComptroller public comptroller;

    address public compToken;

    mapping(address => address) public cTokens;

    function initialize(
        address _admin,
        address _mosaicHolding,
        address _comptroller,
        address _compToken
    ) public initializer {
        initializeBase(_admin, _mosaicHolding);
        comptroller = IComptroller(_comptroller);
        compToken = _compToken;
    }

    function setCTokensAddress(address token, address cToken)
        external
        onlyAdmin
        validAddress(token)
        validAddress(cToken)
    {
        require(CErc20Interface(cToken).underlying() == token, "Wrong cToken address");
        cTokens[token] = cToken;
    }

    function makeInvestment(Investment[] calldata investments, bytes calldata)
        external
        override
        onlyInvestor
        nonReentrant
        oneTokenAllowed(investments)
        returns (uint256)
    {
        Investment memory investment = investments[0];
        address cToken = cTokens[investment.token];
        require(cToken != address(0), "cToken address not set");

        IERC20Upgradeable(investment.token).safeTransferFrom(
            msg.sender,
            address(this),
            investment.amount
        );
        IERC20Upgradeable(investment.token).safeApprove(cToken, investment.amount);

        uint256 mintedTokens = CErc20Interface(cToken).mint(investment.amount);

        return mintedTokens;
    }

    function withdrawInvestment(Investment[] calldata investments, bytes calldata)
        external
        override
        onlyInvestor
        nonReentrant
        oneTokenAllowed(investments)
    {
        Investment memory investment = investments[0];
        CErc20Interface cTokenERC20 = CErc20Interface(cTokens[investment.token]);
        address underlyingToken = cTokenERC20.underlying();
        require(underlyingToken == investment.token, "Wrong cToken address");
        require(cTokenERC20.redeemUnderlying(investment.amount) == 0, "Withdraw fail");
        IERC20Upgradeable(underlyingToken).safeTransfer(
            mosaicHolding,
            IERC20Upgradeable(underlyingToken).balanceOf(address(this))
        );
    }

    function withdrawCTokenInvestment(address _token, uint256 _amount)
        external
        onlyInvestor
        nonReentrant
    {
        CErc20Interface cTokenERC20 = CErc20Interface(cTokens[_token]);
        address underlyingToken = cTokenERC20.underlying();
        require(underlyingToken == _token, "Wrong cToken address");
        require(cTokenERC20.redeem(_amount) == 0, "Withdraw fail");
        IERC20Upgradeable(underlyingToken).safeTransfer(
            mosaicHolding,
            IERC20Upgradeable(underlyingToken).balanceOf(address(this))
        );
    }

    function claimTokens(bytes calldata) external override onlyInvestor returns (address) {
        comptroller.claimComp(address(this));
        IERC20Upgradeable(compToken).safeTransfer(
            mosaicHolding,
            IERC20Upgradeable(compToken).balanceOf(address(this))
        );
        return compToken;
    }

    function investmentAmount(address token)
        external
        view
        override
        validAddress(token)
        returns (uint256)
    {
        address cToken = cTokens[token];
        require(cToken != address(0), "cToken address not set");
        uint256 cTokenBalance = CErc20Interface(cToken).balanceOf(address(this));
        // Note exchangeRateStored might be smaller than exchangeRateCurrent
        uint256 exchangeRateStored = CErc20Interface(cToken).exchangeRateStored();
        uint256 product = cTokenBalance * exchangeRateStored;
        // exchangeRateStored is scaled by 1e18, truncating
        return product / 1e18;
    }
}
