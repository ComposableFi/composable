// SPDX-License-Identifier: MIT
/**
 * @summary: Handles MsgReceiver contract management
 * @author: @gcosmintech
 */
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import "@openzeppelin/contracts/proxy/Clones.sol";

import "../interfaces/IMsgReceiverFactory.sol";
import "./MsgReceiver.sol";

contract MsgReceiverFactory is
    IMsgReceiverFactory,
    OwnableUpgradeable,
    PausableUpgradeable,
    ReentrancyGuardUpgradeable
{
    /// @notice relayer address
    address public override relayer;

    /// @notice tokens from which fee is withdrawn
    mapping(address => bool) public override whitelistedFeeTokens;

    /// @notice all personas created so far
    mapping(address => address) public personas;

    /// @notice address of MsgReceiver implementation to be cloned from
    address public personaImplementationFactory;

    /// @notice event emitted when a new persona is created
    event PersonaCreated(address indexed user, address indexed persona);

    /// @notice event emitted when persona is deleted
    event PersonaRemoved(address indexed admin, address indexed persona);

    /// @notice event emitted when relayer address is changed
    event RelayerChanged(
        address indexed admin,
        address indexed oldRelayer,
        address indexed newLayer
    );
    event RelayerChangedForPersona(
        address indexed admin,
        address newLayer,
        address indexed persona
    );

    /// @notice event emitted when a new token is added to the fee whitelist
    event AddTokenToFeeWhitelist(address indexed admin, address indexed token);

    /// @notice event emitted when an existing token is removed from the fee whitelist
    event RemoveTokenFromFeeWhitelist(address indexed admin, address indexed token);

    function initialize(address _relayer) public initializer {
        __Ownable_init();
        __Pausable_init();
        __ReentrancyGuard_init();
        relayer = _relayer;
        personaImplementationFactory = address(new MsgReceiver());
    }

    /// @notice used to retrieve an existing persona
    /// @param _user persona's owner
    /// @return _user persona's address
    function retrievePersona(address _user) external view override returns (address) {
        return personas[_user];
    }

    /// @notice used to create a persona for user
    /// @param _user user address
    function createPersona(address _user)
        external
        override
        whenNotPaused
        nonReentrant
        validAddress(_user)
        returns (address)
    {
        require(personas[_user] == address(0), "Already created");
        address newPersona = Clones.clone(personaImplementationFactory);

        MsgReceiver persona = MsgReceiver(payable(newPersona));
        persona.init(_user, address(this));
        emit PersonaCreated(_user, address(persona));
        personas[_user] = address(persona);
        return address(persona);
    }

    /// @notice used to delete an existing persona
    /// @param _user persona's owner
    function removePersona(address _user)
        external
        override
        nonReentrant
        onlyOwnerOrRelayer
        validAddress(_user)
    {
        require(personas[_user] != address(0), "Not found");

        emit PersonaRemoved(msg.sender, personas[_user]);

        delete personas[_user];
    }

    /// @notice adds token to the fee whitelist
    /// @param _token token address
    function addFeeToken(address _token) external onlyOwner validAddress(_token) {
        require(whitelistedFeeTokens[_token] == false, "Already whitelisted");
        whitelistedFeeTokens[_token] = true;
        emit AddTokenToFeeWhitelist(msg.sender, _token);
    }

    /// @notice remove tokens from the fee whitelist
    /// @param _token token address
    function removeFeeToken(address _token) external onlyOwner validAddress(_token) {
        require(whitelistedFeeTokens[_token] == true, "Not found");
        delete whitelistedFeeTokens[_token];
        emit RemoveTokenFromFeeWhitelist(msg.sender, _token);
    }

    /// @notice sets the relayer address
    /// @param _relayer new relayer address
    function setRelayer(address _relayer) external onlyOwner validAddress(_relayer) {
        emit RelayerChanged(msg.sender, relayer, _relayer);
        relayer = _relayer;
    }

    /// @notice pauses the contract entirely
    function pause() external whenNotPaused onlyOwner {
        _pause();
    }

    /// @notice unpauses the contract
    function unpause() external whenPaused onlyOwner {
        _unpause();
    }

    modifier onlyOwnerOrRelayer() {
        require(_msgSender() == owner() || _msgSender() == relayer, "Only owner or relayer");
        _;
    }

    modifier validAddress(address _addr) {
        require(_addr != address(0), "Invalid address");
        _;
    }
}
