# Deployment guide

- This guide lists down all the contracts and various components which are needed to deploy and configure them.
- To deploy you need an owner account which the relayer will use to perform various operations.
- Before deploying the contracts check the [network-name].env file for all the addresses which will be needed to deploy.

### Mosaic holding

`upgradable`

Mosaic holding needs to be deployed before deploying any vault.

deploy and initialize

```shell
NODE_ENV=<network> yarn deploy:mosaic_holding <network>
```

- needs admin address `initialize`
- set the rebalancing thresholds for various tokens `setRebalancingThreshold`
- Set the REBALANCING_BOT

deploy investment strategies

```shell
NODE_ENV=<network> yarn deploy:CompoundInvestmentStrategy <network>
NODE_ENV=<network> yarn deploy:AaveInvestmentStrategy <network>
```

- set on the holding using `addInvestmentStrategy`
- various C token addresses `setCTokensAddress` [only for compound] # todo

[list of commands](/docs/configurations/mosaic-holding.md)

### MosaicVaultConfig

`upgradable`

deploy and initialize

```shell
NODE_ENV=<network> yarn deploy:MosaicVaultConfig <network>
```

- needs MosaicHolding address `initialize`
- needs fee address `initialize` # todo

deploy AMMs

```shell
NODE_ENV=<network> yarn deploy:uniswap_wrapper <network>
NODE_ENV=<network> yarn deploy:sushiswap_wrapper <network>
NODE_ENV=<network> yarn deploy:curve_wrapper <network>
```

- set supportedAMMs on config `addSupportedAMM`

deploy MosaicNativeSwappers

```shell
NODE_ENV=<network> yarn deploy:MosaicNativeSwapperETH:UniswapV2 <network>
NODE_ENV=<network> yarn deploy:MosaicNativeSwapperETH:Sushiswap <network>
NODE_ENV=<network> yarn deploy:MosaicNativeSwapperETH:Spiritswap <network>
NODE_ENV=<network> yarn deploy:MosaicNativeSwapperETH:Spookyswap <network>
NODE_ENV=<network> yarn deploy:MosaicNativeSwapperAVAX:Pangolin <network>
NODE_ENV=<network> yarn deploy:MosaicNativeSwapperAVAX:TraderJoe <network>
```

- set supportedMosaicNativeSwappers on config `addSupportedMosaicNativeSwapper`

[list of commands](/docs/configurations/mosaic-vault.md)

### MosaicVault

`upgradable`

deploy and initialize

```shell
NODE_ENV=<network> yarn deploy:mosaic_vault <network>
```

- needs MosaicVaultConfig address `initialize`
- relayer `setRelayer` # todo
- deploy token factory and then set it on config using `setTokenFactoryAddress`
- set MosaicVault address on config `setVault`
- Set the MOSAIC_VAULT role as on the holding as the vault address

[list of commands](/docs/configurations/mosaic-vault.md)

### Summoner

`upgradable`

NFT vault on various layers.

deploy and initialize

```shell
NODE_ENV=<network> yarn deploy:nft_summoner <network>
```

- deploy summoner config
- deploy summoner
- deploy mosaic nft
- set mosaic nft on summoner
- set relayer on summoner
- set fee tokens per network on summoner

[list of commands](/docs/configurations/nft-summoner.md)

### FundKeeper

`polygon`

deploy and initialize

```shell
NODE_ENV=matic deploy:polygon_fundkeeper matic
```

### How to get the addresses

Once the deployment is complete look for [contract-name]\_Proxy address for the upgradable contracts.

```
deploying "MosaicHolding_L1_Proxy" (tx: 0x761cf5036cd5ffb1544a368eaf93082e178d130ffd956777e8e5fab53381bcbf)...: deployed at 0xE77978Fc20707d93b7B76E293a011d0364612d46 with 669314 gas
implementation initialize
MosaicHolding proxy address: 0xE77978Fc20707d93b7B76E293a011d0364612d46
```

For non-upgradable contracts look for:

```
deploying "L1VaultConfig" (tx: 0xe34bc0fe7e61555f6064002b2d2ad867d2bd740af9a1df4a1cf3f7964be53616)...: deployed at 0xad69b6822F406BE4c7188346806665FEED205108 with 1337947 gas
L1VaultConfig deployed at address: 0xad69b6822F406BE4c7188346806665FEED205108
```

[home](/readme.md)
