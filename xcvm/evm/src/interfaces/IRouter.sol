// SPDX-License-Identifier: MIT

pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

interface IRouter {
    struct Origin {
        uint32 networkId;
        bytes account;
    }
}
