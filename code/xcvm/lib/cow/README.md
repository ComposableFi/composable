# Overview 

Custom intention engine for multi-chain usage using [Cross-Chain Virtual Machine (XCVM)]. 
The goal of this project is to allow projects to create and maintain request, propose solution and pick solution to solve cross chain execution in most efficient manner.

[Cross-Chain Virtual Machine (XCVM)]: https://docs.composable.finance/products/xcvm


### Execute

Solution of problem is executed in 3 phases:
1. Transfer funds into user interpreter where solution says they should be by solution
2. Execute whitelisted cross chain program
3. Collect assets back from where solution left these into problem specifid location