// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract DummySetter {
    uint256 public x;

    function setX(uint256 _x) external {
        x = _x;
    }

    function transferTokensAndSetX(
        address _token,
        uint256 _tokenAmount,
        uint256 _x
    ) external {
        IERC20(_token).transferFrom(msg.sender, address(this), _tokenAmount);
        x = _x;
    }
}
