## **How to interact with sushiswap liquidity providing strategy**

#### 1. Quote price from Sushiswap Liquidity Pools

_Get price of token B relative to token A_

- `token-address-a` - address of token A
- `amount-token-a` - amount of token A user want to deposit
- `token-address-b` - address of token B
- `network` - network where **MosaicVault** is deployed

**returns**: Equal amount of value of token B

> NODE_ENV=<env_file> yarn hardhat sushiswap_quote_price --token-address-a <token_address> --amount-token-a <amount_of_tokens> --token-address-b <token_address> --network <network_name>

Example:
`NODE_ENV=mainnet yarn hardhat sushiswap_quote_price --token-address-a 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 --amount-token-a 100 --token-address-b 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 --network mainnet`

#### 2. Provide liquidity for Sushiswap Liquidity Pools

- `token-address-a` - address of token A
- `amount-token-a` - amount of token A admin want to deposit
- `token-address-b` - address of token B
- `amount-token-b` - amount of token B admin want to deposit
- `deadline` - number of seconds trade is available before expiry
- `network` - network where **MosaicVault** is deployed

> NODE_ENV=<env_file> yarn hardhat add_sushi_lp --token-address-a <token_address> --amount-token-a <amount_of_tokens> --token-address-b <token_address> --amount-token-b <amount_of_tokens> --deadline <no_of_seconds> --network <network_name>

Example:
`NODE_ENV=mainnet yarn hardhat add_sushi_lp --token-address-a 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 --amount-token-a 100 --token-address-b 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 --amount-token-b 24260387237 --deadline 3000 --network mainnet`

[home](/readme.md) > [deployment guide](/docs/deploy.md)
