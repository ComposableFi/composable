# How to deploy and configure Summoner

Run the below commands to deploy and configure the Summoner contract.

deploy and initialize all contracts

```shell
NODE_ENV=<network> yarn deploy:nft_summoner <network>
```

set relayer on summoner

```shell
NODE_ENV=<network> yarn hardhat summoner_set_relayer --relayer <relayer_address> --network <network_name>
```

set fee tokens on the summoner config

```shell
NODE_ENV=<network> yarn hardhat summoner_set_fee_token \
  --remotenetwork <network_id> \
  --token <token_address> \
  --amount <fee_amount> \
  --network <network_name>
```

Commands mainnet:

```shell
NODE_ENV=mainnet yarn deploy:nft_summoner mainnet
# todo - change the relayer address
NODE_ENV=mainnet yarn hardhat summoner_set_relayer \
  --relayer 0xaaaa701efea3AC6B184628eD104F827014641592 \
  --network mainnet
# 50 usdc for mainnet to polygon
NODE_ENV=mainnet yarn hardhat summoner_set_fee_token \
 --remotenetwork 137 \
 --token 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
 --amount 50000000 \
 --network mainnet
# 0.02 ether for mainnet to polygon
NODE_ENV=mainnet yarn hardhat summoner_set_fee_token \
  --remotenetwork 137 \
  --token 0x0000000000000000000000000000000000000000 \
  --amount 20000000000000000 \
  --network mainnet
# 0.02 WETH for mainnet to polygon
NODE_ENV=mainnet yarn hardhat summoner_set_fee_token \
  --remotenetwork 137 \
  --token 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 \
  --amount 20000000000000000 \
  --network mainnet
```

[home](/readme.md) > [deployment guide](/docs/deploy.md)
