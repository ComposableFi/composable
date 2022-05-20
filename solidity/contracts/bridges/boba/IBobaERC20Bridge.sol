// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IBobaERC20Bridge {
    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _l2Gas,
        bytes calldata _data
    ) external;
}
