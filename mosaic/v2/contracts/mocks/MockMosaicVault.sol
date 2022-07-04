// SPDX-License-Identifier: MIT

/**
 * Created on 2021-06-07 08:50
 * @summary: Vault for storing ERC20 tokens that will be transferred by external event-based
 *           system to another network. The destination network can be checked on "connectedNetwork"
 * @author: Composable Finance - denis-abag
 */
pragma solidity ^0.8.0;

import "../core/MosaicVault.sol";

contract MockMosaicVault is MosaicVault {
    // solhint-disable-next-line no-empty-blocks
    function setHasBeenWithdrawn(bytes32 _transferId, bool _value) external {}

    function setHasBeenRefunded(bytes32 _transferId, bool _value) external {
        hasBeenRefunded[_transferId] = _value;
    }

    function resetTransferState(bytes32 _transferId) external {
        delete hasBeenWithdrawn[_transferId];
        delete hasBeenRefunded[_transferId];
    }
}
