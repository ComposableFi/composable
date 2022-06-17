// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../interfaces/IReceiptBase.sol";

// This contract is used for printing IOU tokens
contract ReceiptBase is IReceiptBase, ERC20 {
    address public override underlyingToken;
    string public constant DETAILS = "https://composable.finance";
    uint256 public chainId;
    address private _owner;
    uint8 private _decimals;

    constructor(
        address underlyingAddress,
        string memory prefix,
        uint256 _chainId,
        address ownerAddress
    )
        ERC20(
            string(abi.encodePacked(prefix, ERC20(underlyingAddress).name())),
            string(abi.encodePacked(prefix, ERC20(underlyingAddress).symbol()))
        )
    {
        underlyingToken = underlyingAddress;
        chainId = _chainId;
        _owner = ownerAddress;
        _decimals = ERC20(underlyingAddress).decimals();
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    /**
     * @notice Mint new receipt tokens to some user
     * @param to Address of the user that gets the receipt tokens
     * @param amount Amount of receipt tokens that will get minted
     */
    function mint(address to, uint256 amount) public override onlySameChain onlyOwner {
        _mint(to, amount);
    }

    /**
     * @notice Burn receipt tokens from some user
     * @param from Address of the user that gets the receipt tokens burn
     * @param amount Amount of receipt tokens that will get burned
     */
    function burn(address from, uint256 amount) public override onlySameChain onlyOwner {
        _burn(from, amount);
    }

    function owner() public view virtual returns (address) {
        return _owner;
    }

    modifier onlySameChain() {
        require(block.chainid == chainId, "Wrong chain");
        _;
    }
    modifier onlyOwner() {
        require(owner() == _msgSender(), "Ownable: caller is not the owner");
        _;
    }
}
