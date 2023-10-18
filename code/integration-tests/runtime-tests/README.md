# Runtime Tests
Tests for runtime.
## Prerequisites

In order to run the tests, centauri and osmosis binaries should be available on ~/go/ folder and should be executable.
To run tests, on runtime-tests folder: 

``npm install`` 

``npm run test``

This will run the tests. Multihop tests cover two routes: 
1 - Kusama => Picasso => Composable => Picasso 
2 - Kusama => Picasso => Centauri => Osmosis

Tests validate the total issuance changes, next sequence assignment of ibc, escrow and fee balances, and user balances.
The current runtime is around ~30 mins as it is mainly waiting for channel openings between centauri - osmosis and picasso - composable.

