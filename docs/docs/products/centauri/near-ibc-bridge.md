# NEAR IBC

Composable’s bridging team have been working in close collaboration with the NEAR team 
to bring the necessary components for a GRANDPA light client implementation on [NEAR Protocol](https://near.org/). 
To do this, we have been assisting NEAR’s core developers with the development of infrastructure-level improvements for:

1. Signature verification
2. Missing validity
3. Singular validation process
4. And more

All of these can be found in our NEAR enhancement proposals NEP-364, 
NEP-384 and 9 PRs which have been merged into NEAR’s codebase. 
These changes will pave the way for our light client implementation, bringing IBC to the NEAR ecosystem. 
Additionally, we have received a grant from the NEAR Foundation 
as a result of Composable establishing an IBC bridge to NEAR Protocol.

Below you will find a diagram detailing how the same components for Centauri’s Cosmos implementation can effectively 
be repurposed to extend functionality to new blockchains such as NEAR.

![centauri_stack](./images-centauri/centauri-stack.png)
Kusama ⬌ NEAR IBC bridge

Ultimately, Centauri is more aptly described as the trustless bridging hub used to connect light-client-enabled blockchains.
W ill become the transport layer for the Composable XCVM,
facilitating cross-ecosystem orchestration of information and assets. 

Ecosystems such as zkSync and Starkware are examples of 2 mature ecosystems that could greatly benefit from IBC and 
already have the required infrastructure needed to support a light client. 
In the future, we plan to build within such ecosystems, ultimately working towards a future where 
all light client-enabled blockchains are interconnected through IBC.

