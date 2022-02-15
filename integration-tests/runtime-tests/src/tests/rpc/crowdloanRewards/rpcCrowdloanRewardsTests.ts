/* eslint-disable no-trailing-spaces */
import { expect } from 'chai';


export class RpcCrowdloanRewardsTests {
  /**
   * 
   */
  public static runRpcCrowdloanRewardsTests() {
    describe('rpc.crowdloanRewards.account Tests', function () {
      it('STUB', async () => {
        const accountId = walletAlice.derive('/contributor-1/reward').publicKey;
        const result = await RpcCrowdloanRewardsTests.rpcCrowdloanRewardsTest(accountId);
        expect(result).to.be.a["bignumber"].that.equals('0');
      });
    });
  }

  /**
   * 
   */
  private static async rpcCrowdloanRewardsTest(accountId: string | Uint8Array) {
    return await api.rpc.crowdloanRewards.amountAvailableToClaimFor(
      accountId,
    );
  }
}

// Uncomment to debug
// RpcCrowdloanRewardsTests.runRpcCrowdloanRewardsTests();
