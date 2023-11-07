# Overview

Offchain part which gets data, runs algorithms, and sends transactions back.

## How to run

Next command:

```shell
cargo run --bin mantis --osmosis http://127.0.0.1:36657 --centauri http://127.0.0.1:26657 --neutron http://127.0.0.1:46657 --order-contract centauri1a23df5asd..a49a0  --cvm-contract centauri1a23df1123..a49a0 --simulate"1000ppica,10000pdemo,10000ptest" --wallet "kart ... dock"
```

### What above command does

1. Obtains CVM configuration from contract
2. For configured pool starts monitoring pool stats
3. Starts monitoring order contract
4. For orders 
5. Runs solver for order pairs
6. If there are match, sends solution for each pair
7. (optional) For given asses runs simulation with some randomness around with orders
8. Run cleans routine for timed out orders


# PLAN FOR TODAY

1. all code compiles
5. logs added
2. simulator runs on devnet
3. solver is called to mainnet
4. solutions are sent
5. contract is deployed to mainnet
6. sanity tests for solver
7. docs updates
8. TS updates for CVM
9. getting orders
10. positng solution
11. transform of data
12. mainnet