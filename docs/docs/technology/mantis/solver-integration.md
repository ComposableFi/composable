# Solver Integration

Anyone can be a solver with Composable, though permission from Composable is required in initial phases. Solvers are offchain nodes with an on Composable chain wallet. Solvers monitor several chains, run algorithms to find the best solution for intention, and send solution proposals to the Composable chain.

To be a solver, one must make an address on Composable’s Cosmos chain. Addresses can be owned by contract or a person’s machine/bot.

Solver nodes will run well on a machine with 2 Core CPU, 4 GB RAM, and 1 GB high low latency internet connection with Linux x86_64 installed.

Interested in participating as a solver? Join our solver channel [here](https://t.me/+Z69AYRzVTLVhNTk5).

Solvers are able to onboard with MANTIS via the following steps:

## Solver Setup

### Requirements 
1. A Cosmos compatible wallet holding PICA tokens on a valid Composable/Centauri chain address (Ex. Address format: centauri124l2ly8rgm4wjqgs50zzkzznxvqvve27uf6jr5) 

2. Clone the [MANTIS repository](https://github.com/ComposableFi/cvm/tree/main/mantis) and cargo to build mantis-node. As an alternative, you can use `nix run "github:ComposableFi/cvm#mantis-node -- `

3. An example of command line parameters used to run the solver can be found [here](https://github.com/ComposableFi/env/blob/a4bfeef449b5786f0d99f45e38a9acc306980fbb/flake.nix#L57)
(Note: the `--simulate` flag is optional)

An example of problem formatting that will be broadcast to solvers may be found [here](https://github.com/ComposableFi/composable/blob/206e0da16a3543c6212f845372c926fcf49ecf21/docs/docs/technology/mantis/tutorial.md). 

Additional information for troubleshooting can be found on our Github [here](https://github.com/ComposableFi/composable/blob/206e0da16a3543c6212f845372c926fcf49ecf21/docs/docs/technology/mantis/solver-tutorial.md).

### Trading Venues and Tokens for Testing
For the scope of testing, trading will be limited to Osmosis DEX and Astroport across the pairs outlined below.

| **Osmosis** | **Neutron** |
| -------- | -------- |
| PICA/OSMO   | NTRN/ATOM  |
| ATOM/OSMO   |     -     |
| NTRN/OSMO   |     -    |

Problems will be broadcasted in sizes ranging across $1, $5, $10 and $100.

### Rewards and Incentives
Solvers participating in testnet will be awarded an incentive of $500 of PICA in addition to rewards calculated via activity during testing. The reward calculation for activity will be allocated as a percentage of funds processed through MANTIS (ie. funds exchanged via solutions to the intents that are broadcasted during testing). 

A scoring system has been implemented to rank solutions submitted via MANTIS based on a reference score and quality of solution, where `rewards = observedQuality - referenceScore`. The breakdown of this reward calculation may be referred to in the “Solver Rewards” section [here](../mantis/solvers-solutions.md).
