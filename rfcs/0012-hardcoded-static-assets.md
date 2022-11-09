# Overview

This RFC argues usage of hardcoded assets routes, identifiers, and metadata are generally reasonable.  

It describes how one would initialize, update and override static assets.

Hopefully, that description will be good enough to favor hardcoded assets as a viable approach for asset engineering.

## Prerequisites

Reader clear on general consequences of code as data, data as code, and code vs. data approaches for configuration and other small data parameters.

A reader of this RFC knows the mechanics of genesis, runtime upgrades, SCALE, and runtime configuration in Substrate.

Previous RFC explains the data storage approach. This RFC does not bother to repeat. Data migration is also a data storage approach. 

Genesis is a hardcoded approach.

## What are assets?

An asset is a local number identifier. Also, it may be foreign (has a remote reserve location that mints that asset),  sufficient (it has a well-known gas price, a fee payable), and metadata (symbol, name, decimals).

Examples, `PICA = (1, Here, 1/1, PICA, PICA, 12)`, and `USDT = (130, (Statemine, 1984), 1/67, USDT, Tether USD,  6)`. 

## What is a hardcode?

Runtime code base encodes an asset as code. It puts examples literally into the Rust codebase.

## What if an asset changes?

We upgrade runtime.

RPC and extrinsic check runtime configured asset state before using 
hardcode.

Foreign locations and symbols will not change.

## Who can update hardcoded assets?

The root can update hardcoded assets. Such assets are permissioned.

## Who uses hardcoded assets?

Acala and Moonbeam use them.

## How do we encode static assets?

Rust `const` and `macro` allow encoding assets boilerplate-free and bug-free, and consistent (RPC, BYOG, XCM). This encoding simplifies testing, QA, and release management.

## How do we release static assets?

Each Picasso protocol encodes relevant assets. We do often release to bring features and fixes to users along with new assets.

## Why would one be using hardcoded assets?

You use well-known priceable assets.
You are fully permissioned.
You will use the best permissionless (data storage) assets infrastructure and design available when you decide to be permissionless.

## Conclusion

Releasing permissioned well-known assets along with the runtime release is a viable approach, short term, and long term.
