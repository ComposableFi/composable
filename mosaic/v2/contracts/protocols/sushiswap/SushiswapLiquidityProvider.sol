// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

import "../../core/InvestmentStrategyBase.sol";
import "../uniswap/IUniswapV2Router02.sol";
import "../uniswap/IUniswapV2Pair.sol";
import "../uniswap/IUniswapV2Factory.sol";

contract SushiswapLiquidityProvider is InvestmentStrategyBase {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IUniswapV2Router02 public sushiSwapRouter;
    IUniswapV2Factory public sushiswapFactory;

    function initialize(
        address _admin,
        address _mosaicHolding,
        address _sushiSwapRouter,
        address _sushiswapFactory
    ) public initializer validAddress(_sushiSwapRouter) validAddress(_sushiswapFactory) {
        initializeBase(_admin, _mosaicHolding);
        sushiSwapRouter = IUniswapV2Router02(_sushiSwapRouter);
        sushiswapFactory = IUniswapV2Factory(_sushiswapFactory);
    }

    /// @notice External function to set the address of Sushiswap Router
    /// @param _sushiSwapRouter address of the Sushiswap Router
    function setSushiswapRouterAddress(address _sushiSwapRouter)
        external
        onlyAdmin
        validAddress(_sushiSwapRouter)
    {
        sushiSwapRouter = IUniswapV2Router02(_sushiSwapRouter);
    }

    /// @notice External function to set the address of Sushiswap Factory
    /// @param _sushiswapFactory address of the Sushiswap Factory
    function setSushiswapFactoryAddress(address _sushiswapFactory)
        external
        onlyAdmin
        validAddress(_sushiswapFactory)
    {
        sushiswapFactory = IUniswapV2Factory(_sushiswapFactory);
    }

    /// @notice External function called by the MosaicHolding to perform investment
    /// @param _investments struct that contains investment details
    /// @param _data param will be decoded into deadline, minA, minB
    function makeInvestment(Investment[] calldata _investments, bytes calldata _data)
        external
        override
        onlyInvestor
        nonReentrant
        returns (uint256)
    {
        Investment memory investmentA = _investments[0];
        Investment memory investmentB = _investments[1];
        IERC20Upgradeable tokenA = IERC20Upgradeable(investmentA.token);
        IERC20Upgradeable tokenB = IERC20Upgradeable(investmentB.token);

        tokenA.safeTransferFrom(msg.sender, address(this), investmentA.amount);
        tokenB.safeTransferFrom(msg.sender, address(this), investmentB.amount);

        tokenA.safeIncreaseAllowance(address(sushiSwapRouter), investmentA.amount);
        tokenB.safeIncreaseAllowance(address(sushiSwapRouter), investmentA.amount);

        (uint256 deadline, uint256 minA, uint256 minB) = abi.decode(
            _data,
            (uint256, uint256, uint256)
        );
        (, , uint256 liquidity) = sushiSwapRouter.addLiquidity(
            investmentA.token,
            investmentB.token,
            investmentA.amount,
            investmentB.amount,
            minA,
            minB,
            address(this),
            deadline
        );
        return liquidity;
    }

    /// @notice External function called by the MosaicHolding to withdraw investment
    /// @param _investments struct that contains investment details
    /// @param _data param will be decoded into deadline, liquidity
    function withdrawInvestment(Investment[] calldata _investments, bytes calldata _data)
        external
        override
        onlyInvestor
        nonReentrant
    {
        Investment memory investmentA = _investments[0];
        Investment memory investmentB = _investments[1];
        (uint256 deadline, uint256 liquidity) = abi.decode(_data, (uint256, uint256));
        IERC20Upgradeable pair = IERC20Upgradeable(getPair(investmentA.token, investmentB.token));
        pair.safeIncreaseAllowance(address(sushiSwapRouter), liquidity);
        (uint256 amountA, uint256 amountB) = sushiSwapRouter.removeLiquidity(
            investmentA.token,
            investmentB.token,
            liquidity,
            investmentA.amount,
            investmentB.amount,
            address(this),
            deadline
        );

        IERC20Upgradeable(investmentA.token).safeTransfer(mosaicHolding, amountA);
        IERC20Upgradeable(investmentB.token).safeTransfer(mosaicHolding, amountB);
    }

    /// @notice External function called by the MosaicHolding to transfer SushiLP token to mosaicHolding address
    /// @param _data param will be decoded into tokenA and tokenB address
    function claimTokens(bytes calldata _data) external override onlyInvestor returns (address) {
        (address tokenA, address tokenB) = abi.decode(_data, (address, address));
        address pair = getPair(tokenA, tokenB);
        require(pair != address(0), "Token pair doesn't exist");
        uint256 balance = IUniswapV2Pair(pair).balanceOf(address(this));
        IUniswapV2Pair(pair).transfer(mosaicHolding, balance);
        return pair;
    }

    /// @notice External function called to get the balance of SLP
    /// @param _token Address of the SLP token
    function investmentAmount(address _token) external view override returns (uint256) {
        return IERC20Upgradeable(_token).balanceOf(address(this));
    }

    /// @notice Function used to get the address of the tokens pool
    /// @param _tokenA Address of the token A
    /// @param _tokenB Address of the token B
    function getPair(address _tokenA, address _tokenB) public view returns (address) {
        return sushiswapFactory.getPair(_tokenA, _tokenB);
    }

    /// @notice Function used to get how many token B can be brought using _amountA of token A
    /// @param _amountA Amount of token A
    /// @param _tokenA Address of the token A
    /// @param _tokenB Address of the token B
    function getTokenPrice(
        uint256 _amountA,
        address _tokenA,
        address _tokenB
    ) external view returns (uint256) {
        address pair = getPair(_tokenA, _tokenB);
        require(pair != address(0), "Token pair doesn't exist");
        (uint112 reserve0, uint112 reserve1, ) = IUniswapV2Pair(pair).getReserves();
        return sushiSwapRouter.quote(_amountA, reserve0, reserve1);
    }
}
