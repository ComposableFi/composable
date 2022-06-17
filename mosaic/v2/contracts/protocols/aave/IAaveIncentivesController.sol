// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title Interface for the Aave Incentives Controller
interface IAaveIncentivesController {
    /**
     * @dev Claims reward for an user, on all the assets of the lending pool, accumulating the pending rewards
     * @param amount Amount of rewards to claim
     * @param to Address that will be receiving the rewards
     * @return Rewards claimed
     **/
    function claimRewards(
        address[] calldata assets,
        uint256 amount,
        address to
    ) external returns (uint256);

    /**
     * @dev for backward compatibility with previous implementation of the Incentives controller
     */
    // solhint-disable-next-line func-name-mixedcase
    function REWARD_TOKEN() external view returns (address);

    /**
     * @dev returns the unclaimed rewards of the user
     * @param user the address of the user
     * @return the unclaimed user rewards
     */
    function getUserUnclaimedRewards(address user) external view returns (uint256);
}
