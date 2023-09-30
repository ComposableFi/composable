pragma solidity ^0.8.14;

library xc {

    struct AbsoluteAmount {
        uint128 asset_id;
        uint256 amount;
    }

    struct ProgramRequest {
        bytes salt; 
        address tip;   
    }

    ///
    //type AboluteAssets is AbsoluteAmount[]; 
}