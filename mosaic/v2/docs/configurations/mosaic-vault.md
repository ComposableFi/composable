# How to deploy and configure the MosaicVault

NOTE: deploy the mosaic holding before these steps. Follow [this](/docs/configurations/mosaic-holding.md)

deploy and initialize the mosaic vault config

```shell
# TODO: decide the fee address - add in the dot env file as FEE_ADDRESS
NODE_ENV=<network> yarn deploy:MosaicVaultConfig_deploy <network>
```

deploy and initialize the mosaic vault

```shell
NODE_ENV=<network> yarn deploy:mosaic_vault <network>
```

set relayer on mosaic vault

```shell
NODE_ENV=<network> yarn hardhat mosaic_vault_set_relayer \
  --relayer <relayer_address> \
  --network <network_name>
```

add whitelisted tokens

```shell
NODE_ENV=<network> yarn hardhat mosaic_vault_config_whitelist_token \
  --tokenaddress <tokenaddress> \
  --mintransferamount <mintransferamount> \
  --maxtransferamount <maxtransferamount> \
  --network <network_name>
```

add token in remote network

```shell
# the ratio param is OPTIONAL
NODE_ENV=<network> yarn hardhat mosaic_vault_config_add_token_in_network \
  --tokenaddress <token-address> \
  --tokenaddressremote <token-address-remote> \
  --remotenetwork <remote-network-id> \
  --ratio <ratio> \
  --network <network_name>
```

deploy the supported AMMs

```shell
NODE_ENV=<network> yarn deploy:uniswap_wrapper <network>
NODE_ENV=<network> yarn deploy:sushiswap_wrapper <network>
NODE_ENV=<network> yarn deploy:curve_wrapper <network>
```

add AMMs to vault config

```shell
# todo
```

add provide liquidity data

```shell
NODE_ENV=<network> yarn hardhat mosaic_vault_provide_liquidity \
  --amount <amount> \
  --tokenaddress <token-address> \
  --blocksforactiveliq <blocks-for-active-liquidity> \
  --network <network>
```

```shell
NODE_ENV=rinkeby yarn hardhat mosaic_vault_provide_liquidity --amount 100000000 --tokenaddress 0xeb8f08a975ab53e34d8a0330e0d34de942c95926 --blocksforactiveliq 0 --network rinkeby
```

request withdraw liquidity data

```shell
NODE_ENV=<network> yarn hardhat mosaic_vault_withdraw_liquidity_request \
  --amount <amount> \
  --tokenaddress <token-address> \
  --networkid <network-id> \
  --network <network>
```

```shell
NODE_ENV=rinkeby yarn hardhat mosaic_vault_withdraw_liquidity_request --amount 500000 --tokenaddress 0xeb8f08a975ab53e34d8a0330e0d34de942c95926 --networkid 4 --network rinkeby
```

withdraw liquidity data

```shell
NODE_ENV=<network> yarn hardhat mosaic_vault_withdraw_liquidity \
  --amount <amount> \
  --tokenaddress <token-address> \
  --feepercentage <fee-percentage> \
  --basefee <base-fee> \
  --transid <trans-id> \
  --ammid <amm-id> \
  --network <network>
```

```shell
NODE_ENV=rinkeby yarn hardhat mosaic_vault_withdraw_liquidity --amount 500000 --tokenaddress 0xeb8f08a975ab53e34d8a0330e0d34de942c95926 --feepercentage 50 --basefee 100 --transid 0x516bf88dc27853bbd399254a3457143cfe49f69f3b4972aab4f319e7124d3274 --ammid 0 --network rinkeby
```

[Commands to provide lp for sushiswap](/docs/configurations/sushiswap-liquidity-provider.md)

[home](/readme.md) > [deployment guide](/docs/deploy.md)
