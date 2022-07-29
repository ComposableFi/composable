// SPDX-License-Identifier: MIT
/**
 * @summary: Initiate cross chain function calls for whitelisted networks
 * @author: @gcosmintech
 */
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../interfaces/IMsgSender.sol";

import "../libraries/ExcessivelySafeCall.sol";

contract MsgSender is
    IMsgSender,
    OwnableUpgradeable,
    ReentrancyGuardUpgradeable,
    PausableUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;
    using ExcessivelySafeCall for address;

    /// @notice if tru allow forwarding only to whitelsited contracts
    bool public allowOnlyWhitelistedContracts;

    /// @notice relayer address
    address public relayer;

    /// @notice used for generating the unique id
    uint256 public nonce;

    /// @notice pausable per network
    mapping(uint256 => bool) public pausedNetwork;

    /// @notice networks that the contract is able to interact with
    mapping(uint256 => bool) public whitelistedNetworks;

    /// @notice blacklisted addresses cannot use the protocol anymore
    mapping(address => bool) public blacklistedAddress;

    /// @notice whitelisted contracts where calls can be forwarded to
    mapping(uint256 => mapping(address => bool)) public whitelistedContracts;

    /// @notice true when a call was successfully initiated
    mapping(bytes32 => bool) public hasBeenForwarded;

    /// @notice last initiated call
    bytes32 public lastForwardedCall;

    /// @notice event emitted when allow only whitelisted is changed
    event AllowOnlyWhitelistedSwitched(bool oldVal, bool newVal);

    /// @notice event emitted when a new remote chain id is added to the whitelist
    event NetworkAddedToWhitelist(address indexed admin, uint256 chainId);

    /// @notice event emitted when an existing remote chain id is removed from the whitelist
    event NetworkRemovedFromWhitelist(address indexed admin, uint256 chainId);

    /// @notice event emitted when an whitelisted remote chain id is paused
    event PauseNetwork(address indexed admin, uint256 networkID);

    /// @notice event emitted when an whitelisted remote chain id is unpaused
    event UnpauseNetwork(address indexed admin, uint256 networkID);

    /// @notice event emitted when a cross chain call is initiated
    event CallInitiated(address indexed user, uint256 remoteNetworkId);

    /// @notice event emitted when the blacklist status has been changed
    event BlacklistStatusChanged(address indexed user, bool oldVal, bool newVal);

    /// @notice event emitted when a contract is whitelisted
    event WhitelistedContractStatusChanged(
        address indexed contractAddress,
        uint256 chainId,
        bool oldVal,
        bool newVal
    );

    /// @notice event emitted when the relayer is set
    event RelayerSet(address indexed oldVal, address indexed newVal);

    /// @notice event emitted when airdropped tokens are saved from the contract
    event FundsSaved(
        address indexed admin,
        address indexed receiver,
        address indexed token,
        uint256 amount
    );

    /// @notice event emitted when a call is forwarded
    event ForwardCall(
        address indexed user,
        bytes32 id,
        uint256 chainId,
        address feeToken,
        ContractData data
    );

    /// @notice event emitted when a call is forwarded with token approval
    event ForwardCallWithTokenApproval(
        address indexed user,
        bytes32 id,
        uint256 chainId,
        address feeToken,
        address tokenToApprove,
        uint256 amountToApprove,
        ContractData data
    );

    /// @notice event emitted when token approval is forwarded
    event ForwardTokenApproval(
        address indexed user,
        bytes32 id,
        uint256 chainId,
        address feeToken,
        address tokenToApprove,
        uint256 amountToApprove,
        address to
    );

    /// @notice event emitted when save tokens is forwarded
    event ForwardSaveTokens(
        address indexed user,
        bytes32 id,
        uint256 chainId,
        address feeToken,
        address receiver,
        address token,
        uint256 amount
    );

    /// @notice event emitted when save nft tokens is forwarded
    event ForwardSaveNFT(
        address indexed user,
        bytes32 id,
        uint256 chainId,
        address feeToken,
        address receiver,
        address nftContract,
        uint256 nftId
    );

    /// @notice event emitted when save eth is forwarded
    event ForwardSaveETH(
        address indexed user,
        bytes32 id,
        uint256 chainId,
        address feeToken,
        address receiver,
        uint256 amount
    );

    /// @notice XCM event emitted
    /// @dev 'require_weight_at_most' can be calculated offchain; 'call' represents the data needed for the XCM call; 'methodData' is represented by data provided by the method creating the call
    event Transact(
        // solhint-disable-next-line var-name-mixedcase
        address origin_type,
        // solhint-disable-next-line var-name-mixedcase
        uint256 require_weight_at_most,
        bytes call,
        bytes methodData
    );

    function initialize() public initializer {
        __Ownable_init();
        __Pausable_init();
        __ReentrancyGuard_init();
    }

    /// @notice initiates a new cross function call; fees will be taken on the destination layer to avoid rebalancing and user has to pre-fund his MsgReceiver contract
    /// @param _chainId destination chain id
    /// @param _feeToken fee token address,
    /// @param _data destinationContract, destinationData
    function registerCrossFunctionCall(
        uint256 _chainId,
        address _feeToken,
        ContractData calldata _data,
        bool _xcmFormat
    )
        external
        override
        nonReentrant
        onlyWhitelistedNetworks(_chainId)
        onlyUnpausedNetworks(_chainId)
        notBlacklisted(msg.sender)
        checkOnlyWhitelisted(_data.destinationContract, _chainId)
        whenNotPaused
    {
        bytes32 id = _generateId();

        //shouldn't happen
        require(hasBeenForwarded[id] == false, "Call already forwarded");
        require(lastForwardedCall != id, "Forwarded last time");

        lastForwardedCall = id;
        hasBeenForwarded[id] = true;

        if (_xcmFormat) {
            bytes memory xcmData = abi.encodePacked(
                id,
                _chainId,
                _data.destinationContract,
                _feeToken,
                "registerCrossFunctionCall"
            );
            emit Transact(msg.sender, type(uint256).max, _data.destinationData, xcmData);
        } else {
            emit ForwardCall(msg.sender, id, _chainId, _feeToken, _data);
        }
    }

    /// @notice initiates a new cross function call; approve certain token in receiver contract; fees will be taken on the destination layer to avoid rebalancing and user has to pre-fund his MsgReceiver contract
    /// @param _chainId destination chain id
    /// @param _feeToken token to paying the fee
    /// @param _token token to approve tokens in MsgReceiver contract
    /// @param _amount amount of tokens to be approved in MsgReceiver contract
    /// @param _data destinationContract, destinationData
    /// @param _xcmFormat if true will emit XCM format event
    function registerCrossFunctionCallWithTokenApproval(
        uint256 _chainId,
        address _feeToken,
        address _token,
        uint256 _amount,
        ContractData calldata _data,
        bool _xcmFormat
    )
        external
        override
        nonReentrant
        onlyWhitelistedNetworks(_chainId)
        onlyUnpausedNetworks(_chainId)
        notBlacklisted(msg.sender)
        checkOnlyWhitelisted(_data.destinationContract, _chainId)
        whenNotPaused
    {
        bytes32 id = _generateId();

        //shouldn't happen
        require(hasBeenForwarded[id] == false, "Call already forwarded");
        require(lastForwardedCall != id, "Forwarded last time");

        lastForwardedCall = id;
        hasBeenForwarded[id] = true;

        if (_xcmFormat) {
            bytes memory xcmData = abi.encodePacked(
                id,
                _chainId,
                _data.destinationContract,
                _feeToken,
                _token,
                _amount,
                "registerCrossFunctionCallWithTokenApproval"
            );
            emit Transact(msg.sender, type(uint256).max, _data.destinationData, xcmData);
        } else {
            emit ForwardCallWithTokenApproval(
                msg.sender,
                id,
                _chainId,
                _feeToken,
                _token,
                _amount,
                _data
            );
        }
    }

    /// @notice initiates a new cross function call; approve certain token in receiver contract; fees will be taken on the destination layer to avoid rebalancing and user has to pre-fund his MsgReceiver contract
    /// @param _chainId destination chain id
    /// @param _feeToken token to paying the fee
    /// @param _token token to approve tokens in MsgReceiver contract
    /// @param _amount amount of tokens to be approved in MsgReceiver contract
    function registerTokenApproval(
        uint256 _chainId,
        address _feeToken,
        address _token,
        uint256 _amount,
        address _to,
        bool _xcmFormat
    )
        external
        override
        nonReentrant
        onlyWhitelistedNetworks(_chainId)
        onlyUnpausedNetworks(_chainId)
        whenNotPaused
        notBlacklisted(msg.sender)
    {
        bytes32 id = _generateId();

        //shouldn't happen
        require(hasBeenForwarded[id] == false, "Call already forwarded");
        require(lastForwardedCall != id, "Forwarded last time");

        lastForwardedCall = id;
        hasBeenForwarded[id] = true;

        if (_xcmFormat) {
            bytes memory data = abi.encodePacked(
                id,
                _chainId,
                _feeToken,
                _token,
                _amount,
                _to,
                "registerTokenApproval"
            );
            emit Transact(msg.sender, type(uint256).max, "", data);
        } else {
            emit ForwardTokenApproval(msg.sender, id, _chainId, _feeToken, _token, _amount, _to);
        }
    }

    /// @notice transfers ETH available in the contract's balance
    /// @param _chainId destination chain id
    /// @param _receiver destination address
    /// @param _amount eth amount
    /// @param _feeToken fee token
    function registerSaveETH(
        uint256 _chainId,
        address _receiver,
        uint256 _amount,
        address _feeToken,
        bool _xcmFormat
    )
        external
        override
        nonReentrant
        onlyWhitelistedNetworks(_chainId)
        onlyUnpausedNetworks(_chainId)
        whenNotPaused
        notBlacklisted(msg.sender)
    {
        bytes32 id = _generateId();

        //shouldn't happen
        require(hasBeenForwarded[id] == false, "Call already forwarded");
        require(lastForwardedCall != id, "Forwarded last time");

        lastForwardedCall = id;
        hasBeenForwarded[id] = true;

        if (_xcmFormat) {
            bytes memory data = abi.encodePacked(
                id,
                _chainId,
                _feeToken,
                _receiver,
                _amount,
                "registerSaveETH"
            );
            emit Transact(msg.sender, type(uint256).max, "", data);
        } else {
            emit ForwardSaveETH(msg.sender, id, _chainId, _feeToken, _receiver, _amount);
        }
    }

    /// @notice transfers tokens available in the contract's balance
    /// @param _chainId destination chain id
    /// @param _token token address
    /// @param _receiver destination address
    /// @param _amount token amount
    /// @param _feeToken fee token
    function registerSaveTokens(
        uint256 _chainId,
        address _token,
        address _receiver,
        uint256 _amount,
        address _feeToken,
        bool _xcmFormat
    )
        external
        override
        nonReentrant
        onlyWhitelistedNetworks(_chainId)
        onlyUnpausedNetworks(_chainId)
        whenNotPaused
        notBlacklisted(msg.sender)
    {
        bytes32 id = _generateId();

        //shouldn't happen
        require(hasBeenForwarded[id] == false, "Call already forwarded");
        require(lastForwardedCall != id, "Forwarded last time");

        lastForwardedCall = id;
        hasBeenForwarded[id] = true;

        if (_xcmFormat) {
            bytes memory data = abi.encodePacked(
                id,
                _chainId,
                _feeToken,
                _receiver,
                _token,
                _amount,
                "registerSaveTokens"
            );
            emit Transact(msg.sender, type(uint256).max, "", data);
        } else {
            emit ForwardSaveTokens(msg.sender, id, _chainId, _feeToken, _receiver, _token, _amount);
        }
    }

    /// @notice transfers nft tokens available in the contract's balance
    /// @param _chainId destination chain id
    /// @param _nftContract token address
    /// @param _nftId nftid
    /// @param _receiver destination address
    /// @param _feeToken fee token
    function registerSaveNFT(
        uint256 _chainId,
        address _nftContract,
        uint256 _nftId,
        address _receiver,
        address _feeToken,
        bool _xcmFormat
    )
        external
        override
        nonReentrant
        onlyWhitelistedNetworks(_chainId)
        onlyUnpausedNetworks(_chainId)
        whenNotPaused
        notBlacklisted(msg.sender)
    {
        bytes32 id = _generateId();

        //shouldn't happen
        require(hasBeenForwarded[id] == false, "Call already forwarded");
        require(lastForwardedCall != id, "Forwarded last time");

        lastForwardedCall = id;
        hasBeenForwarded[id] = true;

        if (_xcmFormat) {
            bytes memory data = abi.encodePacked(
                id,
                _chainId,
                _feeToken,
                _receiver,
                _nftContract,
                _nftId,
                "registerSaveNFT"
            );
            emit Transact(msg.sender, type(uint256).max, "", data);
        } else {
            emit ForwardSaveNFT(
                msg.sender,
                id,
                _chainId,
                _feeToken,
                _receiver,
                _nftContract,
                _nftId
            );
        }
    }

    /// @notice used to retrieve airdropped tokens from the contract
    /// @param _token token address
    /// @param _receiver funds receiver
    function saveAirdroppedFunds(address _token, address _receiver)
        external
        onlyOwner
        validAddress(_token)
        whenNotPaused
        notBlacklisted(msg.sender)
    {
        uint256 balance = IERC20Upgradeable(_token).balanceOf(address(this));
        require(balance > 0, "No balance");

        IERC20Upgradeable(_token).safeTransfer(_receiver, balance);

        emit FundsSaved(msg.sender, _receiver, _token, balance);
    }

    /// @notice adds a remote chain id to the whitelist
    /// @param _chainId network id
    function addNetwork(uint256 _chainId) external onlyOwner {
        require(whitelistedNetworks[_chainId] == false, "Already whitelisted");

        require(_chainId > 0, "Invalid chain");
        require(block.chainid != _chainId, "Cannot add the same chain");

        whitelistedNetworks[_chainId] = true;
        pausedNetwork[_chainId] = false;

        emit NetworkAddedToWhitelist(msg.sender, _chainId);
    }

    /// @notice removes a remote chain id from the whitelist
    /// @param _chainId network id
    function removeNetwork(uint256 _chainId) external onlyOwner {
        require(whitelistedNetworks[_chainId] == true, "Not whitelisted");
        delete whitelistedNetworks[_chainId];
        delete pausedNetwork[_chainId];
        emit NetworkRemovedFromWhitelist(msg.sender, _chainId);
    }

    /// @notice pauses a whitelisted remote chain id
    /// @param _chainId network id
    function pauseNetwork(uint256 _chainId)
        external
        onlyOwner
        onlyUnpausedNetworks(_chainId)
        onlyWhitelistedNetworks(_chainId)
    {
        pausedNetwork[_chainId] = true;
        emit PauseNetwork(msg.sender, _chainId);
    }

    /// @notice unpauses a whitelisted remote chain id
    /// @param _chainId network id
    function unpauseNetwork(uint256 _chainId)
        external
        onlyOwner
        onlyPausedNetworks(_chainId)
        onlyWhitelistedNetworks(_chainId)
    {
        pausedNetwork[_chainId] = false;
        emit UnpauseNetwork(msg.sender, _chainId);
    }

    /// @notice pauses the contract entirely
    function pause() external whenNotPaused onlyOwner {
        _pause();
    }

    /// @notice unpauses the contract
    function unpause() external whenPaused onlyOwner {
        _unpause();
    }

    /// @notice blacklists address
    function updateBlacklistStatus(address _addr, bool _status) external onlyOwner {
        emit BlacklistStatusChanged(_addr, blacklistedAddress[_addr], _status);
        blacklistedAddress[_addr] = _status;
    }

    /// @notice whitelists contract
    function updateContractWhitelistStatus(
        address _addr,
        bool _status,
        uint256 _chainId
    ) external onlyOwner {
        emit WhitelistedContractStatusChanged(
            _addr,
            _chainId,
            whitelistedContracts[_chainId][_addr],
            _status
        );
        whitelistedContracts[_chainId][_addr] = _status;
    }

    function switchAllowOnlyWhitelited() external onlyOwner {
        emit AllowOnlyWhitelistedSwitched(
            allowOnlyWhitelistedContracts,
            !allowOnlyWhitelistedContracts
        );
        allowOnlyWhitelistedContracts = !allowOnlyWhitelistedContracts;
    }

    /// @notice sets the relayer address
    function setRelayer(address _addr) external onlyOwner {
        emit RelayerSet(relayer, _addr);
        relayer = _addr;
    }

    function _generateId() private returns (bytes32) {
        nonce = nonce + 1;
        return keccak256(abi.encodePacked(block.chainid, block.number, address(this), nonce));
    }

    modifier validAddress(address _addr) {
        require(_addr != address(0), "Invalid address");
        _;
    }

    modifier onlyPausedNetworks(uint256 _network) {
        require(pausedNetwork[_network] == true, "Network not paused");
        _;
    }
    modifier onlyUnpausedNetworks(uint256 _network) {
        require(pausedNetwork[_network] == false, "Network is not active");
        _;
    }

    modifier onlyWhitelistedNetworks(uint256 _network) {
        require(whitelistedNetworks[_network] == true, "Unknown network");
        _;
    }

    modifier notBlacklisted(address _addr) {
        require(!blacklistedAddress[_addr], "Unauthorized");
        _;
    }

    modifier onlyRelayer(address _addr) {
        require(_addr == relayer, "Unauthorized");
        _;
    }

    modifier checkOnlyWhitelisted(address _addr, uint256 _chainId) {
        if (allowOnlyWhitelistedContracts) {
            require(whitelistedContracts[_chainId][_addr], "Contract not whitelisted");
        }
        _;
    }
}
