// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/IERC721ReceiverUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/IERC721MetadataUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

import "./IMosaicNFT.sol";
import "./ISummonerConfig.sol";

// @title: Composable Finance L2 ERC721 Vault
contract Summoner is
    IERC721ReceiverUpgradeable,
    OwnableUpgradeable,
    PausableUpgradeable,
    ReentrancyGuardUpgradeable
{
    struct Fee {
        address tokenAddress;
        uint256 amount;
    }

    struct TransferTemporaryData {
        string sourceUri;
        uint256 originalNetworkId;
        uint256 originalNftId;
        bytes32 id;
        bool isRelease;
        address originalNftAddress;
    }

    ISummonerConfig public config;
    address public mosaicNftAddress;
    address public relayer;

    uint256 private nonce;

    uint256[] private preMints;

    mapping(address => uint256) public lastTransfer;
    mapping(bytes32 => bool) public hasBeenSummoned; //hasBeenWithdrawn
    mapping(bytes32 => bool) public hasBeenReleased; //hasBeenUnlocked
    bytes32 public lastSummonedID; //lastWithdrawnID
    bytes32 public lastReleasedID; //lastUnlockedID

    // stores the fee collected by the contract against a transfer id
    mapping(bytes32 => Fee) private feeCollection;

    event TransferInitiated(
        address indexed sourceNftOwner,
        address indexed sourceNftAddress,
        uint256 indexed sourceNFTId,
        string sourceUri,
        address destinationAddress,
        uint256 destinationNetworkID,
        address originalNftAddress,
        uint256 originalNetworkID,
        uint256 originalNftId,
        uint256 transferDelay,
        bool isRelease,
        bytes32 id
    );

    event SealReleased(
        address indexed nftOwner,
        address indexed nftContract,
        uint256 indexed nftId,
        bytes32 id
    );

    event SummonCompleted(
        address indexed nftOwner,
        address indexed destinationNftContract,
        string nftUri,
        uint256 mosaicNFTId,
        bytes32 id
    );

    event FeeTaken(
        address indexed owner,
        address indexed nftAddress,
        uint256 indexed nftId,
        bytes32 id,
        uint256 remoteNetworkId,
        address feeToken,
        uint256 feeAmount
    );

    event FeeRefunded(
        address indexed owner,
        address indexed nftAddress,
        uint256 indexed nftId,
        bytes32 id,
        address feeToken,
        uint256 feeAmount
    );
    event ValueChanged(address indexed owner, address oldConfig, address newConfig, string valType);

    function initialize(address _config) public initializer {
        __Ownable_init();
        __ReentrancyGuard_init();
        __Pausable_init();

        nonce = 0;
        config = ISummonerConfig(_config);
    }

    function setConfig(address _config) external onlyOwner {
        emit ValueChanged(msg.sender, address(config), _config, "CONFIG");
        config = ISummonerConfig(_config);
    }

    function setMosaicNft(address _mosaicNftAddress) external onlyOwner {
        require(preMints.length == 0, "ALREADY PRE-MINTED");
        require(lastSummonedID == "", "ALREADY SUMMONED");
        require(_mosaicNftAddress != mosaicNftAddress, "SAME ADDRESS");
        emit ValueChanged(msg.sender, mosaicNftAddress, _mosaicNftAddress, "MOSAICNFT");
        mosaicNftAddress = _mosaicNftAddress;
    }

    function setRelayer(address _relayer) external onlyOwner {
        emit ValueChanged(msg.sender, relayer, _relayer, "RELAYER");
        relayer = _relayer;
    }

    /// @notice External callable function to pause the contract
    function pause() external whenNotPaused onlyOwner {
        _pause();
    }

    /// @notice External callable function to unpause the contract
    function unpause() external whenPaused onlyOwner {
        _unpause();
    }

    function transferERC721ToLayer(
        address _sourceNFTAddress,
        uint256 _sourceNFTId,
        address _destinationAddress,
        uint256 _destinationNetworkID,
        uint256 _transferDelay,
        address _feeToken
    ) external payable nonReentrant {
        require(mosaicNftAddress != address(0), "MOSAIC NFT NOT SET");
        require(_sourceNFTAddress != address(0), "NFT ADDRESS");
        require(_destinationAddress != address(0), "DEST ADDRESS");
        require(paused() == false, "CONTRACT PAUSED");
        require(config.getPausedNetwork(_destinationNetworkID) == false, "NETWORK PAUSED");
        require(
            lastTransfer[msg.sender] + config.getTransferLockupTime() <= block.timestamp,
            "TIMESTAMP"
        );
        require(config.getFeeTokenAmount(_destinationNetworkID, _feeToken) > 0, "FEE TOKEN");
        require(_destinationNetworkID != block.chainid, "TRANSFER TO SAME NETWORK");

        IERC721MetadataUpgradeable(_sourceNFTAddress).safeTransferFrom(
            msg.sender,
            address(this),
            _sourceNFTId
        );
        lastTransfer[msg.sender] = block.timestamp;

        TransferTemporaryData memory tempData;
        tempData.id = _generateId();
        tempData.sourceUri = IERC721MetadataUpgradeable(_sourceNFTAddress).tokenURI(_sourceNFTId);

        if (_sourceNFTAddress == mosaicNftAddress) {
            (
                tempData.originalNftAddress,
                tempData.originalNetworkId,
                tempData.originalNftId
            ) = IMosaicNFT(mosaicNftAddress).getOriginalNftInfo(_sourceNFTId);
        } else {
            tempData.originalNftAddress = _sourceNFTAddress;
            tempData.originalNetworkId = block.chainid;
            tempData.originalNftId = _sourceNFTId;
        }

        if (
            _destinationNetworkID == tempData.originalNetworkId &&
            mosaicNftAddress == _sourceNFTAddress
        ) {
            // mosaicNftAddress is being transferred to the original network
            // in this case release the original nft instead of summoning
            // the relayer will read this event and call releaseSeal on the original layer
            tempData.isRelease = true;
        }

        // the relayer will read this event and call summonNFT or releaseSeal
        // based on the value of isRelease
        emit TransferInitiated(
            msg.sender,
            _sourceNFTAddress,
            _sourceNFTId,
            tempData.sourceUri,
            _destinationAddress,
            _destinationNetworkID,
            tempData.originalNftAddress,
            tempData.originalNetworkId,
            tempData.originalNftId,
            _transferDelay,
            tempData.isRelease,
            tempData.id
        );

        // take fees
        _takeFees(_sourceNFTAddress, _sourceNFTId, tempData.id, _destinationNetworkID, _feeToken);
    }

    function _takeFees(
        address _nftContract,
        uint256 _nftId,
        bytes32 _id,
        uint256 _remoteNetworkID,
        address _feeToken
    ) private {
        uint256 fee = config.getFeeTokenAmount(_remoteNetworkID, _feeToken);
        if (_feeToken != address(0)) {
            require(IERC20Upgradeable(_feeToken).balanceOf(msg.sender) >= fee, "LOW BAL");
            SafeERC20Upgradeable.safeTransferFrom(
                IERC20Upgradeable(_feeToken),
                msg.sender,
                address(this),
                fee
            );
        } else {
            require(msg.value >= fee, "FEE");
        }
        // store the collected fee
        feeCollection[_id] = Fee(_feeToken, fee);
        emit FeeTaken(msg.sender, _nftContract, _nftId, _id, _remoteNetworkID, _feeToken, fee);
    }

    // either summon failed or it's a transfer of the NFT back to the original layer
    function releaseSeal(
        address _nftOwner,
        address _nftContract,
        uint256 _nftId,
        bytes32 _id,
        bool _isFailure
    ) public nonReentrant onlyOwnerOrRelayer {
        require(paused() == false, "CONTRACT PAUSED");
        require(hasBeenReleased[_id] == false, "RELEASED");
        require(
            IERC721MetadataUpgradeable(_nftContract).ownerOf(_nftId) == address(this),
            "NOT LOCKED"
        );

        hasBeenReleased[_id] = true;
        lastReleasedID = _id;

        IERC721MetadataUpgradeable(_nftContract).safeTransferFrom(address(this), _nftOwner, _nftId);

        emit SealReleased(_nftOwner, _nftContract, _nftId, _id);

        // refund fee in case of a failed transaction only
        if (_isFailure == true) {
            _refundFees(_nftOwner, _nftContract, _nftId, _id);
        }
    }

    function _refundFees(
        address _nftOwner,
        address _nftContract,
        uint256 _nftId,
        bytes32 _id
    ) private {
        Fee memory fee = feeCollection[_id];
        // refund the fee
        if (fee.tokenAddress != address(0)) {
            SafeERC20Upgradeable.safeTransfer(
                IERC20Upgradeable(fee.tokenAddress),
                _nftOwner,
                fee.amount
            );
        } else {
            // solhint-disable-next-line avoid-low-level-calls
            (bool success, ) = _nftOwner.call{value: fee.amount}("");
            if (success == false) {
                revert("FAILED REFUND");
            }
        }
        emit FeeRefunded(msg.sender, _nftContract, _nftId, _id, fee.tokenAddress, fee.amount);
    }

    function withdrawFees(
        address _feeToken,
        address _withdrawTo,
        uint256 _amount
    ) external nonReentrant onlyOwner {
        if (_feeToken != address(0)) {
            require(IERC20Upgradeable(_feeToken).balanceOf(address(this)) >= _amount, "LOW BAL");
            SafeERC20Upgradeable.safeTransfer(IERC20Upgradeable(_feeToken), _withdrawTo, _amount);
        } else {
            require(address(this).balance >= _amount, "LOW BAL");
            // solhint-disable-next-line avoid-low-level-calls
            (bool success, ) = _withdrawTo.call{value: _amount}("");
            if (success == false) {
                revert("FAILED");
            }
        }
    }

    /// @notice method called by the relayer to summon the NFT
    function summonNFT(
        string memory _nftUri,
        address _destinationAddress,
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId,
        bytes32 _id
    ) public nonReentrant onlyOwnerOrRelayer {
        // summon NFT cannot be called on the original network
        // the transfer method will always emit release event for this
        require(block.chainid != _originalNetworkID, "SUMMONED ON ORIGINAL NETWORK");
        require(_originalNftAddress != address(0), "ORIGINAL NFT ADDRESS");
        require(paused() == false, "CONTRACT PAUSED");
        require(hasBeenSummoned[_id] == false, "SUMMONED");

        hasBeenSummoned[_id] = true;
        lastSummonedID = _id;

        uint256 mosaicNFTId = IMosaicNFT(mosaicNftAddress).getNftId(
            _originalNftAddress,
            _originalNetworkID,
            _originalNftId
        );

        // original NFT is first time getting transferred
        if (mosaicNFTId == 0) {
            // use a pre minted nft and set the meta data
            mosaicNFTId = getPreMintedNftId();
            if (mosaicNFTId != 0) {
                // set the metadata on the pre minted NFT
                IMosaicNFT(mosaicNftAddress).setNFTMetadata(
                    mosaicNFTId,
                    _nftUri,
                    _originalNftAddress,
                    _originalNetworkID,
                    _originalNftId
                );
                // transfer the nft to the user
                IERC721MetadataUpgradeable(mosaicNftAddress).safeTransferFrom(
                    address(this),
                    _destinationAddress,
                    mosaicNFTId
                );
            } else {
                // if no pre mint found mint a new one
                mosaicNFTId = IMosaicNFT(mosaicNftAddress).mintNFT(
                    _destinationAddress,
                    _nftUri,
                    _originalNftAddress,
                    _originalNetworkID,
                    _originalNftId
                );
            }
        } else {
            // the original nft is locked from a previous transfer from another layer
            // so we need to transfer the NFT instead of minting a new one
            IERC721MetadataUpgradeable(mosaicNftAddress).safeTransferFrom(
                address(this),
                _destinationAddress,
                mosaicNFTId
            );
        }

        emit SummonCompleted(_destinationAddress, mosaicNftAddress, _nftUri, mosaicNFTId, _id);
    }

    function onERC721Received(
        address,
        address,
        uint256,
        bytes memory
    ) public virtual override returns (bytes4) {
        return this.onERC721Received.selector;
    }

    function _generateId() private returns (bytes32) {
        return keccak256(abi.encodePacked(block.number, block.chainid, address(this), nonce++));
    }

    function preMintNFT(uint256 n) external onlyOwnerOrRelayer {
        require(mosaicNftAddress != address(0), "MOSAIC NFT NOT SET");
        for (uint256 i = 0; i < n; i++) {
            uint256 nftId = IMosaicNFT(mosaicNftAddress).preMintNFT();
            preMints.push(nftId);
        }
    }

    function getPreMintedNftId() private returns (uint256) {
        uint256 nftId;
        if (preMints.length > 0) {
            nftId = preMints[preMints.length - 1];
            preMints.pop();
        }
        return nftId;
    }

    function getPreMintedCount() external view returns (uint256) {
        return preMints.length;
    }

    modifier onlyOwnerOrRelayer() {
        require(_msgSender() == owner() || _msgSender() == relayer, "ONLY OWNER OR RELAYER");
        _;
    }
}
