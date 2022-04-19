import { SafeRpcWrapper } from '@composable/types/interfaces';
import { expect } from 'chai';
import testConfiguration from './test_configuration.json';

describe('rpc.assets Tests', function() {
  if (!testConfiguration.enabledTests.rpc.enabled)
    return;
  it('rpc.assets.balanceOf Tests', async function() {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success)
        this.skip();
    const publicKey = walletBob.address;
    const PICA = api.createType('SafeRpcWrapper', 1) as SafeRpcWrapper;
    const PICA_amount = await RpcAssetsTests.rpcAssetsTest(PICA, publicKey);
    expect(parseInt(PICA_amount.toString())).to.not.equals(0);
    const KSM = api.createType('SafeRpcWrapper', 4) as SafeRpcWrapper;
    const KSM_amount = await RpcAssetsTests.rpcAssetsTest(KSM, publicKey);
    expect(parseInt(KSM_amount.toString())).to.be.equals(0);
    const kUSD = api.createType('SafeRpcWrapper', 129) as SafeRpcWrapper;
    const kUSD_amount = await RpcAssetsTests.rpcAssetsTest(kUSD, publicKey);
    expect(parseInt(kUSD_amount.toString())).to.be.equals(0);
  });
});

export class RpcAssetsTests {
  public static async rpcAssetsTest(assetId: SafeRpcWrapper, publicKey: string | Uint8Array) {
    return await api.rpc.assets.balanceOf(assetId, publicKey);
  }
}
