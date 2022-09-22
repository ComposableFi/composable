# Overview

This document suggest the way of developing and testing XCM related functional.

There are several options to develop XCM related functional.

1. Create new XCM aware pallet and wrap lower level pallet.
2. Integrate XCM directly into XCM aware pallet.

## About option 1

- May end up ending maximal numbers of pallets.
- Anyway need to modify underlying pallet with hooks for XCM aware pallet

## About option 2

- Can refactor pallet during XCM integration and simplify it, so it is not increase total complexity of pallet
- It will take more time to setup pallet for testing, therefor, XCM functionality must not prevent testing non XCM
- All XCM related testing will be done in full runtime simulator (until test do not use runtime at all)
- We already have Ethereum aware kind of pallets (so it is not clear how other integrations will look like)
- Repetitive XCM setup can be moved into XCM support pallet behind trait later