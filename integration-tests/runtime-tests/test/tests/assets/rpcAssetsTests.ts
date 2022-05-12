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

<<<<<<< HEAD
describe("rpc.assets Tests", function() {
  if (!testConfiguration.enabledTests.rpc.enabled)
    return;
  let api: ApiPromise;
  let walletBobPublicKey: string;

  before("Setting up tests", async function() {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletBob } = getDevWallets(newKeyring);
    walletBobPublicKey = devWalletBob.address;
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  /**
   * The `assets.balanceOf` RPC provides the amount a wallet holds of a specific asset.
   */
  it("rpc.assets.balanceOf Test #1", async function() {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success)
      this.skip();
    const PICA = api.createType("SafeRpcWrapper", 1) as SafeRpcWrapper;
    const PICA_amount = await RpcAssetsTests.rpcAssetsTest(api, PICA, walletBobPublicKey);
    expect(parseInt(PICA_amount.toString())).to.not.equals(0);
  });

  it("rpc.assets.balanceOf Test #2", async function() {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success)
      this.skip();
    const KSM = api.createType("SafeRpcWrapper", 4) as SafeRpcWrapper;
    const KSM_amount = await RpcAssetsTests.rpcAssetsTest(api, KSM, walletBobPublicKey);
    expect(parseInt(KSM_amount.toString())).to.be.equals(0);
  });

  it("rpc.assets.balanceOf Test #3", async function() {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success)
      this.skip();
    const kUSD = api.createType("SafeRpcWrapper", 129) as SafeRpcWrapper;
    const kUSD_amount = await RpcAssetsTests.rpcAssetsTest(api, kUSD, walletBobPublicKey);
    expect(parseInt(kUSD_amount.toString())).to.be.equals(0);
  });

  it('rpc.assets.listAssets Tests', async function () {
    if (!testConfiguration.enabledTests.rpc.listAssets__success)
        this.skip();
    const result = await RpcAssetsTests.rpcListAssetsTest(api);
    expect(result).to.have.lengthOf(5); 
    result.every((i) => expect(i).to.have.all.keys('id','name'))
    expect(result.map(e=>(e.id.toNumber()))).to.include.members([ 1, 2, 3, 4, 129 ]);
    expect(result.map(e=>(hex_to_ascii(e.name.toString())))).to.include.members([ 'PICA', 'LAYR', 'CROWD_LOAN', 'KSM', 'kUSD' ]);
  });
});

export class RpcAssetsTests {
  public static async rpcAssetsTest(apiClient: ApiPromise, assetId: SafeRpcWrapper, publicKey: string | Uint8Array) {
    return await apiClient.rpc.assets.balanceOf(assetId, publicKey);
  }
  public static async rpcListAssetsTest(apiClient: ApiPromise) {
    return await apiClient.rpc.assets.listAssets();
  }
<<<<<<< HEAD
=======
  
=======
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
>>>>>>> d9b5d7e5 (resolve conflicts)
>>>>>>> 2e645290 (rebased upstream)
}

function hex_to_ascii(str1: string)
 {
	var hex  = str1.toString();
	var str = '';
    //skip 0x
	for (var n = 2; n < hex.length; n += 2) {
		str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
	}
	return str;
 }
