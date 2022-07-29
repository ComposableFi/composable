# How to deploy and configure the MosaicHolding

deploy and initialize

```shell
NODE_ENV=<network> yarn deploy:mosaic_holding <network>
```

set REBALANCING_BOT address

```shell
NODE_ENV=<network> yarn hardhat mosaic_holding_set_role \
  --role REBALANCING_BOT \
  --address <role_address> \
  --network <network_name>
```

set rebalancing threshold task

```shell
NODE_ENV=<network> yarn hardhat mosaic_holding_set_rebalancing_threshold \
  --token <token_address> \
  --amount <amount> \
  --network <network_name>
```

deploy and add investment strategies

aave

```shell
NODE_ENV=<network> yarn deploy:AaveInvestmentStrategy <network>
```

compound

```shell
NODE_ENV=<network> yarn deploy:CompoundInvestmentStrategy <network>
```

set the c-tokens on compound strategy

```shell
# todo
```

sushiswap lp provider

```shell
NODE_ENV=<network> yarn deploy:SushiswapLiquidityProvider <network>
```

[home](/readme.md) > [deployment guide](/docs/deploy.md)
