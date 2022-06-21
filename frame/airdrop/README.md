# Airdrop

The Airdrop pallet enables the granting of tokens to a set of users.

## Overview

The airdrop pallet is a generalization of the crowdloan pallet, allowing users 
and protocols to airdrop tokens to users. It differs from the crowdloan pallet 
in that it supports multiple concurrent airdrops, instead of a single one.

## Workflow

Airdrops can be created by any user who is capable of providing the required 
`Stake` needed to create an Airdrop. Once created, the account address 
associated with the creation transaction will be able to utilize the life cycle 
transactions of this pallet. An Airdrop has three life cycle states: created, 
enabled, and disabled.

### Created

During the Created state, the Airdrop can be manipulated with the life cycle 
transactions (`enable_airdrop`, `disable_airdrip`, `add_recipient`, 
`remove_recipient`). During this state, claims can not be made by recipients. 

If the Airdrop was created with a `start_at`, it will automatically transition 
to enabled once that point in time has passed. If no `start_at` was provided, 
the airdrop can be manually enabled by the creator.

### Enabled

Once enabled, recipients can begin claiming funds from the Airdrop. Funds will 
become available to users according to the `vesting_schedule` provided at 
creation and the vesting window size provided when each recipient is added.

While the Airdrop is enabled, the creator can still use the life cycle 
transactions in a limited fashion. The most notable limitation is that, once a 
recipient has started claiming funds, they cannot be removed from the Airdrop.

Once there are no more remaining unclaimed funds (via claiming or removing of 
inactive recipients) the Airdrop will automatically transition to the disabled 
state. The disabled state can also be manually triggered with the 
`disable_airdrop` transaction.

### Disabled

Once an Airdrop has been disabled, it will be removed from pallet storage along 
with other related information.
