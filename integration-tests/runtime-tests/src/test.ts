/**
 * Defines the tests to be run on the picasso parachain node.
 * All tests can be found in the ./tests/ folder.
 **/

import { QueryCrowdloanRewardsTests } from './tests/query/crowdloanRewards/queryCrowdloanRewardsTests';
import { TxCrowdloanRewardsTests } from './tests/tx/crowdloanRewards/txCrowdloanRewardsTests';
import { TxBondedFinanceTests } from "@composable/tests/tx/bondedFinance/txBondedFinanceTests";
import { runBefore, runAfter } from "@composable/utils/testSetup";
import { TxOracleTests } from "@composable/tests/tx/oracle/txOracleTests";


describe('Picasso Runtime Tests', function() {
  before(async function () {
    // Set timeout to 1 minute.
    this.timeout(60*1000);
    await runBefore();
  });

  after(async function() {
    // Set timeout to 1 minute.
    this.timeout(60*1000);
    await runAfter();
  });

  // Query Tests
  describe('Query Tests', function() {
    // Query Crowdloan Rewards Tests
    QueryCrowdloanRewardsTests.runQueryCrowdloanRewardsTests();
  });

  // TX Tests
  describe('TX Tests', function () {
    // TX Crowdloan Rewards Tests
    TxCrowdloanRewardsTests.runTxCrowdloanRewardsTests();

    // TX bondedFinance Tests
    TxBondedFinanceTests.runTxBondedFinanceTests();

    // TX Oracle Tests
    TxOracleTests.runTxOracleTests();
  });

  // RPC Tests
  describe('RPC Tests', function () {
    // No RPC tests implemented yet!
  });
});




