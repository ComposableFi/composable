# Implementation

Default protocol implementation
-------------------------------

### Observer bot

*   Off-chain component observes pallet-lending, initially it will be our bot as part of pallet-lending
*   It should be given some initial amount of PICA to do transactions
*   As soon as it sees account to liquidate, it sends transaction onto chain.
*   That call pays off full TX cost onto sender account of call if proved to be right.
*   Also it is awarded some value of PICA for success. Value is configurable. Base value can be Weight Cost of transaction. More complex scheme is to make award depending in liquidated amount, so it will not be implemented now. Other strategy could be based on median time must liquidate position sits not touched, will not be implemented too.
*   Lending then calls pallet-liquidation through the Liquidator trait

### Liquidation trait

*   Will make all liquidation engines configured in genesis, no runtime configuration for now
*   Dutch auction will implement LiquidationEngine trait
*   LiquidationOrder.minimum\_price <= InitialAck.bounds.lower will be configurable, will set ZERO for this pallet
*   It will implement full flow without XCM
*   So we will NOT use Dutch Auction trait directly in liquidation

### Cross chain

* Will implement XCM version of Dutch Auction calls
* Will run end to end simulator with mapping assets and liquidations for that
* That will be reference implementation for others
* XCM will use `Transact` commands, not `Transfer`
