# Overview

This contract implements `Order` instruction of CVM via (CoW)(https://en.wikipedia.org/wiki/Coincidence_of_wants).
It integrates CoW with CFMM cross chain execution found by Solvers.

## General flow


Some users request exchange some assets to other assets.
Theirs tokens were transferred via well known bridges to here. 

User sends transactions containing order, describing amounts of assets they have(give) and want(take).
Additionally they describe if they allow partial fill and timeout.
Both assets must be local and registered in CVM.
If target wanted out asset is just bridged, transfer path must be provided. 

Solvers read all user orders on chain, and propose solutions to do CoW amid orders (including order from solver which may come with solution),
and cross chain CFMM for the rest. 
Each solver account has one and only one solution per pair. So solver can send and resend solution to replace old one for specific pair.

Solution with maximum volume is accepted, and should settle on second largest volume (second bid auction). 

Bidders reserve amounts as rewards to pay for percentage of volume solved via their preferred CFMM.

More details semantics will be described in whitepaper. including Solver operations.

## Implementation

Current implementation is for 1 solution for one user, 1 to 1 asset, permissioned solvers without collateral.

### Bidding

Bidder call this contract with CFMM identifier and percentage of volume they bid. 
This contract instantiates CVM executor per CFMM and transfer amount to it, and tracks percentage per bidder.
When bidder joins, percentage averaged appropriately, with always larger bid for same volume.
For each accepted route, Solver is delegated with relevant amount to withdraw.



