##  Resolver queries

### activeUsers

Returns the number of active users for a given time range.

`query`

```graphql
activeUsers(params: {range: "month"}) {
    date
    count
  }
```
`response`

```json
"activeUsers": [
      {
        "date": "2023-03-14T00:00:00.000Z",
        "count": 48
      },
      {
        "date": "2023-03-15T00:00:00.000Z",
        "count": 63
      },
      {
        "date": "2023-03-16T00:00:00.000Z",
        "count": 56
      },
      ...
]
```

### assetPrices

Returns the price of an asset in USD.

`query`
```graphql
assetsPrices(params: {assetId: "4"}) {
    price
}
```

`response`

```json
"assetsPrices": {
  "price": 35.26
}
```

### pabloDaily

Returns the daily stats for a given pool.

`query`

```graphql
pabloDaily(params: {poolId: "1"}) {
    assetId
    fees {
      assetId
      amount
      price
    }
    transactions
    volume {
      price
      assetId
      amount
    }
    tradingFeeApr
    swapFee
    poolId
  }
```

`response`

```json
"pabloDaily": {
      "assetId": "1",
      "fees": [
        {
          "assetId": "1",
          "amount": "383291152933406",
          "price": 0.0007224847832511037
        },
        {
          "assetId": "130",
          "amount": "886005",
          "price": 1.001
        }
      ],
      "transactions": "11",
      "volume": [
        {
          "price": 0.0007224847832511037,
          "assetId": "1",
          "amount": "127763717644467832"
        },
        {
          "price": 1.001,
          "assetId": "130",
          "amount": "295333622"
        }
      ],
      "tradingFeeApr": 0.05375272528665437,
      "swapFee": 0.003,
      "poolId": "1"
    }
```

### pabloDailyTransactions

Returns the daily transactions for a given pool.

`query`

```graphql
pabloDailyTransactions(params: {address: "5uye1G73P5SWBEy4SNVUghXpfPDPF5tjwpPz7QYWxhpiEt1C"}) {
    transactions {
      amounts {
        assetId
        amount
        price
      }
      failDescription
      failReason
      poolId
      success
      swap {
        spotPrice
        quoteAssetId
        quoteAssetAmount
        feeAssetId
        feeAssetAmount
        baseAssetId
        baseAssetAmount
      }
      timestamp
      txHash
      txType
    }
  }
```

`response`

```json
"pabloDailyTransactions": {
    "transactions": [
        {
            "amounts": null,
            "failDescription": null,
            "failReason": null,
            "poolId": "0",
            "success": true,
            "swap": {
                "spotPrice": "35.25816845802044",
                "quoteAssetId": "130",
                "quoteAssetAmount": "35511337",
                "feeAssetId": "130",
                "feeAssetAmount": "106535",
                "baseAssetId": "4",
                "baseAssetAmount": "1007180422383"
            },
            "timestamp": 1681377390980,
            "txHash": "0x747dab637752823c78003071ac1914d31c83c77f06e7990f5528cfcab532ea07",
            "txType": "SWAP"
        },
        ...
    ]
}

```


### pabloOverviewStats

Returns the overview stats for Pablo, which includes daily volume and TVL.

`query`

```graphql
pabloOverviewStats {
    dailyVolume {
      assetId
      amount
      price
    }
    totalValueLocked {
      assetId
      amount
      price
    }
}
```

`response`

```json
"pabloOverviewStats": {
  "dailyVolume": [
    {
      "assetId": "1",
      "amount": "392120810041400757",
      "price": 0.0007198308408430041
    },
    {
      "assetId": "4",
      "amount": "3514307070595",
      "price": 35.26
    },
    {
      "assetId": "130",
      "amount": "1236530014",
      "price": 1.001
    }
  ],
  "totalValueLocked": [
    {
      "assetId": "1",
      "amount": "30552193362823820442",
      "price": 0.0007198308408430041
    },
    {
      "assetId": "4",
      "amount": "1307536545076453",
      "price": 35.26
    },
    {
      "assetId": "130",
      "amount": "39974454311",
      "price": 1.001
    }
  ]
}
```

### pabloSpotPrice

Returns the spot price for a given pool. `baseAssetId` and `quoteAssetId` need to match the pool's asset IDs, in any order.

`query`

```graphql
pabloSpotPrice(params: {baseAssetId: "130", quoteAssetId: "4", poolId: "0"}) {
    spotPrice
}
```

`response`

```json
"pabloSpotPrice": {
  "spotPrice": "35.30902076823027"
}
```

### pabloSpotPriceChart

Returns the spot price chart for a given pool and range. Provides the history for each of the pool's assets in terms of the other.

`query`

```graphql
pabloSpotPriceChart(params: {range: "week", poolId: "1"}) {
    assetId
    history {
      date
      spotPrice
    }
  }
```

`response`

```json
 "pabloSpotPriceChart": [
      {
        "assetId": "1",
        "history": [
          {
            "date": "2023-04-06T00:00:00.000Z",
            "spotPrice": 0.000757950176392605
          },
          ...
          {
            "date": "2023-04-13T00:00:00.000Z",
            "spotPrice": 0.0006951271606562785
          }
        ]
      },
      {
        "assetId": "130",
        "history": [
          {
            "date": "2023-04-06T00:00:00.000Z",
            "spotPrice": 1319.3479349254974
          },
          ...
          {
            "date": "2023-04-13T00:00:00.000Z",
            "spotPrice": 1438.5857100676187
          }
        ]
      }
    ]
```

