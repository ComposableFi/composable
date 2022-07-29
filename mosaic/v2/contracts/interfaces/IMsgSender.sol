// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IMsgSender {
    /// @notice data structure for call forwarding
    struct ContractData {
        address destinationContract;
        bytes destinationData;
    }

    function registerCrossFunctionCall(
        uint256 _chainId,
        address _feeToken,
        ContractData calldata _data,
        bool _xcmFormat
    ) external;

    function registerCrossFunctionCallWithTokenApproval(
        uint256 _chainId,
        address _feeToken,
        address _token,
        uint256 _amount,
        ContractData calldata _data,
        bool _xcmFormat
    ) external;

    function registerTokenApproval(
        uint256 _chainId,
        address _feeToken,
        address _token,
        uint256 _amount,
        address _to,
        bool _xcmFormat
    ) external;

    function registerSaveTokens(
        uint256 _chainId,
        address _token,
        address _receiver,
        uint256 _amount,
        address _feeToken,
        bool _xcmFormat
    ) external;

    function registerSaveNFT(
        uint256 _chainId,
        address _nftContract,
        uint256 _nftId,
        address _receiver,
        address _feeToken,
        bool _xcmFormat
    ) external;

    function registerSaveETH(
        uint256 _chainId,
        address _receiver,
        uint256 _amount,
        address _feeToken,
        bool _xcmFormat
    ) external;
}
