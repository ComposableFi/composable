---
title: XCM transfers token loss
---
# Overview

This document describes how a user may lose his tokens during XCM transfers.

As of now, it is about handling `lost` assets on our parachain. 

Please refer to guides of other chains on what they consider to be 'lost' restoration.

This document does not describe a token loss when an XCM message does not reach our parachain XCM message queue.

## Lost locations

When an XCM message arrives, it may bring some tokens within. 

In bad cases token can end up in the:
- `pallet-xcm` trap
- `unknownTokens pallet`
- treasury
- in indexer logs off-chain 

### When assets get into `pallet-xcm` trap? 

Each XCM message is stateful. The normal state is assets in Holder.

For example,
1. Message which does `WithdrawAssets` and `BuyExecution`
2. Execution consumes `BuyExecution` amount which is less than `WithdrawAssets`
3. There is no final `DepositAssets`

Usually happens because of badly formed XCM messages which ordinary users do not send.

Technical collective or Root transfer amount to valid account back.


### When assets get into unknownTokens?

In case if XCM channel is opened but assets are not registered.

For example, 
- Message transfers `(1, parachain(1000), pallet(42), id(32))`. 
- It will appear in the `treasury` account per location.

Technical collective or Root transfer amount to valid account back. 

This can happens because of bad formed XCM message or misconfiguration of XCM(or lack of it for specific chains/assets).

### When do assets end up in `treasury`?

When `BuyExecution` command is not enough to pay fees or `DepositAssets` is not enough for ED of a new account. 

The sender must increase the fee or deposit. In XCM v4, fees will be discoverable.

### When assets appear in indexer logs off the chain?

Hard to imagine or reproduce such a scenario. That can happen in case of XCMP infrastructure failure.