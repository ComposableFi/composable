import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import {
  ComposableTraitsStakingRewardPool,
  ComposableTraitsStakingStake,
  CustomRpcBalance
} from "@composable/types/interfaces";
import { Option, u128, u64, Vec } from "@polkadot/types-codec";
import BN from "bn.js";
import { before } from "mocha";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import {
  getClaimOfStake,
  getNthStakerRewardAlternativeTry4BasedBlocktime,
  getNthStakerSharesPart,
  getPoolStartTime,
  getStakeReduction,
  getTotalRewardForCurrentTimestamp,
  verifyPoolClaiming,
  verifyPoolCreationUsingQuery,
  verifyPoolPotAddition,
  verifyPoolStaking,
  verifyPositionExtension,
  verifyPositionSplitting,
  verifyPositionUnstaking
} from "@composabletests/tests/stakingRewards/testHandlers/stakingRewardsTestHelper";
import { ITuple } from "@polkadot/types/types";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import BigNumber from "bignumber.js";

/**
 * Staking Rewards Pallet Tests
 *
 * Index:
 *
 * before:
 * - Variable setup
 * - Registering asset IDs in currency factory
 * - Minting assets
 *
 * Tests:
 * 1.1 I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with a single reward asset.
 * 1.2 I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with multiple reward assets.
 * 1.3 I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with a single duration preset.
 * 1.6 I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero time locks.
 * 1.7 I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero penalty locks.
 *
 * 2.1 I can, as pool owner, add rewards to staking rewards pool pot #1.1.
 * 2.2 Any user can add all reward assets to another staking rewards pool with multiple reward pots #1.2.
 * 2.3 Any user can add rewards to multiple staking pool at once
 *
 * 3.1 I can stake in the newly created rewards pool #1.1.
 *
 * 4.1 I can claim from the arbitrary asset pool created in #1.1 using the stake from #3.1 during the lock period.
 * // ToDo: Lock Period!
 *
 * 3.2 Another user can stake in the newly created rewards pool #1.1.
 * 3.3 I can stake in the newly created rewards pool #1.2.
 * 3.4 I can stake in the newly preconfigured PICA pool.
 * // ToDo: on-chain pool rewards configuration.
 * 3.5 I can stake in the preconfigured PBLO pool.
 * // ToDo: on-chain pool rewards configuration.
 * 3.6 I can stake in the newly created LP token pool #1.5.
 * 3.7 I can stake in the newly created pool #1.6 with 0 time locks.
 *
 * 4.2 I can claim from the arbitrary asset pool #1.1 using the stake from #3.2 during the lock period.
 * 4.3 I can claim from the arbitrary asset pool #1.2 using the stake from #3.3 after the lock period has ended.
 * 4.4 I can claim from the PICA pool using my stake from #3.34 after the lock period has ended.
 * // ToDo: on-chain pool rewards configuration.
 * 4.5 I can claim from the PBLO pool using my stake from #3.5 after the lock period has ended.
 * // ToDo: on-chain pool rewards configuration.
 * 4.6 I can claim from the LP token pool using my stake from #3.6 after the lock period has ended.
 * // ToDo: on-chain LP token reward pools.
 * 4.7 I can claim from the 0 time lock pool using my stake from #3.7
 * 4.8 I can claim from the 0 unlock penalty pool using my stake from #3.8.
 *
 * 5.1 I can extend the stake amount in pool #1.1 using the stake from #3.3
 * 5.2 I can extend the lock time in pool #1.1 using the stake from #3.1.
 *
 * 6.1 I can split my staking position into 2 separate positions.
 * 6.2 I can split my already split position again.
 *
 * 7.1 I can unstake my staking position before my lock period has ended and get slashed.
 * 7.2  I can unstake my staking position after the lock period has ended without getting slashed.
 * 7.3 I can unstake my staking position from the PICA pool after the lock period has ended.
 * // ToDo: on-chain pool configuration.
 * 7.4 I can unstake my staking position from the PBLO pool after the lock period has ended.
 * // ToDo: on-chain pool configuration.
 * 7.5 I can unstake my staking position from the LP token pool after the lock period has ended.
 * // ToDo; on-chain LP token reward pool.
 * 7.6 I can unstake my staking position from the 0 time lock pool w/o getting slashed.
 * 7.7 I can unstake my position from the 0 unlock penalty pool w/o getting slashed.
 * 7.8 I can unstake all split positions.
 *
 */
