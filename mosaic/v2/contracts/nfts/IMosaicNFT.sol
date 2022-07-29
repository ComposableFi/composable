// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IMosaicNFT {
    function getOriginalNftInfo(uint256 nftId)
        external
        view
        returns (
            address,
            uint256,
            uint256
        );

    function getNftId(
        address originalNftAddress,
        uint256 originalNetworkID,
        uint256 originalNftId
    ) external view returns (uint256);

    function mintNFT(
        address _to,
        string memory _tokenURI,
        address originalNftAddress,
        uint256 originalNetworkId,
        uint256 originalNftId
    ) external returns (uint256);

    function preMintNFT() external returns (uint256);

    function getLatestId() external view returns (uint256);

    function setNFTMetadata(
        uint256 nftId,
        string memory nftUri,
        address originalNftAddress,
        uint256 originalNetworkID,
        uint256 originalNftId
    ) external;
}
