// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "./IMosaicNFT.sol";

contract MosaicNFT is IMosaicNFT, Ownable, ERC721URIStorage {
    struct NFTInfo {
        address originalNftAddress;
        uint256 originalNetworkId;
        uint256 originalNftId;
    }

    event NFTMinted(address indexed nftOwner, uint256 indexed nftId);

    event NFTMetadataSet(
        uint256 indexed nftId,
        string nftUri,
        address originalNftAddress,
        uint256 originalNetworkID,
        uint256 originalNftId
    );

    uint256 private idTracker;

    address public minter;

    // mosaic nft ID => original nft info
    mapping(uint256 => NFTInfo) private nftInfoMapping;
    // hash of original nft info => nftId
    mapping(bytes32 => uint256) private mintedNftId;

    constructor(address _minter) ERC721("MosaicNFT", "mNFT") {
        minter = _minter;
    }

    function setMinter(address _minter) external onlyOwner {
        minter = _minter;
    }

    function _setNFTMetadata(
        uint256 _nftId,
        string memory _nftUri,
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId
    ) private {
        bytes32 id = _generateId(_originalNftAddress, _originalNetworkID, _originalNftId);
        require(mintedNftId[id] == 0, "ORIGINAL NFT ALREADY EXISTS");

        _setTokenURI(_nftId, _nftUri);
        nftInfoMapping[_nftId] = NFTInfo(_originalNftAddress, _originalNetworkID, _originalNftId);
        mintedNftId[id] = _nftId;

        emit NFTMetadataSet(
            _nftId,
            _nftUri,
            _originalNftAddress,
            _originalNetworkID,
            _originalNftId
        );
    }

    function _mintNFT(address _to) private {
        require(msg.sender == minter, "ONLY MINTER");
        idTracker = idTracker + 1;
        _safeMint(_to, idTracker);
        emit NFTMinted(_to, idTracker);
    }

    function mintNFT(
        address _to,
        string memory _tokenURI,
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId
    ) external override returns (uint256) {
        _mintNFT(_to);
        _setNFTMetadata(
            idTracker,
            _tokenURI,
            _originalNftAddress,
            _originalNetworkID,
            _originalNftId
        );
        return idTracker;
    }

    function preMintNFT() external override returns (uint256) {
        _mintNFT(minter);
        return idTracker;
    }

    function setNFTMetadata(
        uint256 _nftId,
        string memory _nftUri,
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId
    ) external override {
        require(msg.sender == minter, "ONLY MINTER");
        require(ownerOf(_nftId) == address(minter), "MINTER DOES NOT OWN");
        require(bytes(tokenURI(_nftId)).length == 0, "METADATA ALREADY SET");
        _setNFTMetadata(_nftId, _nftUri, _originalNftAddress, _originalNetworkID, _originalNftId);
    }

    function getLatestId() external view override returns (uint256) {
        return idTracker;
    }

    function getNftId(
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId
    ) external view override returns (uint256) {
        bytes32 id = _generateId(_originalNftAddress, _originalNetworkID, _originalNftId);
        return mintedNftId[id];
    }

    function getOriginalNftInfo(uint256 _nftId)
        external
        view
        override
        returns (
            address,
            uint256,
            uint256
        )
    {
        NFTInfo memory nftInfo = nftInfoMapping[_nftId];
        return (nftInfo.originalNftAddress, nftInfo.originalNetworkId, nftInfo.originalNftId);
    }

    function _generateId(
        address _originalNftAddress,
        uint256 _originalNetworkID,
        uint256 _originalNftId
    ) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(_originalNetworkID, _originalNftAddress, _originalNftId));
    }

    function _transfer(
        address _from,
        address _to,
        uint256 _nftId
    ) internal override {
        require(bytes(tokenURI(_nftId)).length > 0, "METADATA NOT SET");
        super._transfer(_from, _to, _nftId);
    }
}
