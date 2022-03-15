import { expect } from 'chai';
import {ApiPromise} from "@polkadot/api";
import testConfiguration from './test_configuration.json';

describe('rpc.lending Tests', function() {
    // Check if group of tests are enabled.
    // if (!testConfiguration.enabledTests.query.enabled)
    // return;

  // repeat this block as needed for every test case defined in the class below.
  it('rpc.lending.getBorrowLimit Tests', async function() {
     const result = await RpcLendingTests.rpcLendingTest();
    expect(result).to.be.a["bignumber"].that.equals('0');
  });
});


export class RpcLendingTests {
    public static async rpcLendingTest() {
        const accountId = walletAlice.derive('/contributor-1/reward').publicKey;
        const result = await api.rpc.lending.getBorrowLimit("1", accountId);
        return result;
    }
}