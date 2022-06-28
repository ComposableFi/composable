# Overview

Spam protection of CPU resources is well documented and automated process in Substrate.

Protecting storage is less documented process which seems essential to keep network robust.

This document describes various aspect of storage to be protected, consistent means to do so, user and developer experience of protected storage. 

It overview some existing storage spam protection mechanics in different p2p computation models too.

## Existing spam protection models

### Solana

Solana network forces to pay block based rent or rent except amount for each data account. 
Some contacts allow only working with rent except accounts to protect users contracts from so fail.
Rent amount is voted by validators. Rent depends on size of data.

### P2P Storage with only small part of nodes stores replicas

Overall, if node is interested in data it stores it.
Interest to store data can be payed with some deposit.
Proves of storage are obtained from nodes which where payed to store data. 
Payed node must pin (copy and share on demand) specific data.
Payment to store depends on size of data at least.

Examples are Arweave, Filecoin, Sia.

### Substrate 

In substrate user accounts to pay rent and has some safety from being deleted from chain.

### Solana protocols

Some specific contracts take it rent on top of Solana too. 
These server as gates to public lists. 
Examples would be Order Book DEX Serum, which takes entry to create pool of 3 SOL (600) used.
It prevent contract to be slow and polluting public list from garbage pairs

## Storage spam consequences

There are 2 consequences.

Many data items on chain will raise requirements for each node regarding size, so will not be able to choose speed only variants for storage.
So overall limiting nodes can run parachain.
Can if it will be economical to attack pulling whole storage, will have hard time to clean storage from attack and from valid data later.
So protecting storage is more important than CPU, as CPU attacks are only transient and easy fixed buy very hight multipliers.

Another one is performance and polluting shared search lists. 
Example, are long list of token pairs to choose DEX pool.

## Forms paying ― Rent/Deposit/Stake

Slashable stake. Until slashed and unlike rent, returned fully to staker during deletion.

Rent deposit is not return fully as it is used to pay rent.

Liquidity provider, in some form adds liquidity to market and locks it for some period.

## When can avoid paying for storage?

When count of storage items is finite. 
Example, set of well known currency pairs to form pools.
Rule does not work if set is extended with custom pool configurations or with custom currencies added via registry.

When storage item is added by root or council.
So storage is just permission based.

## Assets used as ED

ED can be expressed in native parachain currency or in other protocol assets.

Example of in protocol asset price, asset account would require ED priced in asset amount itself. That is how ORML designed.

### ORML

Each non native assets must pay ED in itself as per inherited design of ORML.
If asset has dust cost ED price, cannot create many accounts(spam).
So as protection from spam any asset added must be reasonably priceable to have ED defined.
This leads to some permissions requires to create assets and define prices.

## Proposal

Each permissionless storage item of very large potential combinatorial number of items must be accompanied with deposit of native token.

### Considerations

We can consider next set of guidelines applicable when producing rent/deposit based models of storage consumption spam protection:

- Use `rent/deposit` symbols in code base to allow easy to find relevant places
- Motivate user to delete dead storage items by paying back part of rent. Can make burn of storage item block dependant, so of chain observers can get prize for burn calls.
- Increase rent depending on how influential storage item is. Higher size or longer some user interface search list because, the higher price of rent is.
- Rent should be captured on storage item creation, so can gracefully handel reconfiguration of rent after runtime upgrade.
- All rent must be documented as part of pallets documentation. Alternative would be to have RPC API prefixed with `function-name/rent/` to return rent size in native currency for all extrinsic taking rent.
- If rent can be really big, add additional parameter into extrinsic indicating maximal rent user eager to pay for getting storage place holder (similar to slippage). This also would be good indicator to discover rent based dispatchables.
- Each pallet for each storage item has its own rent requirements, so if one pallet calls other pallet, 2 can take rent. So can so simplify can allow rent free call from one pallet to other.
- Can implement taking, so returning back can be implemented after some time.