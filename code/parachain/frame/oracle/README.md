# Apollo

The Oracle pallet provides functionality for setting up and maintaining an Apollo Oracle and submitting prices.

---

## Overview

The Oracle pallet provides functions to:
- Add assets and their respective data, and submit prices for those assets
- Set a signer to ensure proper calls for transactional functions
- Manage stake associated with signer for operating an Oracle
- Adjust reward configuration for Oracles

## Workflows

### Oracle Configuration

Setting up an Oracle for operation requires three steps:
1. `add_asset_and_info` to configure the asset to get prices for
2. `set_signer` to uniquely identify the operator of an Oracle requires a call to `add_stake` to run it.
3. `adjust_rewards` to configure rewards for Oracles

After successfully setting up the Oracle, you can submit prices using the `submit_price` extrinsic.
For more information refer to the [Oracle Set-Up Guide](https://docs.composable.finance/developer-guides/oracle-set-up-guide/oracle-set-up-guide.html)

### Stake Management

The Oracle pallet provides basic functionalities to manage the stake needed to run an Oracle:
- `add_stake` to add more stake
- `remove_stake` claim to remove stake immediately
- `reclaim_stake` reclaim stake after proper time has passed

## References

- [About Apollo](https://docs.composable.finance/products/apollo-overview.html)
- [Design Documentation](https://github.com/ComposableFi/composable/blob/main/frame/oracle/design/design.md)
