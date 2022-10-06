# Overview

Client tools.

## TODO 
- move tools to `node` as cli.
- move para to account to lib which no reference to generated files (they hang/kill RA)
- allow to execute polkadot js decode reference (so you form message in pd.js and send link to and just it)
- add flag --defauls and suffix with port 443 in this case

## Example

cargo +nightly run sudo execute --suri ../../../../drr --call 0x290001010002100004000000000b0060defb740513000000000b0060defb74050006000700f2052a01381700e8030000e8030000009001000d0100040001009d20 --network dali --rpc wss://rpc.composablefinance.ninja:443