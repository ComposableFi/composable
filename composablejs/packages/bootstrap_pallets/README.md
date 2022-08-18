# Bootstrap data for QA/ development of Picasso/Pablo
Please have a look at ```src/constants/config.json``` to adjust paramters and enable disable bootstrapping modules. By default dali types are used.
To disable bootstrapping enable bootstrap<module> flag to false in ```src/constants/config.json```
To mint assets to your address please add your wallet address, asset id and amount (in 12 decimals) in ```src/constants/config.json``` 
# Install Deps

```
yarn
```

# Setup ENV

```
RPC_URL=
GANACHE_URL=
# e.g CHAIN_NAME dali-rococo, dali-local, picasso, composable
CHAIN_NAME=
# if your chain uses a different sudo key please provide it's seed
SUDO_SEED=
```

# Start

```
yarn start
```
# Prettier
```
yarn prettier
```