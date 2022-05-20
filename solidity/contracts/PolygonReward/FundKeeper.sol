// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";

contract FundKeeper is Ownable {
    uint256 public amountToSend;
    mapping(address => bool) public fundsTransfered;

    event NewAmountToSend(uint256 newAmount);
    event FundSent(uint256 amount, address indexed user);
    event Paid(address indexed _from, uint256 _value);

    constructor() {
        amountToSend = 0.05 ether;
    }

    receive() external payable {
        emit Paid(msg.sender, msg.value);
    }

    function setAmountToSend(uint256 amount) external onlyOwner {
        amountToSend = amount;
        emit NewAmountToSend(amount);
    }

    function sendFunds(address user) external onlyOwner {
        require(!fundsTransfered[user], "reward already sent");
        require(address(this).balance >= amountToSend, "Contract balance low");

        // solhint-disable-next-line avoid-low-level-calls
        (bool sent, ) = user.call{value: amountToSend}("");
        require(sent, "Failed to send Polygon");

        fundsTransfered[user] = true;
        emit FundSent(amountToSend, user);
    }
}
