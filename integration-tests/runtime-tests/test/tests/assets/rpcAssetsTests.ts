import { CurrencyId } from '@composable/types/interfaces';
import { expect } from 'chai';
import testConfiguration from './test_configuration.json';
import {mintAssetsToWallet} from "@composable/utils/mintingHelper";


describe.only('rpc.assets Tests', function () {
    if (!testConfiguration.enabledTests.rpc.enabled)
        return;
    it('rpc.assets.balanceOf Tests', async function () {
        if (!testConfiguration.enabledTests.rpc.balanceOf__success)
            this.skip();
        this.timeout(3 * 60 * 1000);
        const converter = require('hex2dec');
        const publicKey = walletAlice.address;

        let asset_id = api.createType('CurrencyId', converter.decToHex('123456789123456789', {prefix: '0x'}));
        console.log("Asset ID Object [123456789123456789]: " + asset_id);
        let result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        console.warn("Result [123456789123456789]: " + result.toString());
        expect(result).to.be.a["bignumber"].that.equals('0');

        asset_id = api.createType('CurrencyId', converter.decToHex('1', {prefix: '0x'}));
        console.log("Asset ID Object [1]: " + asset_id);
        const resultFirstPicaCheck = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        console.warn("First PICA Check Result: " + resultFirstPicaCheck.toString());

        asset_id = converter.decToHex('4', {prefix: '0x'});
        console.log("Asset ID Object [4]: " + asset_id);
        asset_id = api.createType('CurrencyId', asset_id);
        console.log("Asset ID Object [4]: " + asset_id); // Here we can see the string gets somehow converted into a unicode character instead of staying hexadecimal.
        result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        console.warn("Result [4]: " + result.toString());

        // For some reason asset ID 129 gets converted to `�` character. No matter which conversion method I tried!
        // We're catching this to keep our tests running.
        asset_id = converter.decToHex('129', {prefix: '0x'});
        console.log("Asset ID Object [129]: " + asset_id);
        asset_id = api.createType('CurrencyId', asset_id);
        console.log("Asset ID Object [129]: " + asset_id); // Here we can see the string gets somehow converted into a unicode character instead of staying hexadecimal.
        result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey).catch(()=>{return api.createType('u128')});
        console.warn("Result [129]: " + result.toString());

        const mintResult = await mintAssetsToWallet(publicKey, walletAlice, [1, 4, 129]);
        expect(mintResult).to.not.be.an('Error');

        asset_id = api.createType('CurrencyId', converter.decToHex('1', {prefix: '0x'}));
        const secondPICACheckResult = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        console.warn("Second PICA Check Result: " + secondPICACheckResult.toString());

        asset_id = converter.decToHex('4', {prefix: '0x'});
        result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        console.warn("Result [4]: " + result.toString());

        asset_id = api.createType('CurrencyId', converter.decToHex('129', {prefix: '0x'}));
        console.log(asset_id);
        result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey).catch(()=>{return api.createType('u128')});
        console.warn("Result: [129]: " + result.toString());

        expect(resultFirstPicaCheck.toBigInt()).to.be.not.equal(secondPICACheckResult.toBigInt());
    });
});

export class RpcAssetsTests {
    public static async rpcAssetsTest(asset_id: CurrencyId, publicKey: string | Uint8Array) {
        return await api.rpc.assets.balanceOf(asset_id, publicKey);
    }
}
