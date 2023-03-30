# Synergy with Centauri & XCVM

__Connecting Satellite protocols & orchestrating smart contracts across satellites__


At Composable, our mission is to promote a trustless and cohesive experience for web3. 
To achieve this, we have chosen to build a CosmWasm VM (Virtual Machine) 
as it is the most portable and robust solution available in the market. 
This decision is in line with our adoption of the Inter Blockchain Communication Protocol (IBC) 
as the basis of our transport layer via Centauri.

Initially, existing CW projects deployed on different networks can create a clone protocol on Picasso and tap into 
a new user- and liquidity base from DotSama. 
Centauri, our bridge implementing the IBC protocol to reach beyond Cosmos into DotSama and NEAR protocols,
is currently in its testnet stage and launching in Q2. 
Once it goes live, satellite protocols on Cosmos would then be able 
to utilize Centauri as a transport layer to communicate and transfer assets between them. 
This would lead to an array of protocols existing on multiple chains without any orchestration between them, 
easing the user's experience.

This is where the [Cross-chain Virtual Machine (XCVM)] comes in. 
The XCVM will be beneficial for developers 
who have deployed satellite protocols on multiple chains that are currently incapable of cross-chain communication. 
It is important to understand the difference here, multi-chain applications are often redeployed 
or slightly modified codebases that provide the same functionality across different chains and layers. 
While this may provide users with a familiar experience, it is not really cross-chain, 
since liquidity itself is fragmented due to the absence of an interoperability standard. 
With XCVM, applications are natively cross-chain as they can operate cohesively across multiple chains and layers.

In short, the XCVM abstracts complexity from the process of sending instructions to the Routing Layer, 
initiates call-backs into smart contracts, handles circuit failure such as network outages, provides finality, 
and perhaps most notably, allows for the deployment of natively cross-chain protocols and smart contracts. 
Throughout this experience, we enable users 
to tailor their experience by maximizing for a desired parameter while minimizing ecosystem-specific decision making. 
Furthermore, CosmWasm developers will have the ability 
to call into applications on Solidity based chains through use of our Rust SDK.

Our goal is not to create a new standard for cross-chain communication, 
which is already the subject of numerous projects. 
Instead, the XCVM will serve as the orchestration layer for existing bridging protocols.

[Cross-chain Virtual Machine (XCVM)]: https://github.com/ComposableFi/composable/blob/main/code/xcvm/SPEC.md
