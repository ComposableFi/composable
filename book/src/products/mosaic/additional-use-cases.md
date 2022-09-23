# Additional Use Cases

---

## Powering Asset Swaps Through Mosaic

Mosaic connects with external systems, such as AMMs, to allow users to directly swap tokens for whatever token they 
desire within the realm of supported tokens. Mosaic also allows users to receive part of the transfer in the native 
token of the destination layer. This will enable users to immediately start working in the network they are swapping 
into via their newly acquired native tokens.

Composable has created a user interface that sits on top of all the AMMs through Mosaic to allow users to interact with 
contracts and swap tokens as desired. Mosaic enables users to swap tokens without needing to go to another protocol. 
This alleviates the complexity of navigating each AMM’s different interface. At the same time, Mosaic takes care of the 
complexity of navigating the AMM interface. For instance, a user that holds ETH on the mainnet and wants to buy on 
Sushiswap or Arbitrum can now use Mosaic to swap for the desired token.


## Mosaic’s Unique Ability to Allow LP-Native Token Swaps Across Chains and Layers

To further help LPs extract as much utility they can from their LP tokens, Phase 2 will allow for seamless LP token 
transfers across chains and layers, a novel feature not seen on any of the available bridge infrastructures in DeFi. 
LPs will transfer their LP tokens from chain-to-chain, helping to deepen liquidity for dApps offering better APYs at any
instance. This feature of Mosaic is exciting because LPs do not need to burn the LP tokens to transfer from one chain 
and then mint on the destination chain. They are able to move their LP tokens natively, in their current state. 

Mosaic also automatically recognizes that LP tokens are not created equal and accrue yield over time. 
Instrumental Finance leverages this novel feature and is building on the Composable SDK. Through this, 
LPs can easily migrate to any chain or layer offering a more attractive APY than they currently receive and begin
accruing higher yield there.

Mosaic Phase 2 provides a ratio for each token when users perform swapping functions. This becomes useful for LPs 
transferring LP tokens from Layer A, which does not need to have the same value on layer B. Phase 2 comes with advanced 
capabilities that can compute these differences on the fly guaranteeing each transfer gets the right amount of LP tokens
(equal to the value the user initially locked on the source layer).
