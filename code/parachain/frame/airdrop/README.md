# Airdrop

The Airdrop pallet enables the granting of tokens to a set of users.

## Overview

The airdrop pallet is a generalization of the crowdloan pallet, allowing users 
and protocols to airdrop tokens to users. It differs from the crowdloan pallet 
in that it supports multiple concurrent airdrops, instead of a single one.

An Airdrop provides the means to distribute a set amount of tokens over some 
amount of time to a set of accounts. These accounts will come from other chains 
and be used to validate claims. Once a claim is made, users will have a portion 
of their Airdropped funds deposited into their local accounts (e.g. Picasso 
accounts). Users will be able to continue to preform validated claims until they 
have claimed all of their funds, or the Airdrop has ended.

## Signing & Verifying Claims

The Airdrop pallet supports remote accounts from Cosmos, Ethereum, and Polkadot
relay chains. To verify account ownership from all of these chains, Airdrop will 
preform validation on signatures natively produced by each chain. In general, 
these signatures are produced by signing messages of the form `{prefix}-{msg}` 
where the `prefix` is decided by our local runtime and the `msg` is either the 
account ID or public key of the remote account. For Ethereum and relay chain 
accounts, `msg` is expected to be the account ID, while for Cosmos accounts, the 
`msg` is expected to be the accounts public key.

Transactions with the `claim` extrinsic are expected to be unsigned. While users 
will sign part of the transaction payload, the transaction itself will be 
unsigned. To prevent transaction spamming, unsigned transactions are validated 
to ensure they have a signed payload and otherwise relevant information for an 
active Airdrop.

## Gas & Fees

When a creator adds recipients to an Airdrop, they can indicate that specific 
users will have their claims funded. If this is true, users will not pay fees 
associated with the `claim` transaction.

## Workflow

Airdrops can be created by any user who is capable of providing the required 
`Stake` needed to create an Airdrop. Once created, the account address 
associated with the creation transaction will be able to utilize the life cycle 
transactions of this pallet. An Airdrop has three life cycle states: created, 
enabled, and disabled.

### Created

During the Created state, the Airdrop can be manipulated with the life cycle 
transactions (`enable_airdrop`, `disable_airdrop`, `add_recipient`,
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
