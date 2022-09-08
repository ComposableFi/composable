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
    expect(result).to.have.lengthOf(17);
    result.every(i => expect(i).to.have.all.keys("id", "name"));
    expect(result.map(e => e.id.toNumber())).to.include.members([
      1, 2, 3, 4, 5, 129, 130, 131, 132, 133, 134, 1001, 1002, 1004, 1005, 2001, 2005
    ]);
    expect(result.map(e => hex_to_ascii(e.name.toString()))).to.include.members([
      "PICA",
      "LAYR",
      "CROWD_LOAN",
      "KSM",
      "PBLO",
      "kUSD",
      "USDT",
      "USDC",
      "wBTC",
      "wETH",
      "aUSD",
      "xPICA",
      "xLAYR",
      "xKSM",
      "xPBLO",
      "PICA_STAKE_FNFT_COLLECTION",
      "PBLO_STAKE_FNFT_COLLECTION"
    ]);
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
