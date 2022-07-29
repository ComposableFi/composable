# DEX Router

A pallet that provides basic functionality to add a route to a DEX for any given pair of asset id's.

## Overview

The DEX Router pallet's main goal is to enable cross-market trading infrastructure for currency pair's.
This allows us to connect different pools and enabling transactional across multiple pools to reach a requested outcome.
As an example let's say: There is no direct pool to perform transactions for currency A to currency C, but we have routed pools to swap currency A to B and another pool to swap currency B for C. 
This allows for seamless transactions across a large number of currencies

## Workflow

### Route Operations
We start by creating a route for a given set of asset id's. Using the same method we can update or delete the route later. 
Once a route has been established and validated, given instructions based on a quote asset transactional methods become available for the route.
Routes are validated by special origins like root or in the future governance.

### Transaction Operations

Users can compose aforementioned instructions to either `exchange`, `buy` or `sell` for an amount given in the quote asset. 
The necessary operations will be performed on liquidity pools from DEX's we routed earlier.
Said functions can be used to make transactions across multiple pools to achieve the composition of assets requested by the user.

Functions to `add_liquidity` and `remove_liquidity` are constrained to only be called on single route pools.

## Use Cases

Part of DEX-Router's functionality was implemented in pallet pablo.
