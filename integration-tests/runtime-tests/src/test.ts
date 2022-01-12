/**
 * Defines the tests to be run on the picasso parachain node.
 * All tests can be found in the ./tests/ folder.
 **/

import { QueryCrowdloanRewardsTests } from './tests/query/crowdloanRewards/queryCrowdloanRewardsTests';
import { QuerySystemAccountTests } from './tests/query/system/querySystemAccountTests';
import { QueryTokenTests } from './tests/query/tokens/queryTokenTests';
import { TxCrowdloanRewardsTests } from './tests/tx/crowdloanRewards/txCrowdloanRewardsTests';



// Query Tests

// Query.System.Account Tests
QuerySystemAccountTests.runQuerySystemAccountTests();

// Query.Token Tests
QueryTokenTests.runQueryTokenTests();

// Query Crowdloan Rewards Tests
QueryCrowdloanRewardsTests.runQueryCrowdloanRewardsTests();


// TX Tests

// TX Crowdloan Rewards Tests
TxCrowdloanRewardsTests.runTxCrowdloanRewardsTests();

// RPC Tests