describe("[SHORT] Staking Rewards Pallet", function() {
  if (!testConfiguration.enabledTests.query.enabled) return;
  this.retries(0);
  this.timeout(2 * 60 * 1000);

  let api: ApiPromise;
  let sudoKey: KeyringPair,
    walletStaker: KeyringPair,
    walletStaker2: KeyringPair,
    walletPoolOwner: KeyringPair,
    walletRewardAdder: KeyringPair;
  let fNFTCollectionId1: u128,
    fNFTCollectionId2: u128,
    fNFTCollectionId3: u128,
    fNFTCollectionId4Pica: u128,
    fNFTCollectionId5Pblo: u128,
    fNFTCollectionId6: u128,
    fNFTCollectionId7: u128,
    fNFTCollectionId8: u128,
    fNFTCollectionId12: u128;
  let fNFTInstanceId1: u64,
    fNFTInstanceId2: u64,
    fNFTInstanceId3: u64,
    fNFTInstanceId4Pica: u64,
    fNFTInstanceId5Pblo: u64,
    fNFTInstanceId6: u64,
    fNFTInstanceId7: u64,
    fNFTInstanceId8: u64,
    fNFTInstanceId12: u64;
  let allSplitPositions: Vec<ITuple<[u128, u64, u128]>>;

  let inflationStake31: BigNumber,
    inflationStake32: BigNumber,
    inflationStake33_1: BigNumber,
    inflationStake33_2: BigNumber,
    inflationStake33_3: BigNumber,
    inflationStake37: BigNumber,
    inflationStake38: BigNumber,
    inflationStake312: BigNumber,
    inflationStake312_1: BigNumber,
    inflationStake312_2: BigNumber,
    inflationStake312_3: BigNumber;
  let blockNumAtStake31: number,
    blockNumAtStake32: number,
    blockNumAtStake33: number,
    blockNumAtStake37: number,
    blockNumAtStake38: number,
    blockNumAtFundsAddition3_12: number;

  const POOL_11_BASE_ASSET_ID = 10000;
  const POOL_11_SHARE_ASSET_ID = 10000001;
  const POOL_11_REWARD_ASSET_ID = 10001;
  const POOL_11_REWARD_RATE = Pica(1);
  let poolStartTime: number;
  let pool11startBlock: number, pool18startBlock: number;

  const POOL_12_BASE_ASSET_ID = 20000;
  const POOL_12_SHARE_ASSET_ID = 20000001;
  const POOL_12_REWARD_ASSET_ID_1 = 20001;
  const POOL_12_REWARD_ASSET_ID_2 = 20002;
  const POOL_12_REWARD_ASSET_ID_3 = 20003;
  const POOL_12_REWARD_RATE_1 = Pica(1_000);
  const POOL_12_REWARD_RATE_2 = Pica(5_000);
  const POOL_12_REWARD_RATE_3 = Pica(10_000);
  let stakeTimestampPool12;

  const POOL_13_BASE_ASSET_ID = 30000;
  const POOL_13_SHARE_ASSET_ID = 30000001;
  const POOL_13_REWARD_RATE = Pica(1);
  const POOL_13_REWARD_ASSET_ID = 30001;
  let stakeTimestampPool13;

  const POOL_14_BASE_ASSET_ID = 40000;
  const POOL_14_SHARE_ASSET_ID = 40000001;
  const POOL_14_REWARD_ASSET_ID = 40001;
  let stakeTimestampPool14;

  const POOL_15_PBLO_BASE_ASSET_ID = 50000;
  const POOL_15_PBLO_QUOTE_ASSET_ID = 50001;
  const POOL_15_SHARE_ASSET_ID = 21474836476;
  let stakeTimestampPool15;
  let pool15LpTokenId: u128, pool15PabloPoolId: u128;

  const POOL_16_BASE_ASSET_ID = 60000;
  const POOL_16_SHARE_ASSET_ID = 60000001;
  const POOL_16_REWARD_RATE = Pica(1);
  const POOL_16_REWARD_ASSET_ID = 60001;

  const POOL_17_BASE_ASSET_ID = 70000;
  const POOL_17_SHARE_ASSET_ID = 70000001;
  const POOL_17_REWARD_RATE = Pica(1);
  const POOL_17_REWARD_ASSET_ID = 70001;

  const POOL_18_BASE_ASSET_ID = 80000;
  const POOL_18_SHARE_ASSET_ID = 80000001;
  let POOL_18_REWARD_RATE = Pica(1_000);
  const POOL_18_REWARD_ASSET_ID_1 = 80001;
  const POOL_18_REWARD_ASSET_ID_2 = 80002;
  const POOL_18_REWARD_ASSET_ID_3 = 80003;

  const POOL_64_BASE_ASSET_ID = 640000;
  const POOL_64_SHARE_ASSET_ID = 640000001;
  const POOL_64_REWARD_ASSET_ID = 640001;

  const POOL_65_BASE_ASSET_ID = 650000;
  const POOL_65_SHARE_ASSET_ID = 650000001;
  const POOL_65_REWARD_ASSET_ID = 650001;

  const POOL_66_BASE_ASSET_ID = 660000;
  const POOL_66_SHARE_ASSET_ID = 660000001;
  const POOL_66_REWARD_ASSET_ID = 660001;

  const PICA_ASSET_ID = 1;
  const PBLO_ASSET_ID = 5;

  let stakingPoolId1: u128,
    stakingPoolId2: u128,
    stakingPoolId3: u128,
    stakingPoolId4: u128,
    stakingPoolId5: u128,
    stakingPoolId6: u128,
    stakingPoolId7: u128,
    stakingPoolId8: u128;

  before("Setting up the tests", async function() {
    // Getting connection & wallets
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletBob, devWalletEve } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletStaker = devWalletBob.derive("/test/staking-rewards/staker");
    walletStaker2 = devWalletBob.derive("/test/staking-rewards/staker2");
    walletRewardAdder = devWalletBob.derive("/test/staking-rewards/reward/adder");
    walletPoolOwner = devWalletEve.derive("/test/staking-rewards/owner");
  });

  before("Registering share asset IDs", async function() {
    const txs = [
      // Share asset IDs to register
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_11_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_12_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_13_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_14_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_15_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_16_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_17_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_18_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_64_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_65_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_66_SHARE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      // Staking asset IDs to register
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_11_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_12_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_13_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_14_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_16_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_17_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_18_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_64_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_65_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_66_BASE_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      // Reward asset IDs to register
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_11_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_12_REWARD_ASSET_ID_1, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_12_REWARD_ASSET_ID_2, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_12_REWARD_ASSET_ID_3, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_13_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_14_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_16_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_17_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_18_REWARD_ASSET_ID_1, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_18_REWARD_ASSET_ID_2, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_18_REWARD_ASSET_ID_3, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_64_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_65_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12)),
      api.tx.sudo.sudo(api.tx.assetsRegistry.updateAsset(POOL_66_REWARD_ASSET_ID, {
        parents: 0,
        interior: "Here"
      }, { n: 1, d: 1 }, 12))
    ];
    const { data: [result] } = await sendWithBatchAndWaitForSuccess(api,
      sudoKey,
      api.events.sudo.Sudid.is,
      txs,
      false
    );
    expect(result.isOk).to.be.true;
  });

  before("Providing funds", async function() {
    this.timeout(10 * 60 * 1000);
    await mintAssetsToWallet(
      api,
      walletStaker,
      sudoKey,
      [
        PICA_ASSET_ID, POOL_11_BASE_ASSET_ID, POOL_12_BASE_ASSET_ID, POOL_13_BASE_ASSET_ID, POOL_14_BASE_ASSET_ID,
        POOL_16_BASE_ASSET_ID, POOL_17_BASE_ASSET_ID, POOL_18_BASE_ASSET_ID, POOL_64_BASE_ASSET_ID, POOL_65_BASE_ASSET_ID, POOL_66_BASE_ASSET_ID
      ],
      Pica(10_000)
    );
    await mintAssetsToWallet(
      api,
      walletStaker2,
      sudoKey,
      [PICA_ASSET_ID, POOL_11_BASE_ASSET_ID, POOL_12_BASE_ASSET_ID, POOL_13_BASE_ASSET_ID, POOL_14_BASE_ASSET_ID,
        POOL_16_BASE_ASSET_ID, POOL_17_BASE_ASSET_ID, POOL_18_BASE_ASSET_ID, POOL_64_BASE_ASSET_ID, POOL_65_BASE_ASSET_ID, POOL_66_BASE_ASSET_ID],
      Pica(10_000)
    );
    await mintAssetsToWallet(
      api,
      walletPoolOwner,
      sudoKey,
      [
        PICA_ASSET_ID, POOL_11_REWARD_ASSET_ID
      ],
      Pica(100_000_000_000_000)
    );
    await mintAssetsToWallet(
      api,
      walletRewardAdder,
      sudoKey,
      [
        PICA_ASSET_ID, POOL_11_REWARD_ASSET_ID, POOL_12_REWARD_ASSET_ID_1, POOL_12_REWARD_ASSET_ID_2, POOL_12_REWARD_ASSET_ID_3, POOL_13_REWARD_ASSET_ID,
        POOL_14_REWARD_ASSET_ID, POOL_16_REWARD_ASSET_ID, POOL_17_REWARD_ASSET_ID, POOL_18_REWARD_ASSET_ID_1, POOL_64_REWARD_ASSET_ID, POOL_65_REWARD_ASSET_ID, POOL_66_REWARD_ASSET_ID],
      Pica(100_000_000_000_000_000)
    );
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  describe("1. Creation of reward pools.", function() {
    it("1.1  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with a single reward asset.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(18));
      const assetId = api.createType("u128", POOL_11_BASE_ASSET_ID);
      const durationPreset = {
        "0": 1000000000, // 1x default rate
        "12": 1250000000, // 1.25x for 12 seconds lock time
        "600": 1500000000, // 1.5x for 600 seconds lock time
        "1200": 2000000000 // 2x for 1200 seconds lock time
      };
      const unlockPenalty = 100_000_000;
      const shareAssetId = POOL_11_SHARE_ASSET_ID;
      const financialNftAssetId = 10000002;
      const minimumStakingAmount = Pica(10);
      poolStartTime = await getPoolStartTime(api, startBlock, currentBlockNumber);
      pool11startBlock = startBlock.toNumber();

      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId, // Asset to stake in pool
          startBlock: startBlock, // When pool allows start staking
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // Reward Asset ID
            "10001": {
              rewardRate: {
                period: "PerSecond",
                amount: POOL_11_REWARD_RATE
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: durationPreset
            },
            unlockPenalty: unlockPenalty
          },
          shareAssetId: shareAssetId,
          financialNftAssetId: financialNftAssetId,
          minimumStakingAmount: minimumStakingAmount
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultPoolConfig]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );

      // After waiting for our event, we make sure the defined staking asset id
      // is the same as the pool id.
      expect(resultPoolId).to.be.bignumber.equal(assetId);
      expect(resultPoolConfig.toString()).to.be.equal(poolConfig.toString());
      stakingPoolId1 = resultPoolId;

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId1,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.2  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with multiple reward assets.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const assetId = api.createType("u128", POOL_12_BASE_ASSET_ID);
      const amount1 = Pica(1_000);
      const amount2 = Pica(5_000);
      const amount3 = Pica(10_000);
      const durationPreset = {
        "600": "1200000000",
        "1200": "1500000000"
      };
      const unlockPenalty = 100_000_000;
      const shareAssetId = POOL_12_SHARE_ASSET_ID;
      const financialNftAssetId = 20000002;
      const minimumStakingAmount = Pica(10);
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "20001": {
              rewardRate: {
                period: "PerSecond",
                amount: amount1
              }
            },
            "20002": {
              rewardRate: {
                period: "PerSecond",
                amount: amount2
              }
            },
            "20003": {
              rewardRate: {
                period: "PerSecond",
                amount: amount3
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: durationPreset
            },
            unlockPenalty: unlockPenalty
          },
          shareAssetId: shareAssetId,
          financialNftAssetId: financialNftAssetId,
          minimumStakingAmount: minimumStakingAmount
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultPoolConfig]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      stakingPoolId2 = resultPoolId;

      // Verifications
      // Querying pool info
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId2)
      );
      expect(poolInfo.unwrap().owner.toString())
        .to.be.equal(resultOwner.toString())
        .to.be.equal(api.createType("AccountId32", walletPoolOwner.publicKey).toString());
      expect(resultPoolConfig.toString()).to.be.equal(poolConfig.toString());

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId2,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    })
    ;

    it("1.3  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with single duration preset.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const assetId = api.createType("u128", POOL_13_BASE_ASSET_ID);
      const durationPreset = {
        "600": "1200000000",
        "1200": "1500000000"
      };
      const unlockPenalty = 100_000_000;
      const shareAssetId = POOL_13_SHARE_ASSET_ID;
      const financialNftAssetId = 30000002;
      const minimumStakingAmount = Pica(10);
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "30001": {
              rewardRate: {
                period: "PerSecond",
                amount: POOL_13_REWARD_RATE
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: durationPreset
            },
            unlockPenalty: unlockPenalty
          },
          shareAssetId: shareAssetId,
          financialNftAssetId: financialNftAssetId,
          minimumStakingAmount: minimumStakingAmount
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultPoolConfig]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      stakingPoolId3 = resultPoolId;

      // Verifications
      // Querying pool info
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId3)
      );
      expect(poolInfo.unwrap().owner.toString())
        .to.be.equal(resultOwner.toString())
        .to.be.equal(api.createType("AccountId32", walletPoolOwner.publicKey).toString());
      expect(resultPoolConfig.toString()).to.be.equal(poolConfig.toString());

      // // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId3,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.5  I can create a Pablo pool using sudo & an LP token pool will get automatically created.");

    it("1.6  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero time locks.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const assetId = api.createType("u128", POOL_16_BASE_ASSET_ID);
      const durationPreset = {
        "0": 1000000000
      };
      const unlockPenalty = "100000000";
      const shareAssetId = POOL_16_SHARE_ASSET_ID;
      const financialNftAssetId = 60000002;
      const minimumStakingAmount = Pica(1);
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "60001": {
              rewardRate: {
                period: "PerSecond",
                amount: POOL_16_REWARD_RATE
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: durationPreset
            },
            unlockPenalty: unlockPenalty
          },
          shareAssetId: shareAssetId,
          financialNftAssetId: financialNftAssetId,
          minimumStakingAmount: minimumStakingAmount
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultPoolConfig]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      // After waiting for our event, we make sure the defined staking asset id
      // is the same as the pool id.
      expect(resultPoolId).to.be.bignumber.equal(assetId);
      expect(resultPoolConfig.toString()).to.be.equal(poolConfig.toString());
      stakingPoolId6 = resultPoolId;

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId6,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.7  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero penalty locks.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const assetId = api.createType("u128", POOL_17_BASE_ASSET_ID);
      const durationPreset = {
        "1200": 1000000000
      };
      const unlockPenalty = "0";
      const shareAssetId = POOL_17_SHARE_ASSET_ID;
      const financialNftAssetId = 70000002;
      const minimumStakingAmount = Pica(1);
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "70001": {
              rewardRate: {
                period: "PerSecond",
                amount: POOL_17_REWARD_RATE
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: durationPreset
            },
            unlockPenalty: unlockPenalty
          },
          shareAssetId: shareAssetId,
          financialNftAssetId: financialNftAssetId,
          minimumStakingAmount: minimumStakingAmount
        }
      });

      // Transaction
      const {
        data: [resultPoolId, resultOwner, resultPoolConfig]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      // After waiting for our event, we make sure the defined staking asset id
      // is the same as the pool id.
      expect(resultPoolId).to.be.bignumber.equal(assetId);
      expect(resultPoolConfig.toString()).to.be.equal(poolConfig.toString());
      stakingPoolId7 = resultPoolId;

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId7,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.8  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID, which will have stakers before funds will be added.",
      async function() {
        // Parameters
        const currentBlockNumber = await api.query.system.number();
        const startBlock = api.createType("u32", currentBlockNumber.addn(4));
        const assetId = api.createType("u128", POOL_18_BASE_ASSET_ID);
        const durationPreset = {
          "0": 1000000000
        };
        const unlockPenalty = "0";
        const shareAssetId = POOL_18_SHARE_ASSET_ID;
        const financialNftAssetId = 80000002;
        const minimumStakingAmount = Pica(1);
        pool18startBlock = startBlock.toNumber();
        // Creating pool config parameter
        const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
          RewardRateBasedIncentive: {
            owner: walletPoolOwner.publicKey,
            assetId: assetId,
            startBlock: startBlock,
            rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
              // The dict keys are the reward asset IDs!
              "80001": {
                rewardRate: {
                  period: "PerSecond",
                  amount: POOL_18_REWARD_RATE
                }
              }
            }),
            lock: {
              durationMultipliers: {
                Presets: durationPreset
              },
              unlockPenalty: unlockPenalty
            },
            shareAssetId: shareAssetId,
            financialNftAssetId: financialNftAssetId,
            minimumStakingAmount: minimumStakingAmount
          }
        });

        // Transaction
        const {
          data: [resultPoolId, resultOwner, resultPoolConfig]
        } = await sendAndWaitForSuccess(
          api,
          sudoKey,
          api.events.stakingRewards.RewardPoolCreated.is,
          api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
        );
        // After waiting for our event, we make sure the defined staking asset id
        // is the same as the pool id.
        expect(resultPoolId).to.be.bignumber.equal(assetId);
        expect(resultPoolConfig.toString()).to.be.equal(poolConfig.toString());
        stakingPoolId8 = resultPoolId;

        // Verifications
        await verifyPoolCreationUsingQuery(
          api,
          stakingPoolId8,
          resultOwner,
          walletPoolOwner.publicKey,
          startBlock,
          api.createType("u128", shareAssetId),
          api.createType("u128", financialNftAssetId),
          api.createType("u128", minimumStakingAmount)
        );
      });

    it("F1.1  I can not create a staking rewards pool w/o sudo", async function() {
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: 1337,
          startBlock: startBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "1337": {
              rewardRate: {
                period: "PerSecond",
                amount: POOL_11_REWARD_RATE
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: {
                "0": 1000000000
              }
            },
            unlockPenalty: "0"
          },
          shareAssetId: 80001337,
          financialNftAssetId: 800001338,
          minimumStakingAmount: Pica(1)
        }
      });
      const err = await sendAndWaitForSuccess(
        api,
        walletPoolOwner,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.stakingRewards.createRewardPool(poolConfig)
      ).catch(err => err);
      expect(err.toString()).to.contain("BadOrigin");
    });
  });

  describe("2. Adding rewards to pool pots. pt.1", function() {
    it("2.1  [SHORT] I can, as pool owner, add rewards to staking rewards pool pot #1.1, " +
      "before the pool has started.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      expect(currentBlockNumber).to.be.bignumber.lessThan(new BN(pool11startBlock.toString())); // Test if poolRewardState isNone if started already!
      const poolId = stakingPoolId1;
      const assetId = POOL_11_REWARD_ASSET_ID;
      const amount = Pica(100_000_000_000);
      const keepAlive = true;
      const walletBalanceBefore = await api.rpc.assets.balanceOf(assetId.toString(), walletPoolOwner.publicKey);

      const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId);
      const poolRewardAmountBefore = poolInfoBefore.unwrap().rewards.toJSON()[assetId.toString()];
      const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
      console.debug(`isEmpty ${poolRewardStateBefore.toString()}`);

      // expect(poolRewardStateBefore.isNone).to.be.false; // ToDo: ?
      // Transaction
      const {
        data: [resultPoolId, resultAssetId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletPoolOwner,
        api.events.stakingRewards.RewardsPotIncreased.is,
        api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
      );

      // Verification
      // Verifying the poolId, reported by the event, is correct.
      expect(poolId).to.be.bignumber.equal(resultPoolId);
      // Verifying the reward asset id, reported by the event, is equal to the reward asset we added to the pool.
      expect(new BN(assetId)).to.be.bignumber.equal(resultAssetId);
      // Verifying the added amount, reported by the event, is equal to the amount we added.
      expect(new BN(amount.toString())).to.be.bignumber.equal(resultAmount);
      await verifyPoolPotAddition(
        api,
        poolId,
        assetId,
        amount,
        walletPoolOwner,
        walletBalanceBefore,
        // @ts-ignore
        new BN(poolRewardAmountBefore["totalRewards"].toString())
      );
    });

    it("2.4  [SHORT] Any user can add funds to a pool whose reward pot is not empty.", async function() {
      // Parameters
      const poolId = stakingPoolId1;
      const assetId = POOL_11_REWARD_ASSET_ID;
      const amount = Pica(100_000_000_000);
      const keepAlive = true;
      const walletBalanceBefore = await api.rpc.assets.balanceOf(assetId.toString(), walletRewardAdder.publicKey);
      const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId);
      const poolRewardAmountBefore = poolInfoBefore.unwrap().rewards.toJSON()[assetId.toString()];

      const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
      expect(poolRewardStateBefore.isNone).to.be.true;

      // Transaction
      const {
        data: [resultPoolId, resultAssetId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletRewardAdder,
        api.events.stakingRewards.RewardsPotIncreased.is,
        api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
      );

      // Verification
      // Verifying the poolId, reported by the event, is correct.
      expect(poolId).to.be.bignumber.equal(resultPoolId);
      // Verifying the reward asset id, reported by the event, is equal to the reward asset we added to the pool.
      expect(new BN(assetId)).to.be.bignumber.equal(resultAssetId);
      // Verifying the added amount, reported by the event, is equal to the amount we added.
      expect(new BN(amount.toString())).to.be.bignumber.equal(resultAmount);

      await verifyPoolPotAddition(
        api,
        poolId,
        assetId,
        amount,
        walletRewardAdder,
        walletBalanceBefore,
        // @ts-ignore
        new BN(poolRewardAmountBefore["totalRewards"].toString())
      );
    });

    it("2.2  Any user can add all reward assets to another staking rewards pool with multiple reward pots #1.2.", async function() {
      // Parameters
      const poolId = stakingPoolId2;
      const assetIds = [POOL_12_REWARD_ASSET_ID_1, POOL_12_REWARD_ASSET_ID_2, POOL_12_REWARD_ASSET_ID_3];
      const amount = Pica(100_000_000_000);
      const keepAlive = true;

      const poolInfosBefore: Option<ComposableTraitsStakingRewardPool>[] = await Promise.all([
        api.query.stakingRewards.rewardPools(poolId),
        api.query.stakingRewards.rewardPools(poolId),
        api.query.stakingRewards.rewardPools(poolId)
      ]);
      const poolRewardAmountsBefore = [
        poolInfosBefore[0].unwrap().rewards.toJSON()[assetIds[0].toString()],
        poolInfosBefore[1].unwrap().rewards.toJSON()[assetIds[1].toString()],
        poolInfosBefore[2].unwrap().rewards.toJSON()[assetIds[2].toString()]
      ];
      // Transaction
      const transactions: SubmittableExtrinsic<"promise">[] = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const walletBalanceFunctions: Promise<CustomRpcBalance>[] = [];
      assetIds.forEach(function(asset) {
        transactions.push(api.tx.stakingRewards.addToRewardsPot(poolId, asset, amount, keepAlive));
        walletBalanceFunctions.push(api.rpc.assets.balanceOf(asset.toString(), walletRewardAdder.publicKey));
      });
      const walletBalancesBefore = await Promise.all(walletBalanceFunctions);
      await sendWithBatchAndWaitForSuccess(
        api,
        walletRewardAdder,
        api.events.stakingRewards.RewardsPotIncreased.is,
        transactions,
        false
      );

      // Verification
      // @ts-ignore
      await verifyPoolPotAddition(api, poolId, assetIds[0], amount, walletRewardAdder, walletBalancesBefore[0], new BN(poolRewardAmountsBefore[0]["totalRewards"].toString()));
      // @ts-ignore
      await verifyPoolPotAddition(api, poolId, assetIds[1], amount, walletRewardAdder, walletBalancesBefore[1], new BN(poolRewardAmountsBefore[1]["totalRewards"].toString()));
      // @ts-ignore
      await verifyPoolPotAddition(api, poolId, assetIds[2], amount, walletRewardAdder, walletBalancesBefore[2], new BN(poolRewardAmountsBefore[2]["totalRewards"].toString()));
    });

    it("2.3  Any user can add rewards to multiple staking pools at once.", async function() {
      // Parameters
      const poolIds = [stakingPoolId3, stakingPoolId6, stakingPoolId7];
      const assetIds = [POOL_13_REWARD_ASSET_ID, POOL_16_REWARD_ASSET_ID, POOL_17_REWARD_ASSET_ID];
      const amount = Pica(100_000_000_000);
      const keepAlive = false;

      const poolInfosBefore: Option<ComposableTraitsStakingRewardPool>[] = await Promise.all([
        api.query.stakingRewards.rewardPools(poolIds[0]),
        api.query.stakingRewards.rewardPools(poolIds[1]),
        api.query.stakingRewards.rewardPools(poolIds[2])
      ]);
      const poolRewardAmountsBefore = [
        poolInfosBefore[0].unwrap().rewards.toJSON()[assetIds[0].toString()],
        poolInfosBefore[1].unwrap().rewards.toJSON()[assetIds[1].toString()],
        poolInfosBefore[2].unwrap().rewards.toJSON()[assetIds[2].toString()]
      ];
      // Transaction
      const transactions: SubmittableExtrinsic<"promise">[] = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const walletBalanceFunctions: Promise<CustomRpcBalance>[] = [];
      assetIds.forEach(function(asset, i) {
        transactions.push(api.tx.stakingRewards.addToRewardsPot(poolIds[i], asset, amount, keepAlive));
        walletBalanceFunctions.push(api.rpc.assets.balanceOf(asset.toString(), walletRewardAdder.publicKey));
      });
      const walletBalancesBefore = await Promise.all(walletBalanceFunctions);
      await sendWithBatchAndWaitForSuccess(
        api,
        walletRewardAdder,
        api.events.stakingRewards.RewardsPotIncreased.is,
        [
          api.tx.stakingRewards.addToRewardsPot(poolIds[0], assetIds[0], amount, keepAlive),
          api.tx.stakingRewards.addToRewardsPot(poolIds[1], assetIds[1], amount, keepAlive),
          api.tx.stakingRewards.addToRewardsPot(poolIds[2], assetIds[2], amount, keepAlive)
        ],
        false
      );

      // Verification
      await Promise.all([
        // @ts-ignore
        verifyPoolPotAddition(api, poolIds[0], assetIds[0], amount, walletRewardAdder, walletBalancesBefore[0], new BN(poolRewardAmountsBefore[0]["totalRewards"].toString())),
        // @ts-ignore
        verifyPoolPotAddition(api, poolIds[1], assetIds[1], amount, walletRewardAdder, walletBalancesBefore[1], new BN(poolRewardAmountsBefore[1]["totalRewards"].toString())),
        // @ts-ignore
        verifyPoolPotAddition(api, poolIds[2], assetIds[2], amount, walletRewardAdder, walletBalancesBefore[2], new BN(poolRewardAmountsBefore[2]["totalRewards"].toString()))
      ]);
    });

    it("F2.1  I can not add rewards to a staking rewards pool pot with insufficient funds.",
      async function() {
        // Parameters
        const poolId = stakingPoolId1;
        const assetId = POOL_11_REWARD_ASSET_ID;
        const amount = Pica(100_000_000_000);
        const keepAlive = true;

        const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
        expect(poolRewardStateBefore.isNone).to.be.true;

        // Transaction
        const error = await sendAndWaitForSuccess(
          api,
          walletStaker2,
          api.events.stakingRewards.RewardsPotIncreased.is,
          api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
        ).catch(err => err);
        expect(error.toString()).to.contain("tokens.BalanceTooLow");
      });

    it("F2.2  I can not add reward assets to a pool that aren't part of the reward asset IDs.",
      async function() {
        // Parameters
        const poolId = stakingPoolId1;
        const assetId = POOL_13_REWARD_ASSET_ID;
        const amount = Pica(100_000_000_000);
        const keepAlive = true;
        const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId);

        const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
        expect(poolRewardStateBefore.isNone).to.be.true;

        // Transaction
        const error = await sendAndWaitForSuccess(
          api,
          walletRewardAdder,
          api.events.stakingRewards.RewardsPotIncreased.is,
          api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
        ).catch(err => err);
        expect(error.toString()).to.contain("RewardAssetNotFound");
      });
  });

  describe("3. Staking in the pools. pt. 1", function() {
    it("3.1  [SHORT] I can stake in the newly created rewards pool #1.1", async function() {
      const currentBlockNumber = (await api.query.system.number());
      if (currentBlockNumber.lt(new BN(pool11startBlock.toString())))
        await waitForBlocks(api, Number(pool11startBlock - currentBlockNumber.toNumber()));

      // Parameters
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      const durationPreset = 0;
      const stakeAmount = Pica(1_000);

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId1, stakeAmount, durationPreset)
      );

      blockNumAtStake31 = Number((await api.query.system.number()).toString());

      const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId1)
      );
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);
      inflationStake31 = (BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfo.unwrap().share.toString())
        ).toString()
      ));


      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(stakingPoolId1);
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      // Noting down the fNFT details for later use.
      fNFTCollectionId1 = resultFNFTCollectionId;
      fNFTInstanceId1 = resultFNFTInstanceId;

      await verifyPoolStaking(
        api,
        fNFTCollectionId1,
        fNFTInstanceId1,
        stakeAmount,
        stakeAmount,
        stakingPoolId1,
        walletStaker,
        userFundsBefore
      );
    });

    it("3.12  I can stake in a newly created pool before funds have been added. (#1.8)", async function() {
      const currentBlockNumber = (await api.query.system.number());
      if (currentBlockNumber.lt(new BN(pool18startBlock.toString())))
        await waitForBlocks(api, Number(pool18startBlock - currentBlockNumber.toNumber()));

      // Parameters
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_18_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      const durationPreset = 0;
      const stakeAmount = Pica(1_000);

      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId8, stakeAmount, durationPreset)
      );


      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(stakingPoolId8);
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      // Noting down the fNFT details for later use.
      fNFTCollectionId12 = resultFNFTCollectionId;
      fNFTInstanceId12 = resultFNFTInstanceId;

      await verifyPoolStaking(
        api,
        fNFTCollectionId12,
        fNFTInstanceId12,
        stakeAmount,
        stakeAmount,
        stakingPoolId8,
        walletStaker,
        userFundsBefore
      );
    });
  });

  describe("2. Adding rewards to pool pots. pt. 2", function() {
    it("2.5  I can add rewards to a staking rewards pool which was empty but has stakers in it (#3.12).",
      async function() {
        // Parameters
        const poolId = stakingPoolId8;
        const assetId = POOL_18_REWARD_ASSET_ID_1;
        const amount = Pica(250_000); // We'll empty this pool in the tests!
        const keepAlive = true;
        const walletBalanceBefore = await api.rpc.assets.balanceOf(assetId.toString(), walletRewardAdder.publicKey);

        const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId);
        const poolRewardAmountBefore = poolInfoBefore.unwrap().rewards.toJSON()[assetId.toString()];
        const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
        expect(poolRewardStateBefore.isNone).to.be.false;
        // Transaction
        const {
          data: [resultPoolId, resultAssetId, resultAmount]
        } = await sendAndWaitForSuccess(
          api,
          walletRewardAdder,
          api.events.stakingRewards.RewardsPotIncreased.is,
          api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
        );

        blockNumAtFundsAddition3_12 = Number((await api.query.system.number()).toString()) + 1; // Pool starts one block later!
        // Verification
        // Verifying the poolId, reported by the event, is correct.
        expect(poolId).to.be.bignumber.equal(resultPoolId);
        // Verifying the reward asset id, reported by the event, is equal to the reward asset we added to the pool.
        expect(new BN(assetId)).to.be.bignumber.equal(resultAssetId);
        // Verifying the added amount, reported by the event, is equal to the amount we added.
        expect(new BN(amount.toString())).to.be.bignumber.equal(resultAmount);

        const stakeInfo = await api.query.stakingRewards.stakes(fNFTCollectionId12, fNFTInstanceId12);
        const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
          await api.query.stakingRewards.rewardPools(stakingPoolId8)
        );
        const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_18_SHARE_ASSET_ID);
        inflationStake312 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_18_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        await verifyPoolPotAddition(
          api,
          poolId,
          assetId,
          amount,
          walletRewardAdder,
          walletBalanceBefore,
          // @ts-ignore
          new BN(poolRewardAmountBefore["totalRewards"].toString())
        );
      });
  });

  describe("4. Claiming from staked positions pt. 1", function() {
    it("4.1  [SHORT] I can claim from the arbitrary asset pool in #1.1 using the stake from #3.1, during the lock period.", async function() {
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_11_REWARD_ASSET_ID.toString(),
        walletStaker.publicKey
      );
      const userFundsBeforeA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId1;
      const fNFTInstanceId = fNFTInstanceId1;
      const stakingPoolId = stakingPoolId1;


      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate the claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);

      const stakerSharePart = await getNthStakerSharesPart(
        api,
        POOL_11_SHARE_ASSET_ID,
        BigNumber(stakeInfoBefore.unwrap().share.toString()));
      console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);

      // Calculating claimable amount.
      // Adding the inflation calculated at the end of #3.1, which is required
      // for the first claim in the pool for the amounts to align.
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake31,
        BigNumber(POOL_11_REWARD_RATE.toString())
      )).plus(BigNumber(inflationStake31.toString()));
      console.debug(`verifAmt4 ${verifAmt4th.toString()}`);

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );

      const userFundsAfterA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );
      console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      inflationStake31 = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      console.debug(`infAft ${inflationStake31.toString()}`);

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [POOL_11_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        [api.createType("u128", verifAmt4th.toString())]
      );
    });
  });

  describe("3. Staking in the pools pt. 2", function() {
    it("3.2  Another user can stake in the newly created rewards pool #1.1", async function() {
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker2.publicKey);
      // Parameters
      const durationPreset = 0;
      const stakeAmount = Pica(1_000);
      const totalShareAssetIssuanceBefore = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);
      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId1, stakeAmount, durationPreset)
      );
      const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);

      // Getting pool info before transaction, to calculate the claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId1)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);
      // inflationAtStake32 = getStakeReduction(api, stakeInfo.unwrap(), poolInfo.unwrap(), POOL_11_REWARD_ASSET_ID.toString(), totalShareAssetIssuance);

      blockNumAtStake32 = Number((await api.query.system.number()).toString());

      const totalRewards = await getTotalRewardForCurrentTimestamp(api, poolStartTime, BigNumber(Pica(1).toString()));
      const newReduction = totalRewards.multipliedBy(BigNumber(stakeInfo.unwrap().stake.toString())).dividedBy(BigNumber(totalShareAssetIssuance.toString()));
      console.debug(`redu alt ${newReduction.toString()}`);

      inflationStake32 = (BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfo.unwrap().share.toString())
        ).toString()
      ));
      console.debug(`stake31Reduction ${inflationStake32.toString()}`);
      console.debug(`stake31Reduction alt ${totalRewards.multipliedBy(BigNumber(stakeInfo.unwrap().stake.toString())).dividedBy(BigNumber(totalShareAssetIssuanceBefore.toString()))}`);

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(stakingPoolId1);
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker2.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      fNFTCollectionId2 = resultFNFTCollectionId;
      fNFTInstanceId2 = resultFNFTInstanceId;

      await verifyPoolStaking(
        api,
        fNFTCollectionId2,
        fNFTInstanceId2,
        stakeAmount.toString(),
        stakeAmount.toString(),
        stakingPoolId1,
        walletStaker2,
        userFundsBefore
      );
    });

    it("3.3  A user can stake multiple times in the same staking rewards pool with multiple multiplier options.",
      async function() {
        // Getting funds before transaction
        const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
        // Parameters
        const durationPreset1 = 600;
        const durationPreset2 = 1200;
        const stakeAmount = Pica(100);
        // Transaction
        const {
          data: [
            resultPoolId,
            resultOwnerAccountId,
            resultAmount,
            resultDurationPreset,
            resultFNFTCollectionId,
            resultFNFTInstanceId,
            resultRewardMultiplier,
            resultKeepAlive
          ]
        } = await sendWithBatchAndWaitForSuccess(
          api,
          walletStaker,
          api.events.stakingRewards.Staked.is,
          [
            api.tx.stakingRewards.stake(stakingPoolId2, stakeAmount, durationPreset1),
            api.tx.stakingRewards.stake(stakingPoolId2, stakeAmount, durationPreset2)
          ],
          false
        );

        // Verification
        // Verifying the poolId, reported by the event, is reported correctly.
        expect(resultPoolId).to.be.bignumber.equal(stakingPoolId2);
        // Verifying the stake owner, reported by the event, is reported correctly.
        expect(resultOwnerAccountId.toString()).to.be.equal(
          api.createType("AccountId32", walletStaker.publicKey).toString()
        );
        // Verifying the amount, reported by the event, is correct.
        expect(resultAmount.toString()).to.equal(stakeAmount.toString());
        // Verifying the durationPreset equals our requested durationPreset.
        expect(resultDurationPreset.toString()).to.equal(durationPreset1.toString());
        // Verifying the keepAlive parameter, reported by the event, is correct.
        expect(resultKeepAlive);
        fNFTCollectionId3 = resultFNFTCollectionId;
        fNFTInstanceId3 = resultFNFTInstanceId;
        blockNumAtStake33 = Number((await api.query.system.number()).toString());

        const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);
        const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
          await api.query.stakingRewards.rewardPools(stakingPoolId2)
        );
        const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_12_SHARE_ASSET_ID);
        inflationStake33_1 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        inflationStake33_2 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_2.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        inflationStake33_3 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_3.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        await verifyPoolStaking(
          api,
          fNFTCollectionId3,
          fNFTInstanceId3,
          stakeAmount.toString(),
          (stakeAmount * BigInt(2)).toString(),
          stakingPoolId2,
          walletStaker,
          userFundsBefore
        );
      });

    it("3.4  I can stake in the preconfigured PICA pool", async function() {
      this.skip();
      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Getting funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      // Parameters
      const durationPreset = 604800;
      const picaPoolId = 1;
      const stakeAmount = (10 ** 12).toString();
      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(picaPoolId, stakeAmount, durationPreset)
      );

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(new BN(picaPoolId));
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      fNFTCollectionId4Pica = resultFNFTCollectionId;
      fNFTInstanceId4Pica = resultFNFTInstanceId;

      // ToDo: Verify function does not work for PICA pool!
    });

    it("3.5  I can stake in the preconfigured PBLO pool", async function() {
      this.skip();

      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker.publicKey);
      // Parameters
      const durationPreset = 604800;
      const pbloPoolId = 5;
      const stakeAmount = (10 ** 12).toString();
      // Transaction
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(pbloPoolId, stakeAmount, durationPreset)
      );

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(new BN(pbloPoolId));
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      fNFTCollectionId5Pblo = resultFNFTCollectionId;
      fNFTInstanceId5Pblo = resultFNFTInstanceId;

      await verifyPoolStaking(
        api,
        fNFTCollectionId5Pblo,
        fNFTInstanceId5Pblo,
        stakeAmount,
        stakeAmount,
        api.createType("u128", pbloPoolId),
        walletStaker,
        userFundsBefore
      );
    });

    it("3.6  I can stake in the created LP token pool #1.5.");

    it("3.7  I can stake in the newly created pool with 0 time locks. #1.6", async function() {
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_16_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 0;
      const stakeAmount = Pica(1_000);

      // Transaction
      const {
        data: [
          resultPoolId,
          resultStakeOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId6, stakeAmount, durationPreset)
      );
      blockNumAtStake37 = (await api.query.system.number()).toNumber();

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(stakingPoolId6);
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultStakeOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      fNFTCollectionId7 = resultFNFTCollectionId;
      fNFTInstanceId7 = resultFNFTInstanceId;
      const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId6)
      );
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_16_SHARE_ASSET_ID);
      inflationStake37 = (BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_16_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfo.unwrap().share.toString())
        ).toString()
      ));
      await verifyPoolStaking(
        api,
        fNFTCollectionId7,
        fNFTInstanceId7,
        stakeAmount,
        stakeAmount,
        stakingPoolId6,
        walletStaker,
        userFundsBefore
      );
    });

    it("3.8  I can stake in the newly created pool with 0 unlock penalty. #1.7", async function() {
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_17_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 1200;
      const stakeAmount = Pica(1_000);

      // Transaction
      const {
        data: [
          resultPoolId,
          resultStakeOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId7, stakeAmount, durationPreset)
      );
      blockNumAtStake38 = Number((await api.query.system.number()).toString());

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(stakingPoolId7);
      // Verifying the pool owner, reported by the event, is reported correctly.
      expect(resultStakeOwnerAccountId.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker.publicKey).toString()
      );
      // Verifying the amount, reported by the event, is correct.
      expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // Verifying the durationPreset equals our requested durationPreset.
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      fNFTCollectionId8 = resultFNFTCollectionId;
      fNFTInstanceId8 = resultFNFTInstanceId;
      const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId7)
      );
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_17_SHARE_ASSET_ID);
      inflationStake38 = (BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_17_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfo.unwrap().share.toString())
        ).toString()
      ));
      await verifyPoolStaking(
        api,
        fNFTCollectionId8,
        fNFTInstanceId8,
        stakeAmount,
        stakeAmount,
        stakingPoolId7,
        walletStaker,
        userFundsBefore
      );
    });
  });

  describe("4. Claiming from staked positions. pt. 2", function() {
    it("4.2  I can claim from the arbitrary asset pool in #1.1 using the stake from #3.2, during the lock period.", async function() {
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_11_REWARD_ASSET_ID.toString(),
        walletStaker2.publicKey
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId2;
      const fNFTInstanceId = fNFTInstanceId2;
      const stakingPoolId = stakingPoolId1;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate the claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);

      const userFundsBeforeA = await api.query.tokens.accounts(
        walletStaker2.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );

      const stakerSharePart = await getNthStakerSharesPart(
        api,
        POOL_11_SHARE_ASSET_ID,
        BigNumber(stakeInfoBefore.unwrap().share.toString()));
      console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);

      // Calculating claimable amount.
      const verifAmt = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake32,
        BigNumber(POOL_11_REWARD_RATE.toString())
      ));
      console.debug(`verifAmt4 ${verifAmt.toString()}`);

      // Transaction
      const {
        data: [resultStakeOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId2, fNFTInstanceId2)
      );

      const userFundsAfterA = await api.query.tokens.accounts(
        walletStaker2.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );
      console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);
      // Verification
      expect(resultStakeOwner.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker2.publicKey).toString()
      );
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      inflationStake32 = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      console.debug(`inf after ${BigNumber(inflationStake32.toString())}`);


      await verifyPoolClaiming(
        api,
        fNFTCollectionId2,
        fNFTInstanceId2,
        [POOL_11_REWARD_ASSET_ID],
        walletStaker2,
        [userFundsBefore],
        [api.createType("u128", verifAmt.toString())]
      );
    });

    it("4.3  I can claim from the arbitrary asset pool in #1.2 using the stakes from #3.3.", async function() {
      // Get funds before transaction
      const userFundsBefore = await Promise.all([
        api.rpc.assets.balanceOf(
          POOL_12_REWARD_ASSET_ID_1.toString(),
          walletStaker.publicKey
        ),
        api.rpc.assets.balanceOf(
          POOL_12_REWARD_ASSET_ID_2.toString(),
          walletStaker.publicKey
        ),
        api.rpc.assets.balanceOf(
          POOL_12_REWARD_ASSET_ID_3.toString(),
          walletStaker.publicKey
        )
      ]);

      const userFundsBeforeA1 = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_12_REWARD_ASSET_ID_1.toString()
      );
      const userFundsBeforeA2 = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_12_REWARD_ASSET_ID_2.toString()
      );
      const userFundsBeforeA3 = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_12_REWARD_ASSET_ID_3.toString()
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId3;
      const fNFTInstanceId = fNFTInstanceId3;
      const stakingPoolId = stakingPoolId2;

      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate the claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_12_SHARE_ASSET_ID);
      console.debug(`inf1 bef ${inflationStake33_1.toString()}`);
      console.debug(`inf2 bef ${inflationStake33_2.toString()}`);
      console.debug(`inf3 bef ${inflationStake33_3.toString()}`);

      const stakerSharePart = await getNthStakerSharesPart(
        api,
        POOL_12_SHARE_ASSET_ID,
        BigNumber(stakeInfoBefore.unwrap().share.toString()));
      console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);
      // Calculating claimable amount.
      const verifAmt_1 = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake33,
        BigNumber(POOL_12_REWARD_RATE_1.toString())
      )).plus(inflationStake33_1);
      const verifAmt_2 = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake33,
        BigNumber(POOL_12_REWARD_RATE_2.toString())
      )).plus(inflationStake33_2);
      const verifAmt_3 = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake33,
        BigNumber(POOL_12_REWARD_RATE_3.toString())
      )).plus(inflationStake33_3);
      console.debug(`verifAmt ${verifAmt_1}`);
      console.debug(`verifAmt ${verifAmt_2}`);
      console.debug(`verifAmt ${verifAmt_3}`);

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());


      inflationStake33_1 = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      inflationStake33_2 = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_2.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      inflationStake33_3 = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_3.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      console.debug(`inf1 aft ${inflationStake33_1.toString()}`);
      console.debug(`inf2 aft ${inflationStake33_2.toString()}`);
      console.debug(`inf3 aft ${inflationStake33_3.toString()}`);

      const userFundsAfterA1 = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_12_REWARD_ASSET_ID_1.toString()
      );
      const userFundsAfterA2 = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_12_REWARD_ASSET_ID_2.toString()
      );
      const userFundsAfterA3 = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_12_REWARD_ASSET_ID_3.toString()
      );
      console.debug(`cA1 ${BigNumber(userFundsAfterA1.free.toString()).minus(BigNumber(userFundsBeforeA1.free.toString()))}`);
      console.debug(`cA2 ${BigNumber(userFundsAfterA2.free.toString()).minus(BigNumber(userFundsBeforeA2.free.toString()))}`);
      console.debug(`cA3 ${BigNumber(userFundsAfterA3.free.toString()).minus(BigNumber(userFundsBeforeA3.free.toString()))}`);

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [POOL_12_REWARD_ASSET_ID_1, POOL_12_REWARD_ASSET_ID_2, POOL_12_REWARD_ASSET_ID_3],
        walletStaker,
        userFundsBefore,
        [
          api.createType("u128", verifAmt_1.toFixed(0, 1)),
          api.createType("u128", verifAmt_2.toFixed(0, 1)),
          api.createType("u128", verifAmt_3.toFixed(0, 1))
        ]
      );
    });

    it("4.4  I can claim from the PICA pool using my stake in #3.4, after the lock period has ended.", async function() {
      this.skip();

      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_11_REWARD_ASSET_ID.toString(),
        walletStaker.publicKey
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId4Pica;
      const fNFTInstanceId = fNFTInstanceId4Pica;
      const stakingPoolId = 1;
      const userFundsBeforeA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_12_SHARE_ASSET_ID);
      // Calculating claimable amount.
      const claimableAmount = getClaimOfStake(
        api,
        stakeInfoBefore.unwrap(),
        poolInfo.unwrap(),
        POOL_12_REWARD_ASSET_ID_1.toString(),
        totalShareAssetIssuance
      );
      console.debug(`eCA ${claimableAmount.toString()}`);

      const userFundsAfterA = await api.query.tokens.accounts(
        walletStaker2.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );
      console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId4Pica.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId4Pica.toString());

      // ToDo: Verify function does not work for PICA asset!
      /*await verifyPoolClaiming(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        POOL_12_REWARD_ASSET_ID,
        walletStaker2,
        userFundsBefore
      );*/
    });

    it("4.5  I can claim from the PBLO pool using my stake in #3.5, after the lock period has ended.", async function() {
      this.skip();
      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(PBLO_ASSET_ID.toString(), walletStaker.publicKey);

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId5Pblo;
      const fNFTInstanceId = fNFTInstanceId5Pblo;
      const stakingPoolId = 5;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(1005);
      // Calculating claimable amount.
      const claimableAmount = getClaimOfStake(
        api,
        stakeInfoBefore.unwrap(),
        poolInfo.unwrap(),
        PBLO_ASSET_ID.toString(),
        totalShareAssetIssuance
      );

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [PBLO_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        [api.createType("u128", claimableAmount)]
      );
    });

    it("4.6  I can claim from the LP token pool using my stake in #3.6, after the lock period has ended.", async function() {

      // LP staking does not seem to work!
      // Please see test 3.6 for further information!
      // ToDo: Trade to have claimable funds!
      this.skip();

      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(pool15LpTokenId.toString(), walletStaker.publicKey);

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId6;
      const fNFTInstanceId = fNFTInstanceId6;
      const stakingPoolId = stakingPoolId5;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_15_SHARE_ASSET_ID);
      // Calculating claimable amount.
      const claimableAmount = getClaimOfStake(
        api,
        stakeInfoBefore.unwrap(),
        poolInfo.unwrap(),
        POOL_11_REWARD_ASSET_ID.toString(),
        totalShareAssetIssuance
      );

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId6, fNFTInstanceId6)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId6.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId6.toString());

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [pool15LpTokenId], // ToDo: Check!
        walletStaker,
        [userFundsBefore],
        [api.createType("u128", claimableAmount)]
      );
    });

    it("4.7  I can claim from the 0 time lock pool using my stake in #3.7", async function() {
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_16_REWARD_ASSET_ID.toString(),
        walletStaker.publicKey
      );
      const userFundsBeforeA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_16_REWARD_ASSET_ID.toString()
      );
      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId7;
      const fNFTInstanceId = fNFTInstanceId7;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );

      const stakerSharePart = await getNthStakerSharesPart(
        api,
        POOL_16_SHARE_ASSET_ID,
        BigNumber(stakeInfoBefore.unwrap().share.toString()));
      console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);
      console.debug(`inf ${BigNumber(inflationStake37.toString())}`);

      // Calculating claimable amount.
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake37,
        BigNumber(POOL_16_REWARD_RATE.toString())
      )).plus(BigNumber(inflationStake37.toFixed()));
      console.debug(`verifAmt4 ${verifAmt4th.toString()}`);

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );

      const userFundsAfterA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_16_REWARD_ASSET_ID.toString()
      );
      console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [POOL_16_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        [api.createType("u128", verifAmt4th.toString())]
      );
    });

    it("4.8  I can claim from the 0 unlock penalty pool using my stake in #3.8.", async function() {
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_17_REWARD_ASSET_ID.toString(),
        walletStaker.publicKey
      );
      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId8;
      const fNFTInstanceId = fNFTInstanceId8;

      const userFundsBeforeA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_17_REWARD_ASSET_ID.toString()
      );

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      const stakerSharePart = await getNthStakerSharesPart(
        api,
        POOL_17_SHARE_ASSET_ID,
        BigNumber(stakeInfoBefore.unwrap().share.toString()));
      console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);
      console.debug(`inf ${BigNumber(inflationStake38.toString())}`);

      // Calculating claimable amount.
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake38,
        BigNumber(POOL_17_REWARD_RATE.toString())
      )).plus(BigNumber(inflationStake38.toFixed()));
      console.debug(`verifAmt4 ${verifAmt4th.toString()}`);

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );
      blockNumAtStake38 = Number((await api.query.system.number()).toString());
      const userFundsAfterA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_17_REWARD_ASSET_ID.toString()
      );
      console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [POOL_17_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        [api.createType("u128", verifAmt4th.toString())]
      );
    });

    it("4.15  I can claim from the staking rewards pool, " +
      "in which I staked before any funds were added. (#1.8, #2.5, #3.12)",
      async function() {
        // Get funds before transaction
        const userFundsBefore = await api.rpc.assets.balanceOf(
          POOL_18_REWARD_ASSET_ID_1.toString(),
          walletStaker.publicKey
        );

        const userFundsBeforeA = await api.query.tokens.accounts(
          walletStaker.publicKey,
          POOL_18_REWARD_ASSET_ID_1.toString()
        );
        // Setting Parameters
        const fNFTCollectionId = fNFTCollectionId12;
        const fNFTInstanceId = fNFTInstanceId12;

        // Getting stake info before transaction, to calculate claimable amount.
        const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
          await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
        );
        // Getting pool info before transaction, to calculate the claimable amount.
        const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
          await api.query.stakingRewards.rewardPools(stakingPoolId8)
        );

        // Getting total issuance of the defined share asset, used for claimable amount calculation.
        const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);
        const stakerSharePart = await getNthStakerSharesPart(
          api,
          POOL_18_SHARE_ASSET_ID,
          BigNumber(stakeInfoBefore.unwrap().share.toString()));
        console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);
        console.debug(`inf ${BigNumber(inflationStake312.toString())}`);

        // Calculating claimable amount.
        const verifAmt = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
          api,
          stakerSharePart,
          blockNumAtFundsAddition3_12,
          BigNumber(POOL_18_REWARD_RATE.toString())
        )).plus(BigNumber(inflationStake312.toString()));
        console.debug(`verifAmt4 ${verifAmt.toString()}`);

        // Transaction
        const {
          data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
        } = await sendAndWaitForSuccess(
          api,
          walletStaker,
          api.events.stakingRewards.Claimed.is,
          api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
        );

        // Verification
        expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
        expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
        expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

        inflationStake312 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_18_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
            BigNumber(stakeInfoBefore.unwrap().share.toString())
          ).toString()
        ));
        const userFundsAfterA = await api.query.tokens.accounts(
          walletStaker.publicKey,
          POOL_18_REWARD_ASSET_ID_1.toString()
        );
        console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);
        await verifyPoolClaiming(
          api,
          fNFTCollectionId,
          fNFTInstanceId,
          [POOL_18_REWARD_ASSET_ID_1],
          walletStaker,
          [userFundsBefore],
          [api.createType("u128", verifAmt.toFixed(0))]
        );
      });
  });

  describe("5. Extending existing positions.", function() {
    it("5.1  [SHORT] I can extend the staked amount in pool #1.2 using the stake from #3.3", async function() {
      // Querying stake
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId3, fNFTInstanceId3)
      );
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const amount = Pica(10);

      // Transaction
      const {
        data: [resultFNFTCollectionId, resultFNFTInstanceId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.StakeAmountExtended.is,
        api.tx.stakingRewards.extend(fNFTCollectionId3, fNFTInstanceId3, amount)
      );

      // Verification
      // Verifying the fNFTCollectionId & instance ID is correct.
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId3);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId3);
      // Verifying the extended amount, reported by the event, is correct to what we set it to.
      expect(resultAmount.toString()).to.equal(amount.toString());

      await verifyPositionExtension(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeInfoBefore,
        Number(amount.toString()),
        walletStaker,
        POOL_12_BASE_ASSET_ID,
        userFundsBefore,
        api.createType("u128", POOL_12_SHARE_ASSET_ID)
      );
    });

    it.skip("5.2  I can extend the lock time in pool #1.1 using the stake from #3.1", async function() {
      // ToDo: Enable when lock time extension works!
      // Querying stake
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId3, fNFTInstanceId3)
      );
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const amount = 0;

      // Transaction
      const {
        data: [resultFNFTCollectionId, resultFNFTInstanceId, resultAmount]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.StakeAmountExtended.is,
        api.tx.stakingRewards.extend(fNFTCollectionId3, fNFTInstanceId3, amount)
      );

      // Verification
      // Verifying the fNFTCollectionId & instance ID is correct.
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId3);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId3);
      // Verifying the extended amount, reported by the event, is correct to what we set it to.
      expect(resultAmount.toString()).to.equal(amount.toString());

      await verifyPositionExtension(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeInfoBefore,
        amount,
        walletStaker,
        POOL_12_BASE_ASSET_ID,
        userFundsBefore,
        api.createType("u128", POOL_12_SHARE_ASSET_ID)
      );
    });
  });

  describe("6. Splitting existing positions.", function() {
    it("6.1  [SHORT] I can split my staking position into 2 separate positions", async function() {
      const stakeInfoBefore = await api.query.stakingRewards.stakes(fNFTCollectionId3, fNFTInstanceId3);

      // Transaction
      const {
        data: [resultPositions]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.SplitPosition.is,
        api.tx.stakingRewards.split(fNFTCollectionId3, fNFTInstanceId3, 500_000)
      );

      // Verification
      expect(resultPositions.length).to.be.equal(2);

      await verifyPositionSplitting(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeInfoBefore,
        0.5,
        0.5,
        resultPositions[1][0],
        resultPositions[1][1]
      );
    });

    it("6.2  I can split my already split position again", async function() {
      // ToDo: Buggy!
      // Ticket: https://app.clickup.com/t/3200cuw

      const stakeInfoBefore = await api.query.stakingRewards.stakes(fNFTCollectionId3, fNFTInstanceId3);

      // Transaction
      const {
        data: [resultPositions]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.SplitPosition.is,
        api.tx.stakingRewards.split(fNFTCollectionId3, fNFTInstanceId3, 300_000)
      );

      // Verification
      expect(resultPositions.length).to.be.equal(2);
      await verifyPositionSplitting(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeInfoBefore,
        0.3,
        0.7,
        resultPositions[1][0],
        resultPositions[1][1]
      );
      allSplitPositions = resultPositions;
      console.debug(`allSplitPositions ${allSplitPositions.toString()}`);
    });

    it("6.4  I can split an fNFT while the staking pool is paused.", async function() {

      // Creating pool for test
      const currentBlockNumber = await api.query.system.number();
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: POOL_64_BASE_ASSET_ID, // Asset to stake in pool
          startBlock: currentBlockNumber.addn(4), // When pool allows start staking
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // Reward Asset ID
            "640001": {
              rewardRate: {
                period: "PerSecond",
                amount: Pica(100)
              }
            }
          }),
          lock: {
            durationMultipliers: {
              Presets: {
                "0": 1000000000 // 1x default rate
              }
            },
            unlockPenalty: 100_000_000
          },
          shareAssetId: POOL_64_SHARE_ASSET_ID,
          financialNftAssetId: 640000002,
          minimumStakingAmount: Pica(10)
        }
      });

      const {
        data: [resultPoolId]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      const poolId = resultPoolId;

      // Staking in pool for test
      const {
        data: [
          resultStakingPoolId,
          resultStakingOwnerAccountId,
          resultStakingAmount,
          resultStakingDurationPreset,
          resultStakingFNFTCollectionId,
          resultStakingFNFTInstanceId,
          resultStakingRewardMultiplier,
          resultStakingKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(resultPoolId, Pica(100), 0)
      );
      const fNFTCollectionId = resultStakingFNFTCollectionId;
      const fNFTInstanceId = resultStakingFNFTInstanceId;

      const stakeInfoBefore = await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId);

      // Transaction
      const {
        data: [resultPositions]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.SplitPosition.is,
        api.tx.stakingRewards.split(fNFTCollectionId, fNFTInstanceId, 500_000)
      );

      // Verification
      expect(resultPositions.length).to.be.equal(2);

      await verifyPositionSplitting(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        stakeInfoBefore,
        0.5,
        0.5,
        resultPositions[1][0],
        resultPositions[1][1]
      );
    });
  });

  describe("7. Unstaking positions pt. 1.", function() {
    it("7.1  I can unstake my staking position before my lock period has ended and get slashed.", async function() {
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker2.publicKey);
      const stakeAmount = Pica(1_000);
      const fNFTCollectionId = fNFTCollectionId2;
      const fNFTInstanceId = fNFTInstanceId2;

      // Transaction
      const {
        data: [resultStakeOwner, resultFNFTCollectionId, resultFNFTInstanceId, resultSlash]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultStakeOwner.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker2.publicKey).toString()
      );
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);

      // Verifying the slashed amount must be greater than 0, because that'd make no sense.
      // expect(resultSlash.unwrap()).to.be.bignumber.greaterThan(new BN(0));
      // expect(resultSlash.isNone).to.be.false;

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        POOL_11_BASE_ASSET_ID,
        walletStaker2,
        userFundsBefore,
        stakeAmount.toString()
        // true,
        // resultSlash.unwrap()
      );
    });

    it("7.2  [SHORT] I can unstake my staking position after the locking period has ended without getting slashed.",
      async function() {
        // ToDo: Bugged!
        // Ticket: https://app.clickup.com/t/3200gvc

        // Waiting a few blocks to safely unstake funds.
        await waitForBlocks(api, 3);
        // Getting funds before
        const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker.publicKey);
        // Parameters
        const fNFTCollectionId = fNFTCollectionId1;
        const fNFTInstanceId = fNFTInstanceId1;
        const stakeAmount = Pica(1_000);

        // Transaction
        const {
          data: [resultStakeOwner, resultFNFTCollectionId, resultFNFTInstanceId, resultSlash]
        } = await sendAndWaitForSuccess(
          api,
          walletStaker,
          api.events.stakingRewards.Unstaked.is,
          api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
        );
        // Verification
        expect(resultStakeOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
        expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
        expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);
        // expect(resultSlash.isNone).to.be.true; // ToDo

        await verifyPositionUnstaking(
          api,
          fNFTCollectionId,
          fNFTInstanceId,
          POOL_11_BASE_ASSET_ID,
          walletStaker,
          userFundsBefore,
          Number(stakeAmount.toString())
        );
      });

    it("7.3  I can unstake my staking position from PICA pool after the locking period has ended without getting slashed.", async function() {
      this.skip();
      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId4Pica;
      const fNFTInstanceId = fNFTInstanceId4Pica;
      const stakeAmount = Pica(1).toString();

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);

      // ToDo: Verification doesn't work for PICA!
    });

    it("7.4  I can unstake my staking position from PBLO pool after the locking period has ended without getting slashed.", async function() {
      this.skip();
      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId5Pblo;
      const fNFTInstanceId = fNFTInstanceId5Pblo;
      const stakeAmount = Pica(1).toString();

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        5,
        walletStaker,
        userFundsBefore,
        stakeAmount
      );
    });

    it("7.5  I can unstake my staking position from the LP token pool after the locking period has ended without getting slashed.", async function() {
      this.skip();
      // ToDo: LP Token staking does not seem to work!
      // Check 3.6 for further information.

      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker.publicKey); // ToDo: LP Token Pool Reward Asset ID.
      // Parameters
      const fNFTCollectionId = fNFTCollectionId6;
      const fNFTInstanceId = fNFTInstanceId6;
      const stakeAmount = Pica(1).toString();

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker2.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        POOL_11_BASE_ASSET_ID, // ToDo: LP Token Pool Reward Asset ID.
        walletStaker,
        userFundsBefore,
        stakeAmount
      );
    });

    it("7.6  I can unstake my staking position from the 0 time lock pool without getting slashed.", async function() {
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_16_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId7;
      const fNFTInstanceId = fNFTInstanceId7;
      const stakeAmount = Pica(1_000).toString();

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId, resultSlash]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);
      expect(resultSlash.isNone).to.be.true;

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        POOL_16_BASE_ASSET_ID,
        walletStaker,
        userFundsBefore,
        stakeAmount
      );
    });

    it("7.7  I can unstake my position from the 0 unlock penalty pool without getting slashed.", async function() {
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_17_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId8;
      const fNFTInstanceId = fNFTInstanceId8;
      const stakeAmount = Pica(1_000).toString();

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId, resultSlash]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(fNFTCollectionId, fNFTInstanceId)
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(fNFTCollectionId);
      expect(resultFNFTInstanceId).to.be.bignumber.equal(fNFTInstanceId);
      if (!resultSlash.isNone) expect(resultSlash.unwrap()).to.be.bignumber.equal(new BN(0));
      else expect(resultSlash.isNone).to.be.true;

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        POOL_17_BASE_ASSET_ID,
        walletStaker,
        userFundsBefore,
        stakeAmount
      );
    });

    it("7.8  I can unstake all split positions.", async function() {
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_11_BASE_ASSET_ID.toString(),
        walletStaker.publicKey
      );
      // Parameters
      const fNFTCollectionId = allSplitPositions[0][0];
      const fNFTInstanceId = allSplitPositions[0][1];
      const stakeAmount = allSplitPositions[0][2];

      // Transaction
      const {
        data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(api.createType("u128", fNFTCollectionId), api.createType("u64", fNFTInstanceId))
      );

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId).to.be.bignumber.equal(api.createType("u128", fNFTCollectionId));
      expect(resultFNFTInstanceId).to.be.bignumber.equal(api.createType("u64", fNFTInstanceId));

      await verifyPositionUnstaking(
        api,
        api.createType("u128", fNFTCollectionId),
        api.createType("u64", fNFTInstanceId),
        POOL_11_BASE_ASSET_ID,
        walletStaker,
        userFundsBefore,
        stakeAmount.toString()
      );

      // Parameters
      const fNFTCollectionId2 = allSplitPositions[1][0];
      const fNFTInstanceId2 = allSplitPositions[1][1];
      const stakeAmount2 = allSplitPositions[1][2];

      // Transaction
      const {
        data: [resultOwner2, resultFNFTCollectionId2, resultFNFTInstanceId2]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Unstaked.is,
        api.tx.stakingRewards.unstake(
          api.createType("u128", fNFTCollectionId),
          api.createType("u64", fNFTInstanceId)
        )
      );

      // Verification
      expect(resultOwner2.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId2).to.be.bignumber.equal(api.createType("u128", fNFTCollectionId2));
      expect(resultFNFTInstanceId2).to.be.bignumber.equal(api.createType("u64", fNFTInstanceId2));

      await verifyPositionUnstaking(
        api,
        api.createType("u128", fNFTCollectionId),
        api.createType("u64", fNFTInstanceId),
        POOL_11_BASE_ASSET_ID,
        walletStaker,
        userFundsBefore,
        stakeAmount2.toString()
      );
    });
  });
  describe("8. Updating staking rewards pools pt.1.", function() {
    let testUpdatePoolId: number;
    // Emptying pool.
    before(async function() {
      const fNFTCollectionId = fNFTCollectionId12;
      const fNFTInstanceId = fNFTInstanceId12;
      await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );
    });

    it("8.1  I can, as sudo, edit the rewards per second for a reward asset while the pool is paused.",
      async function() {
        // Setting Parameters
        const fNFTCollectionId = fNFTCollectionId12;
        const fNFTInstanceId = fNFTInstanceId12;
        // Get funds before transaction
        const userFundsBefore = await api.rpc.assets.balanceOf(
          POOL_18_REWARD_ASSET_ID_1.toString(),
          walletStaker.publicKey
        );
        const userFundsBeforeA = await api.query.tokens.accounts(
          walletStaker.publicKey,
          POOL_18_REWARD_ASSET_ID_1.toString()
        );
        const newRewardRate = Pica(10);
        POOL_18_REWARD_RATE = newRewardRate;
        const newRewardConfiguration = api.createType("BTreeMap<u128, ComposableTraitsStakingRewardUpdate>", {
          // Reward Asset ID
          "80001": {
            rewardRate: {
              period: "PerSecond",
              amount: newRewardRate
            }
          }
        });
        const { data: [resultPoolId] } = await sendAndWaitForSuccess(
          api,
          sudoKey,
          api.events.stakingRewards.RewardPoolUpdated.is,
          api.tx.sudo.sudo(api.tx.stakingRewards.updateRewardsPool(stakingPoolId8, newRewardConfiguration))
        );
        expect(resultPoolId).to.be.bignumber.equal(stakingPoolId8);
      });
  });

  let blockNumAt26Update: number;
  describe("2. Adding rewards to pool pots. pt. 3", function() {
    this.timeout(6 * 60 * 1000);
    before(async function() {
      // Emptying pool by a final claim and waiting a few blocks for the pool to pause.
      await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId12, fNFTInstanceId12)
      );
    });

    it("2.6  I can add rewards to a staking rewards pool that previously depleted.",
      async function() {
        // Parameters
        const poolId = stakingPoolId8;
        const assetId = POOL_18_REWARD_ASSET_ID_1;
        const amount = Pica(100_000_000);
        const keepAlive = true;
        const walletBalanceBefore = await api.rpc.assets.balanceOf(assetId.toString(), walletRewardAdder.publicKey);

        const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId);
        const poolRewardAmountBefore = poolInfoBefore.unwrap().rewards.toHuman()[assetId];
        const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
        console.debug(`isEmpty ${poolRewardStateBefore.toString()}`);
        // expect(poolRewardStateBefore.isNone).to.be.false; // ToDo?
        // Transaction
        const {
          data: [resultPoolId, resultAssetId, resultAmount]
        } = await sendAndWaitForSuccess(
          api,
          walletRewardAdder,
          api.events.stakingRewards.RewardsPotIncreased.is,
          api.tx.stakingRewards.addToRewardsPot(poolId, assetId, amount, keepAlive)
        );
        blockNumAt26Update = Number((await api.query.system.number()).toString()) + 1;

        // Verification
        // Verifying the poolId, reported by the event, is correct.
        expect(poolId).to.be.bignumber.equal(resultPoolId);
        // Verifying the reward asset id, reported by the event, is equal to the reward asset we added to the pool.
        expect(new BN(assetId.toString())).to.be.bignumber.equal(resultAssetId);
        // Verifying the added amount, reported by the event, is equal to the amount we added.
        expect(new BN(amount.toString())).to.be.bignumber.equal(resultAmount);
        await verifyPoolPotAddition(
          api,
          poolId,
          assetId,
          amount,
          walletRewardAdder,
          walletBalanceBefore,
          // @ts-ignore
          new BN(poolRewardAmountBefore["totalRewards"].replaceAll(",", ""))
          // new BN(parseInt(poolRewardAmountBefore["totalRewards"].toString().replace('0x', ''), 16).toString())
        );
      });
  });

  describe("3. Staking in the pools pt. 3", function() {
    it("3.12  I can stake in a pool with multiple reward asset IDs which is depleted.",
      async function() {
        // Getting funds before transaction
        const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
        // Parameters
        const durationPreset1 = 600;
        const stakeAmount = Pica(100);

        // ToDo: Depl!
        // Transaction
        const {
          data: [
            resultPoolId,
            resultOwnerAccountId,
            resultAmount,
            resultDurationPreset,
            resultFNFTCollectionId,
            resultFNFTInstanceId,
            resultRewardMultiplier,
            resultKeepAlive
          ]
        } = await sendAndWaitForSuccess(
          api,
          walletStaker,
          api.events.stakingRewards.Staked.is,
          api.tx.stakingRewards.stake(stakingPoolId2, stakeAmount, durationPreset1),
          false
        );

        // Verification
        expect(resultPoolId).to.be.bignumber.equal(stakingPoolId2);
        expect(resultOwnerAccountId.toString()).to.be.equal(
          api.createType("AccountId32", walletStaker.publicKey).toString()
        );
        expect(resultAmount.toString()).to.equal(stakeAmount.toString());
        expect(resultDurationPreset.toString()).to.equal(durationPreset1.toString());
        expect(resultKeepAlive);
        const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);
        const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
          await api.query.stakingRewards.rewardPools(stakingPoolId2)
        );
        const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_12_SHARE_ASSET_ID);
        inflationStake312_1 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        inflationStake312_2 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_2.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        inflationStake312_3 = (BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_12_REWARD_ASSET_ID_3.toString()].totalRewards.toString()),
            BigNumber(stakeInfo.unwrap().share.toString())
          ).toString()
        ));
        await verifyPoolStaking(
          api,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          (stakeAmount).toString(),
          (stakeAmount).toString(),
          stakingPoolId2,
          walletStaker,
          userFundsBefore
        );
      });
  });

  describe("4. Claiming from the staked positions pt. 3", function() {
    it("4.13  I can claim unclaimed funds from a pool that was paused for some blocks but more funds were added meanwhile.",
      async function() {
        // Get funds before transaction
        const userFundsBefore = await api.rpc.assets.balanceOf(
          POOL_18_REWARD_ASSET_ID_1.toString(),
          walletStaker.publicKey
        );
        const userFundsBeforeA = await api.query.tokens.accounts(
          walletStaker.publicKey,
          POOL_18_REWARD_ASSET_ID_1.toString()
        );

        // Setting Parameters
        const fNFTCollectionId = fNFTCollectionId12;
        const fNFTInstanceId = fNFTInstanceId12;
        const stakingPoolId = stakingPoolId8;

        const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
          await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
        );
        // Getting pool info before transaction, to calculate the claimable amount.
        const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
          await api.query.stakingRewards.rewardPools(stakingPoolId)
        );
        // Getting total issuance of the defined share asset, used for claimable amount calculation.
        const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_18_SHARE_ASSET_ID);
        // inflationStake38 = BigNumber(
        //   getStakeReduction(
        //     BigNumber(totalShareAssetIssuance.toString()),
        //     // @ts-ignore
        //     BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_18_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
        //     BigNumber(stakeInfoBefore.unwrap().share.toString())
        //   ).toString()
        // );

        const stakerSharePart = await getNthStakerSharesPart(
          api,
          POOL_18_SHARE_ASSET_ID,
          BigNumber(stakeInfoBefore.unwrap().share.toString()));
        console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);
        // Calculating claimable amount.
        const verifAmtAlt = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
          api,
          stakerSharePart,
          blockNumAtStake38,
          BigNumber(POOL_18_REWARD_RATE.toString())
        )).plus(inflationStake38);
        const verifAmt = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
          api,
          stakerSharePart,
          blockNumAt26Update,
          BigNumber(POOL_18_REWARD_RATE.toString())
        ));
        console.debug(`verifAmt4 ${verifAmt.toString()}`);
        console.debug(`verifAmtAlt ${verifAmtAlt.toString()}`);
        console.debug(`inf ${inflationStake38.toString()}`);
        console.debug(`verifAmt4 - inf ${verifAmt.minus(inflationStake38).toString()}`);

        // Transaction
        const {
          data: [resultOwner, resultFNFTCollectionId, resultFNFTInstanceId]
        } = await sendAndWaitForSuccess(
          api,
          walletStaker,
          api.events.stakingRewards.Claimed.is,
          api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
        );

        // Verification
        expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
        expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
        expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

        const userFundsAfterA = await api.query.tokens.accounts(
          walletStaker.publicKey,
          POOL_18_REWARD_ASSET_ID_1.toString()
        );
        console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);

        inflationStake312 = BigNumber(
          getStakeReduction(
            BigNumber(totalShareAssetIssuance.toString()),
            // @ts-ignore
            BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_18_REWARD_ASSET_ID_1.toString()].totalRewards.toString()),
            BigNumber(stakeInfoBefore.unwrap().share.toString())
          ).toString()
        );
        console.debug(`inf after ${BigNumber(inflationStake312.toString())}`);

        await verifyPoolClaiming(
          api,
          fNFTCollectionId,
          fNFTInstanceId,
          [POOL_18_REWARD_ASSET_ID_1],
          walletStaker,
          [userFundsBefore],
          [api.createType("u128", verifAmt.toFixed(0))]
        );
      });
  });

  describe("8. Updating staking rewards pools pt.2.", function() {

    it("8.2  I can modify more than a single reward asset ID.");
  });


  describe("7. Unstaking positions pt. 2.", function() {
    this.timeout(4 * 60 * 1000);
    let api2: ApiPromise;
    let fNFTCollectionId79: number;
    let fNFTInstanceId79: number;
    before(async function() {
      // Getting 2nd connection for simultaneous transaction.
      const { newClient } = await getNewConnection();
      api2 = newClient;

      // Creating 2nd stake.
      const {
        data: [
          resultPoolId,
          resultOwnerAccountId,
          resultAmount,
          resultDurationPreset,
          resultFNFTCollectionId,
          resultFNFTInstanceId,
          resultRewardMultiplier,
          resultKeepAlive
        ]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId8, Pica(100), 0),
        false
      );
      fNFTCollectionId79 = resultFNFTCollectionId.toNumber();
      fNFTInstanceId79 = resultFNFTInstanceId.toNumber();
    });

    after(async function() {
      await api2.disconnect();
    })

    it("7.9  Me and someone else can simultaneously unstake our positions.", async function() {
      const userFundsBefore1 = await api.rpc.assets.balanceOf(POOL_18_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      const userFundsBefore2 = await api.rpc.assets.balanceOf(POOL_18_BASE_ASSET_ID.toString(), walletStaker2.publicKey);
      const stakeAmount1 = Pica(1_000);
      const stakeAmount2 = Pica(100);
      const [
        {
          data: [resultStakeOwner, resultFNFTCollectionId, resultFNFTInstanceId, resultSlash]
        },
        {
          data: [result2StakeOwner, result2FNFTCollectionId, result2FNFTInstanceId, result2Slash]
        }
      ] = await Promise.all([
        sendAndWaitForSuccess(
          api,
          walletStaker,
          api.events.stakingRewards.Unstaked.is,
          api.tx.stakingRewards.unstake(fNFTCollectionId12, fNFTInstanceId12)
        ), sendAndWaitForSuccess(
          api2,
          walletStaker2,
          api.events.stakingRewards.Unstaked.is,
          api.tx.stakingRewards.unstake(fNFTCollectionId79, fNFTInstanceId79)
        )]);

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId12,
        fNFTInstanceId12,
        POOL_18_BASE_ASSET_ID,
        walletStaker,
        userFundsBefore1,
        stakeAmount1.toString()
        // true,
        // resultSlash.unwrap()
      );
      await verifyPositionUnstaking(
        api2,
        api.createType("u128", fNFTCollectionId79),
        api.createType("u64", fNFTInstanceId79),
        POOL_18_BASE_ASSET_ID,
        walletStaker2,
        userFundsBefore2,
        stakeAmount2.toString()
        // true,
        // resultSlash.unwrap()
      );
    });
  });
});
