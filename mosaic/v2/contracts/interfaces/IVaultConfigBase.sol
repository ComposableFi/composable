// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IVaultConfigBase {
    event TokenCreated(address _underlyingToken);

    function getMosaicHolding() external view returns (address);

    function setTokenFactoryAddress(address _tokenFactoryAddress) external;

    function setVault(address _vault) external;
}
