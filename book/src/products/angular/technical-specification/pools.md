# Pools

Angular is isolated pairs lending. Each pool(lending market) consists of two assets (the lending pair) . Providing liquidity or borrowing does into/from one pool does not influence other pool directly. Isolated lending pairs isolate the risk of a collateral default to that specific pool, unlike mixed-asset pools.


Users can create and configure there their own lending pairs without needing the approval of the protocol or complicated governance. Lending pools are permissionless. Creating pools requires stake locked as spam protection.


Only assets which has Apollo oracle price information can be used to create pool. Prices help to determine collateralization ratios and liquidation prices. Liquidity provider tokens (LP) when used as collateral have their prices determined from the respective assets in the underlying pools.


Liquidity is provided per pool and not shared.


There is no global debt ceiling because markets are isolated and allowed to collapse one by one. Vault limits amount of borrowed assets can be taken back.
