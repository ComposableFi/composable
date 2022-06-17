// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../ReceiptBase.sol";

// This contract is used for printing IOU tokens
contract IOUToken is ReceiptBase {
    string public receiptType = "IOU";

    constructor(
        address underlyingAddress,
        string memory prefix,
        uint256 _chainId,
        address admin
    ) ReceiptBase(underlyingAddress, prefix, _chainId, admin) {}
}
