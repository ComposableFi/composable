// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IMsgReceiver.sol";

interface IMsgReceiverFactory {
    function relayer() external view returns (address);

    function retrievePersona(address _user) external view returns (address);

    function createPersona(address _user) external returns (address);

    function removePersona(address _user) external;

    function whitelistedFeeTokens(address _tokenAddress) external returns (bool);
}
