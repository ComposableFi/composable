# DEX Router

A pallet that provides basic functionality to add a route to a DEX for any given pair of asset id's.

## Overview

The DEX Router pallet is used to find liquidity pools on DEX's for a set given assets. The pallet also provides methods to maintain established routes.

## Workflow

### Route Operations
We start by creating a route for a given set of asset id's. Using the same method we can update the route later. Once a route has been established and validated, given instructions based on a quote asset transactional methods become available.

### Transaction Operations

We can compose said instructions to either exchange, buy or sell for an amount given in the quote asset with/from/to a liquidity pool from a DEX we routed to earlier.

## Use Cases

- This pallet was implemented in pallet pablo to provide an easy way for users to take part in the Defi ecosystem.
- Swap exchange
- Liquidity pool aggregator
- Trading bot