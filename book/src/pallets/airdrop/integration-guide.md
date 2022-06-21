# Airdrop Integration Guide

[**Pallet Overview & Workflow**](../airdrop.md)

## Integration Status

| Dali | Picasso     | Composable |
| ---- | ----------- | ---------- |
| IP   | No          | No         |

## Setup / Configuration

Airdrops are created and managed by users. Only the airdrop creator will have 
access to life cycle methods of the airdrop. Airdrops have three states: 
created, enabled, disabled. These states transition linearly. 

During the created state, no claims can be made, but all management transactions 
can be conducted. During the enabled state, claims can be made and management 
transactions can be conducted in a limited fashion. During the disabled state, 
the airdrop and its relevant info in pallet storage will be deleted.

Automatic state transitions can accrue under two conditions. 1) The Airdrop was 
provided with a scheduled `start_at` and that time has come, the Airdrop will be 
enabled. 2) All funds in the Airdrop have been claimed and/or a recipient was 
deleted that leaves no more funds to claim, the Airdrop will be disabled and 
deleted from storage.

For more details on the Airdrop life cycle, see the [Workflow section](../airdrop.md#workflow) 
of the Pallet Overview & Workflow file. This information is retrieved directly 
from the pallet's `READMD.md`.

## RPC & Data Retrieval
<!-- RPCs w/ links to cargo docs? -->
*Soon(TM)*

## Subsquid Data Retrieval

<!-- Not required yet since we have no subsquid yet -->
N/A

## Locally Consumed Types

### Types

* `AirdropId` - Numeric type used to uniquely identify Airdrops
* `Balance` - Numeric type used to represent some amount of tokens
* `Convert` - Function for converting between `Moment` and `Balance`
* `Moment` - Numeric type used to represent a time stamp
* `RelayChainAccountId` - Numeric type used to uniquely identify relay chain accounts
* `Time` - Time provider
* `WeightInfo` - Provider for extrinsic transaction weights

### Constants

* `PalletId` - Unique ID of the pallet
* `Prefix` - The prefix used in proofs
* `Stake` - The amount required to create an Airdrop

## Calculations & Sources of Values
<!-- "Provide calculations of APY or APR if any and mention the source of all 
values that need to be fetched from the chain/backend/subsquid or any other data 
source" -->
N/A

## Extrinsic Parameter Sources

### create_airdrop

* `start_at` - user provided, optional
* `vesting_scchedule` - user provided

### add_recipient

* `airdrop_id` - user selected, provided by the system
* `recipients` - user provided

### remove_recipient

* `airdrop_id` - user selected, provided by the system
* `recipient` - user selected, provided by the system

### enable_airdrop

* `airdrop_id` - user selected, provided by the system

### disable_airdrop

* `airdrop_id` - user selected, provided by the system

### claim

* `airdrop_id` - user selected, provided by the system
* `reward_account` - user provided
* `proof` - calculated by the system (requires applicable signing)

## Pricing Sources
<!-- "Pricing sources are a must have if any Zeplin designs show users values in USD$" -->
N/A