### pabloTotalVolume

Returns the total volume for a given Pablo pool and range.

`query`

```graphql
pabloTotalVolume(params: {range: "month"}) {
  date
  volumes {
    assetId
    amount
    price
  }
}
```

`response`

```json
"pabloTotalVolume": [
  {
    "date": "2023-03-14T00:00:00.000Z",
    "volumes": [
      {
        "assetId": "1",
        "amount": "473028994568114336",
        "price": 0.0009519997907450687
      },
      {
        "assetId": "4",
        "amount": "22876295847805",
        "price": 33.510639007458245
      },
      {
        "assetId": "130",
        "amount": "1507484475",
        "price": 1.0036596765164458
      }
    ]
  },
  ...
  {
    "date": "2023-04-13T00:00:00.000Z",
    "volumes": [
      {
        "assetId": "1",
        "amount": "934188884385815079",
        "price": 0.0007065966603805569
      },
      {
        "assetId": "4",
        "amount": "26129405958891",
        "price": 34.37141070184467
      },
      {
        "assetId": "130",
        "amount": "598844616",
        "price": 1.0007944670821702
      }
    ]
  }
]
   
```

### pabloTVL

Returns the total value locked for a given Pablo pool and range.

`query`

```graphql
  pabloTVL(params: {range: "month", poolId: "0"}) {
    date
    lockedValues {
      assetId
      amount
      price
    }
  }
```

`response`

```json
    "pabloTVL": [
      {
        "date": "2023-03-14T00:00:00.000Z",
        "lockedValues": [
          {
            "assetId": "4",
            "amount": "937748975096780",
            "price": 33.510639007458245
          },
          {
            "assetId": "130",
            "amount": "31335423476",
            "price": 1.0036596765164458
          }
        ]
      },
      ...
      {
        "date": "2023-04-13T00:00:00.000Z",
        "lockedValues": [
          {
            "assetId": "4",
            "amount": "924383351570515",
            "price": 34.37141070184467
          },
          {
            "assetId": "130",
            "amount": "31451335516",
            "price": 1.0007944670821702
          }
        ]
      }
    ]
```

## Raw queries

These are automatically generated queries that can be used to fetch raw data from the database. They are not intended to be used directly, but rather as a reference for the data that is available.

### 1. historicalAssetPrices

Returns all prices stored for all assets, with their respective `timestamp`.

___Reference___

###### assetId

ID of the asset.

###### currency

Currency in which the price is expressed. For now, it is always `USD`

###### price

Price of the asset in the reference currency.

### 2. historicalLockedValues

Returns all the history of locked values for all sources (ex. `Pablo`), assets and entities (ex. `Pablo pools`).

___Reference___

###### source
Where is the locked value coming from. Ex. `Pablo` or `StakingRewards`.

###### sourceEntityId:
ID of the entity that is locking the value. Ex. `Pablo pool ID`.

###### assetId:
ID of the asset that is being locked.

###### amount:
Amount locked. Can be negative if the value is being unlocked.

###### accumulatedAmount:
Total amount locked up to that moment, in the reference asset.

### 3. historicalPabloFeeAprs

Returns all the history of Pablo fee APRs for all pools.

___Reference___

###### pool

Object that includes all data from the pool. This can be used when filtering by `pool ID`.

###### tradingFee

Trading fee of the pool at the given timestamp.

### 4. historicalStakingAprs

Similar to the previous one, but not implemented yet (staking pallet missing) so it will empty.

### 5. historicalVolumes

Returns all the history of volumes for all pools and assets.

___Reference___

###### pool

Object that includes all data from the pool. This can be used when filtering by `pool ID`.

###### assetId

ID of the asset that is being traded.

###### amount

Amount traded for the given asset.

###### accumulatedAmount

Total amount traded for the given asset up to that moment.

### 6. pabloSwaps

Returns all the swaps that have been executed in Pablo.

___Reference___

###### pool

Object that includes all data from the pool.

###### baseAssetId

ID of the base asset for the swap.

###### baseAssetAmount

Amount of the base asset for the swap.

###### quoteAssetId

ID of the quote asset for the swap.

###### quoteAssetAmount

Amount of the quote asset for the swap.

###### spotPrice

Spot price of the swap.

###### fee

Fee paid for the swap. Includes different types of fees, like `lpFee`, `ownerFee`, and `protocolFee`.

###### success

Boolean expressing whether the swap was successful or not.

### pabloTransactions

Returns all the transactions that have been executed in Pablo.

___Reference___

###### pool

Object that includes all data from the pool.

###### account

Account that executed the transaction.

###### txType

Type of transaction: `ADD_LIQUIDITY`, `REMOVE_LIQUIDITY`, `SWAP`.

###### error

Object which includes data about the error, if the transaction failed.

###### liquidityAdded / liquidityRemoved / swap

Objects that include specific data about the transaction.

### 7. vestingSchedules

Returns all the vesting schedules that have been created in the network.

___Reference___

###### from / to

Account that pays/receives the vesting schedule.

###### fullyClaimed

Boolean expressing whether the vesting schedule has been fully claimed or not.

###### totalAmount

Total amount of the vesting schedule.

###### schedule

Object with the schedule data, including the amount that has already been claimed (`alreadyClaimed`).

