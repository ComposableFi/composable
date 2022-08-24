# Apollo

The Oracle pallet provides functionality for setting up and maintaining an Apollo oracle and submitting prices.

## Overview

The Oracle provides functions to:
- Add assets and their respective data and submit prices for those assets
- Set a signer to ensure proper calls for transactional functions
- Manage stake associated with signer for operating
- Adjust reward configuration for Oracles

## Workflows

### Oracle Configuration

Setting up an Oracle for operation requires three steps:
1. `add_asset_and_info` to configure the asset to get prices for
2. `set_signer` to uniquely identify the operator of an Oracle by requiring a stake to run.
3. `adjust_rewards` to configure rewards for Oracles

Once an Oracle has been set up it will start to `submit_prices` for an asset every block.

### Stake Management

The Oracle pallet provides basic functionality to manage the stake needed to run an Oracle:
- `add_stake` to add more stake
- `remove_stake` claim to remove stake immediately
- `reclaim_stake` reclaim stake after proper time has passed
