import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { AnyNumber } from "@polkadot/types-codec/types";
import { BTreeMap, u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces";
import {
  ComposableTraitsStakingLockLockConfig,
  ComposableTraitsStakingRewardConfig
} from "@composable/types/interfaces";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";

/**
 * Extrinsic Tests for Staking Rewards Pallet
 */
describe.only("tx.stakingRewards Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let poolOwnerWallet: KeyringPair, sudoKey: KeyringPair;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    poolOwnerWallet = devWalletAlice.derive("/stakingRewards/poolOwner");
  });

  before("Providing assets for tests", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, poolOwnerWallet, sudoKey, [1]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("query.system.account Tests", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("Wallet balance check should be >0", async function () {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();

      // Here we make our request
      const {
        data: [result]
      } = await QuerySystemAccountTests.createRewardPool(
        api,
        poolOwnerWallet.publicKey,
        1,
        10,
        api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig", null),
        api.createType("ComposableTraitsStakingLockLockConfig", {
          durationPresets: api.createType("BTreeMap<u64, Perbill", null)
        })
      );
      console.log("res", result);
    });
  });
});

/**
 * If the test file is quite small like this one, we often have the request functions within the same file.
 * Though for big files, like `txOracleTests.ts`, we outsource the tests handlers into an extra subdirectory
 * called `testHandlers`.
 */
export class QuerySystemAccountTests {
  /**
   * Sends a requests for `query.system.account` using the provided `walletAddress`
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param owner
   * @param assetId
   * @param endBlock
   * @param rewardConfigs
   * @param lock
   */
  public static async createRewardPool(
    api: ApiPromise,
    owner: AccountId32 | Uint8Array,
    assetId: AnyNumber,
    endBlock: AnyNumber,
    rewardConfigs: BTreeMap<u128, ComposableTraitsStakingRewardConfig>,
    lock: ComposableTraitsStakingLockLockConfig
  ) {
    const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
      owner: owner,
      assetId: assetId,
      endBlock: endBlock,
      rewardConfigs: rewardConfigs
    });
    return await api.tx.stakingRewards.createRewardPool(poolConfig);
  }
}
