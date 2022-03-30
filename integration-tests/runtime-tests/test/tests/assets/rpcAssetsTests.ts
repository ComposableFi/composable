import { CurrencyId } from '@composable/types/interfaces';
import { AnyNumber } from '@polkadot/types-codec/types';
import { expect } from 'chai';
import exp from 'constants';
import testConfiguration from './test_configuration.json';

describe('rpc.assets Tests', function () {
    if (!testConfiguration.enabledTests.rpc.enabled)
        return;
    it('rpc.assets.balanceOf Tests', async function () {
        if (!testConfiguration.enabledTests.rpc.balanceOf__success)
            this.skip();
        const asset_id = api.createType('CurrencyId', '123456789123456789');
        const publicKey = walletAlice.address;
        const result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        expect(result).to.be.a["bignumber"].that.equals('0');
    });

    it('rpc.assets.listAssets Tests', async function () {
        if (!testConfiguration.enabledTests.rpc.listAssets__success)
            this.skip();
        const result = await RpcAssetsTests.rpcListAssetsTest();
        expect(result).to.have.lengthOf(5); 
        result.every((i) => expect(i).to.have.all.keys('id','name'))
        expect(result.map(e=>(e.id.toNumber()))).to.include.members([ 1, 2, 3, 4, 129 ]);
    });
});

export class RpcAssetsTests {
    public static async rpcAssetsTest(asset_id: CurrencyId | AnyNumber, publicKey: string | Uint8Array) {
        return await api.rpc.assets.balanceOf(asset_id, publicKey);
    }

    public static async rpcListAssetsTest() {
        return await api.rpc.assets.listAssets();
    }
}
