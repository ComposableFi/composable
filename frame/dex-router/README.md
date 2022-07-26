# DEX Router

A pallet that provides basic functionality to add a route to a DEX for any given pair of asset id's.

## Overview

The DEX Router pallet's main goal is to enable trading infrastructure for currency pair's outside local.

## Workflow

### Route Operations
We start by creating a route for a given set of asset id's. Using the same method we can update the route later. Once a route has been established and validated, given instructions based on a quote asset transactional methods become available.

### Transaction Operations

We can compose said instructions to either exchange, buy or sell for an amount given in the quote asset with/from/to a liquidity pool from a DEX we routed to earlier.

## Use Cases

This pallet was implemented in pallet pablo.
