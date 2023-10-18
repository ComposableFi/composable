# Runtime Tests

Currently, it only has multihop tests. In order to run these on runtime-tests folder: 

``npm install`` 

``npm run test``

This will run the tests. Multihop tests cover two routes: 
1 - Kusama => Picasso => Composable => Picasso 
2 - Kusama => Picasso => Centauri => Osmosis

Tests validate the total issuance changes, next sequence assignment of ibc, escrow and fee balances and user balances.
Current runtime is around ~30 mins as it is mainly waiting for channel openings between centauri - osmosis and picasso - composable.

