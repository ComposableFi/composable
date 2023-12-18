## How to run solver node

Solver observes user orders on chain, and find matches, and posts solution so users get exchange. 
If solver does not find match, formulates cross chain route.

## Prerequisites

You know how to build simple rust cargo project from git repositories or how to use nix.

You know basics of blockchain, transactions, gas and easy can catch up with Cosmos specifics as needed based on Cosmos ecosystem docs.

## Prepare

1. Get wallet with PICA on Centauri

2. Clone https://github.com/ComposableFi/cvm and `cargo run --bin mantis`. You can use `nix run "github:ComposableFi/cvm#mantis" --` as alternative.

## Run

Here is example of command line parameters used by solver run by us https://github.com/ComposableFi/env/blob/e9eaa098e103cb16f033e2abc26d09d79823da26/flake.nix#L49 .

`--simulate` is optional, so is example how to provision own liquidity.

## Troubleshoot

In case of failure - read error. Usually it is:
1. timeout/rate limit by RP
2. bad wallet - no PICA or bad mnemonic
3. Node not hosted with process auto restart policy on failure as it should be
4. latency, if you picked slow RPC or host node on slow network - other solvers will be faster and solve instead o you

## GTP bot

You may consider train the bot by asking questions here https://discord.com/channels/828751308060098601/1163404253537247283

## How it works

- [degen math](./degen-math.md)
- [problem solver flow](./problem-solver-flow.md)
