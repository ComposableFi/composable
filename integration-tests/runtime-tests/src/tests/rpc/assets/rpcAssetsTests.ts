/* eslint-disable no-trailing-spaces */
import { CurrencyId } from '@composable/types/interfaces';
import { ApiPromise } from '@polkadot/api';
import { AnyNumber } from '@polkadot/types-codec/types';
import { expect } from 'chai';


export class RpcAssetsTests {
    /**
     * 
     */
    public static runRpcAssetsTests() {
        describe('rpc.assets.balanceOf Tests', function () {
            it('STUB', async () => {
                const asset_id = api.createType('CurrencyId', '123456789123456789');
                const publicKey = walletAlice.address;
                const result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
                expect(result).to.be.a["bignumber"].that.equals('0');
            });
        });
    }

    /**
     * 
     */
    private static async rpcAssetsTest(asset_id: CurrencyId | AnyNumber, publicKey: string | Uint8Array) {
        return await api.rpc.assets.balanceOf(asset_id, publicKey);
    }
}

// Uncomment to debug
// RpcAssetsTests.runRpcAssetsTests();
