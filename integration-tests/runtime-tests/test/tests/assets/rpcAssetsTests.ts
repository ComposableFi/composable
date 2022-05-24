import { SafeRpcWrapper } from "@composable/types/interfaces";
import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";

describe("rpc.assets Tests", function() {
  if (!testConfiguration.enabledTests.rpc.enabled)
    return;
  let api: ApiPromise;
  let walletBobPublicKey: string;

  before("Setting up tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletBob } = getDevWallets(newKeyring);
    walletBobPublicKey = devWalletBob.address;
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  /**
   * The `assets.balanceOf` RPC provides the amount a wallet holds of a specific asset.
   */
  it("rpc.assets.balanceOf Test #1", async function () {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success) this.skip();
    const PICA = api.createType("SafeRpcWrapper", 1) as SafeRpcWrapper;
    const PICA_amount = await RpcAssetsTests.rpcAssetsTest(api, PICA, walletBobPublicKey);
    expect(parseInt(PICA_amount.toString())).to.not.equals(0);
  });

  it("rpc.assets.balanceOf Test #2", async function () {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success) this.skip();
    const KSM = api.createType("SafeRpcWrapper", 4) as SafeRpcWrapper;
    const KSM_amount = await RpcAssetsTests.rpcAssetsTest(api, KSM, walletBobPublicKey);
    expect(parseInt(KSM_amount.toString())).to.be.equals(0);
  });

  it("rpc.assets.balanceOf Test #3", async function () {
    if (!testConfiguration.enabledTests.rpc.balanceOf__success) this.skip();
    const kUSD = api.createType("SafeRpcWrapper", 129) as SafeRpcWrapper;
    const kUSD_amount = await RpcAssetsTests.rpcAssetsTest(api, kUSD, walletBobPublicKey);
    expect(parseInt(kUSD_amount.toString())).to.be.equals(0);
  });
});
export class RpcAssetsTests {
  public static async rpcAssetsTest(apiClient: ApiPromise, assetId: SafeRpcWrapper, publicKey: string | Uint8Array) {
    return await apiClient.rpc.assets.balanceOf(assetId, publicKey);
  }
}
