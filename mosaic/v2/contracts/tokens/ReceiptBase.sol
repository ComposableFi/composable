// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../interfaces/IReceiptBase.sol";

// This contract is used for printing IOU tokens
contract ReceiptBase is IReceiptBase, ERC20 {
    address public override underlyingToken;
    string public receiptType;
    uint256 public chainId;
    address private _owner;
    uint8 private _decimals;
    bool private _initialized;
    string private _actualName;
    string private _actualSymbol;

    event ReceiptTransferOperationExecuted(
        address indexed from,
        address indexed to,
        uint256 amount
    );

    constructor(
        address underlyingAddress,
        string memory prefix,
        uint256 _chainId,
        address ownerAddress,
        string memory _receiptType
    )
        ERC20(
            string(abi.encodePacked(prefix, ERC20(underlyingAddress).name())),
            string(abi.encodePacked(prefix, ERC20(underlyingAddress).symbol()))
        )
    {
        receiptType = _receiptType;
        underlyingToken = underlyingAddress;
        chainId = _chainId;
        _owner = ownerAddress;
        _decimals = ERC20(underlyingAddress).decimals();
        _initialized = true;
        _actualName = string(abi.encodePacked(prefix, ERC20(underlyingAddress).name()));
        _actualSymbol = string(abi.encodePacked(prefix, ERC20(underlyingAddress).symbol()));
    }

    function init(
        address _underlyingAddress,
        string memory _prefix,
        uint256 _chainId,
        address _ownerAddress,
        string memory _receiptType
    ) external {
        require(!_initialized, "already initialized");
        _initialized = true;
        underlyingToken = _underlyingAddress;
        receiptType = _receiptType;
        chainId = _chainId;
        _owner = _ownerAddress;
        _decimals = ERC20(_underlyingAddress).decimals();
        _actualName = string(abi.encodePacked(_prefix, ERC20(_underlyingAddress).name()));
        _actualSymbol = string(abi.encodePacked(_prefix, ERC20(_underlyingAddress).symbol()));
    }

    function name() public view override returns (string memory) {
        return _actualName;
    }

    function symbol() public view override returns (string memory) {
        return _actualSymbol;
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    /**
     * @dev Hook that is called after any transfer of tokens. This includes
     * minting and burning.
     *
     * Calling conditions:
     *
     * - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens has been transferred to `to`.
     * - when `from` is zero, `amount` tokens have been minted for `to`.
     * - when `to` is zero, `amount` of ``from``'s tokens have been burned.
     * - `from` and `to` are never both zero.
     *
     */
    function _afterTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override {
        emit ReceiptTransferOperationExecuted(from, to, amount);
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
