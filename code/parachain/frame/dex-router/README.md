# DEX Router

A pallet that provides basic functionality to add a route to a DEX for any given pair of asset id's.

## Overview

The DEX Router pallet's main goal is to enable cross-market trading infrastructure for currency pairs.
This allows us to connect different pools and enables transactions across multiple pools to reach a requested outcome.
### Example

Assume the following pools exist:

A -> B
B -> C

Without the router, swapping between A and C is not possible. With the router, however, we are able to *route through* the aforementioned pools to perform the transaction from A -> C.
This allows for seamless transactions across a large number of currencies.

## Workflow

### Route Operations
We start by creating a route for a given set of asset id's. Using the same method we can update or delete the route later. 
Once a route has been established and validated, instructions based on the quote asset's transactional methods become available for the route.
Routes are validated by a specific origin (currently root) but will in the future be validated through governance.

### Transaction Operations

Users can compose aforementioned instructions to either `exchange`, `buy` or `sell` for an amount given in the quote asset. 
The necessary operations will be performed on liquidity pools from the previously routed DEXes..
Said functions can be used to make transactions across multiple pools to achieve the composition of assets requested by the user.

Functions to `add_liquidity` and `remove_liquidity` are constrained to only be called on single pool routes.

## Use Cases
Dex Router is built onto pallet pablo to differentiate pablo pools which should be treated as verified.
