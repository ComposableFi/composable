# Overview

In memory tests running several parachains and relay chain.

## TODO

Test Assets TX payments 
https://github.com/AcalaNetwork/Acala/commit/88193d6b3f636e483a916a355e1db7a89d38a60b#diff-79521dd3ae35d7e19dff40c49b325850fbad442c1f09d742cf8f03306ef77188

Ensure trapped assets are to claim
https://github.com/AcalaNetwork/Acala/commit/f40e8f9277fe2fabefd4b51d8d2cfd97f088f3b1#diff-4918885dbae3244dd19ee256ec2d575908d8b599007adc761b8651082c4b3288

Add barrier and ED tests

https://github.com/AcalaNetwork/Acala/commit/7a1b02961a9d795d1a62e9ab6e43c5735e244e6f#diff-4918885dbae3244dd19ee256ec2d575908d8b599007adc761b8651082c4b3288R606

Run not only Kusama spec, but polkadot too

https://github.com/AcalaNetwork/Acala/commit/c4f40d1bfba1405c775ba87f57dd17d309290403#diff-9514ad9ceca0c0b988d2614e422ce1366ae94b403f6e1513a47315b7fcb9c21a

Unignore all tests and fix them (broken on some upgrade). 

Make all tests using calculated fees our of types, not hardcoded (can be used in future to build RPC for fee calculator).

In each reserver transfer test ensure sibling accounts, para accounts are set up well and total supplys always same in the end.

Remove all usage of XTokens as these violate XCM guidlines and do not work well.

Ensure that assets from pallets encoded like (parent = 0, pallet = 44, index = 1232132).