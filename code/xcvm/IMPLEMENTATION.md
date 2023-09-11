
## CVM Docs for implementation level

### DoD
Given that the user has PICA, DOT on Centauri 
- the user should be able to swap on Osmosis from PICA / DOT -> OSMO with a single tx using CVM

### Asnwers


Joon(ART)
  < 1 minute ago
1. example payload

- live TX https://explorer.nodestake.top/composable/tx/F4BDDC1F0D502E55F5B413C139C7DCE5608C1662B5D7919CAD9BA889B38C8861
- same in raw JSON https://github.com/ComposableFi/composable/pull/4091/files#diff-79ad53577207629481229b103ab8a8abc18cc1535206aa36b664bdff2c7a5215
- how i send on devnet via cli https://github.com/ComposableFi/composable/blob/866f40ba5558ef9c1a1adc6209e726c69f3c492a/inputs/notional-labs/composable-centauri/flake-module.nix#L377

2.1 which events to look for 
will share link to Rust docs
2.2. how to query the state

will share link to schmea with improved docs and generator

3. ditto
@fl-y ????????????????
   
5. how to tell whether funds are stuck
in progress

### Required for DoD

- How to execute the CW contract on Centauri, as in an example payload to an RPC endpoint

-> @dzmitry-lahoda

<p>How to query the CW contract on, </p>
- Centauri, to verify the IBC tx has been initiated to go over to Osmosis. Which events to look for or how to query the state

-> @dzmitry-lahoda

- Osmosis, to verify the swap has happened. Which events to look for or how to query the state.

-> @dzmitry-lahoda

- How to check whether there was an error during the process to tell the user their funds are stuck.

-> @dzmitry-lahoda
