// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IInvestmentStrategy {
    struct Investment {
        address token;
        uint256 amount;
    }

    function makeInvestment(Investment[] calldata _investments, bytes calldata _data)
        external
        returns (uint256);

    function withdrawInvestment(Investment[] calldata _investments, bytes calldata _data) external;

    function claimTokens(bytes calldata _data) external returns (address);

    function investmentAmount(address _token) external view returns (uint256);
}
