# Overview

This contract implements `Order` instruction of CVM.
It integrates CoW with CFMM cross chain execution found by Solvers.

## General flow


Some users request exchange some assets to other assets.
Theirs tokens were transferred via well known bridges to here. 

User sends transactions containing order, describing amounts of assets they have(give) and want(take).
Additionally they describe if they allow partial fill and timeout.

Solvers read all user orders on chain, and propose solutions to do CoW amid orders (including order from solver which may come with solution),
and cross chain CFMM for the rest. 

Solution with maximum volume is accepted, and should settle on second largest volume (second bid auction). 

Bribers reserve amounts as rewards to pay for percentage of volume solved via their preferred CFMM.

More details semantics will be described in whitepaper. including Solver operations.

## Implementation

Current implementation is for 1 solution for one user, 1 to 1 asset, with explicit approval for CFMM route.

Bribers are owner of their CVM Executor instance. 
The transfer amounts to it and set CFMM identifiers for routing via which they are to bribe.



