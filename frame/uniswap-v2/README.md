# Uniswap v2

Uniswap is a protocol enabling the automted exchange of tokens.

## Overview 

"Uniswap is a protocol for automated token exchange on Ethereum. It is designed 
around ease-of-use, gas efficiency, censorship resistance, and zero rent 
extraction. It is useful for traders and functions particularily well as a 
component of other smart contracts which require guaranteed on-chain liquidity." 
- @haydenadams

This pallet is Composable's interpretation of Uniswap v2 for the Substrate 
ecosystem. This pallet has constant-product AMM alogrithm as explained in the 
["Uniswap Whitepaper"](https://hackmd.io/@HaydenAdams/HJ9jLsfTz?type=view) by 
@haydenadams.

## Workflow

To conduct exchanges with uniswap, a pool must exist. Pools can be created with 
the [`create`](Pallet::create) extrinsic. Once created, other operations may be 
conducted.

## References

["Uniswap Whitepaper"](https://hackmd.io/@HaydenAdams/HJ9jLsfTz?type=view) by 
@haydenadams

[Solidity code](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol) 
and [some functions](https://github.com/Uniswap/v2-periphery).

[Other interesting paper](https://raw.githubusercontent.com/runtimeverification/verified-smart-contracts/uniswap/uniswap/x-y-k.pdf)
