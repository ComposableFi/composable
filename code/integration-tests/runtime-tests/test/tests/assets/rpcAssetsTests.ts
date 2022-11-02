import { SafeRpcWrapper } from "@composable/types/interfaces";
import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";

describe("[SHORT] rpc.assets Tests", function () {
  if (!testConfiguration.enabledTests.rpc.enabled) return;
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

  it("rpc.assets.listAssets Tests", async function () {
    if (!testConfiguration.enabledTests.rpc.listAssets__success) this.skip();
    const result = await RpcAssetsTests.rpcListAssetsTest(api);
    result.every(i => expect(i).to.have.all.keys("id", "name", "decimals", "foreignId"));
    expect(result.map(e => e.id.toNumber())).to.include.members([
      // These are the assets to be included on the first release
      1, 4, 5, 129, 130, 131
    ]);
    expect(result.map(e => hex_to_ascii(e.name.toString()))).to.include.members([
      // These are the assets to be included on the first release
      "PICA",
      "KSM",
      "PBLO",
      "kUSD",
      "USDT",
      "USDC"
    ]);
    result
      .map(e => e.foreignId.toHuman())
      .filter(Boolean)
      .every(i => expect(i).to.have.all.keys("parents", "interior"));
    // These assets will exist as checked before
    const PICA = result.find(e => hex_to_ascii(e.name.toString()) === "PICA")!;
    const KSM = result.find(e => hex_to_ascii(e.name.toString()) === "KSM")!;
    expect(PICA.id.toNumber()).to.equal(1);
    expect(KSM.id.toNumber()).to.equal(4);
  });
});

class RpcAssetsTests {
  public static async rpcAssetsTest(apiClient: ApiPromise, assetId: SafeRpcWrapper, publicKey: string | Uint8Array) {
    return await apiClient.rpc.assets.balanceOf(assetId, publicKey);
  }

  public static async rpcListAssetsTest(apiClient: ApiPromise) {
    return await apiClient.rpc.assets.listAssets();
  }
}

function hex_to_ascii(str1: string) {
  const hex = str1.toString();
  let str = "";
  //skip 0x
  for (let n = 2; n < hex.length; n += 2) {
    str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
  }
  return str;
}
