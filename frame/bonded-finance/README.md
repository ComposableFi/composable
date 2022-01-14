# Bonded Finance Pallet

## Overview

A simple pallet providing means of submitting bond offers.

## Interface

This pallet implements the `BondedFinance` trait from `composable-traits`.

## Dispatchable Functions

- `offer` ― Register a new bond offer, allowing use to later bond it.
- `bond` ― Bond to an offer, the user should provide the number of contracts a user is willing
  to buy. On offer completion (a.k.a. no more contract on the offer), the `stake` put by the creator is refunded.
- `cancel_offer` ― Cancel a running offer, blocking further bond but not cancelling the
  currently vested rewards. The `stake` put by the creator is refunded.
