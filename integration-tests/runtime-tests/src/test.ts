/**
 * Defines the tests to be run on the picasso parachain node.
 * All tests can be found in the ./tests/ folder.
 **/

import { QueryCrowdloanRewardsTests } from './tests/query/crowdloanRewards/queryCrowdloanRewardsTests';
import { TxCrowdloanRewardsTests } from './tests/tx/crowdloanRewards/txCrowdloanRewardsTests';
import { TxBondedFinanceTests } from "@composable/tests/tx/bondedFinance/txBondedFinanceTests";



// Query Tests

// Query Crowdloan Rewards Tests
QueryCrowdloanRewardsTests.runQueryCrowdloanRewardsTests();


// TX Tests

// TX Crowdloan Rewards Tests
TxCrowdloanRewardsTests.runTxCrowdloanRewardsTests();

// TX bondedFinance Tests
TxBondedFinanceTests.runTxBondedFinanceTests();

// RPC Tests
