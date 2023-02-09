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
    fNFTCollectionId8: u128;
  let fNFTInstanceId1: u64,
    fNFTInstanceId2: u64,
    fNFTInstanceId3: u64,
    fNFTInstanceId4Pica: u64,
    fNFTInstanceId5Pblo: u64,
    fNFTInstanceId6: u64,
    fNFTInstanceId7: u64,
    fNFTInstanceId8: u64;
  let allSplitPositions: Vec<ITuple<[u128, u64, u128]>>;

  const inflations: BigNumber[] = [];
  let blockNumAtStake31: number, blockNumAtStake32: number, blockNumAtStake37: number, blockNumAtStake38: number;

  const POOL_11_BASE_ASSET_ID = 10000;
  const POOL_11_SHARE_ASSET_ID = 10000001;
  const POOL_11_REWARD_ASSET_ID = 10001;
  const POOL_11_REWARD_RATE = Pica(1);
  let poolStartTime: number;
  let pool11startBlock: number;
  let stakeTimestampPool11_1: number;
  let stakeTimestampPool11_2: number;

  const POOL_12_BASE_ASSET_ID = 20000;
  const POOL_12_SHARE_ASSET_ID = 20000001;
  const POOL_12_REWARD_ASSET_ID_1 = 20001;
  const POOL_12_REWARD_ASSET_ID_2 = 20002;
  const POOL_12_REWARD_ASSET_ID_3 = 20003;
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
  let stakeTimestampPool16;

  const POOL_17_BASE_ASSET_ID = 70000;
  const POOL_17_SHARE_ASSET_ID = 70000001;
  const POOL_17_REWARD_RATE = Pica(1);
  const POOL_17_REWARD_ASSET_ID = 70001;
  let stakeTimestampPool17;

  const PICA_ASSET_ID = 1;
  const PBLO_ASSET_ID = 5;

  let stakingPoolId1: u128,
    stakingPoolId2: u128,
    stakingPoolId3: u128,
    stakingPoolId4: u128,
    stakingPoolId5: u128,
    stakingPoolId6: u128,
    stakingPoolId7: u128;

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
        PICA_ASSET_ID, POOL_11_BASE_ASSET_ID, POOL_12_BASE_ASSET_ID, POOL_13_BASE_ASSET_ID, POOL_14_BASE_ASSET_ID, POOL_16_BASE_ASSET_ID, POOL_17_BASE_ASSET_ID
      ],
      Pica(10_000)
    );
    await mintAssetsToWallet(
      api,
      walletStaker2,
      sudoKey,
      [PICA_ASSET_ID, POOL_11_BASE_ASSET_ID, POOL_12_BASE_ASSET_ID, POOL_13_BASE_ASSET_ID, POOL_14_BASE_ASSET_ID, POOL_16_BASE_ASSET_ID, POOL_17_BASE_ASSET_ID],
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
        POOL_14_REWARD_ASSET_ID, POOL_16_REWARD_ASSET_ID, POOL_17_REWARD_ASSET_ID],
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
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(32));
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
          endBlock: endBlock, // Pool ends at this block
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
        data: [resultPoolId, resultOwner, resultEndBlock]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );

      // After waiting for our event, we make sure the defined staking asset id
      // is the same as the pool id.
      expect(resultPoolId).to.be.bignumber.equal(assetId);
      expect(resultEndBlock).to.be.bignumber.equal(endBlock);
      stakingPoolId1 = resultPoolId;

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId1,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.2  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with multiple reward assets.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(32));
      const assetId = api.createType("u128", POOL_12_BASE_ASSET_ID);
      const amount1 = Pica(1);
      const amount2 = Pica(5);
      const amount3 = Pica(1);
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
          endBlock: endBlock,
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
        data: [resultPoolId, resultOwner, resultEndBlock]
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
      expect(resultEndBlock.toNumber()).to.be.equal(endBlock.toNumber());

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId2,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        endBlock,
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
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
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
          endBlock: endBlock,
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
        data: [resultPoolId, resultOwner, resultEndBlock]
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
      expect(resultEndBlock.toNumber()).to.be.equal(endBlock.toNumber());

      // // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId3,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.4  I can, using governance, create a new Staking Rewards pool for any arbitrary asset ID.");

    it("1.5  I can create a Pablo pool using sudo & an LP token pool will get automatically created.");

    it("1.6  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero time locks.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(32));
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
          endBlock: endBlock,
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
        data: [resultPoolId, resultOwner, resultEndBlock]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      // After waiting for our event, we make sure the defined staking asset id
      // is the same as the pool id.
      expect(resultPoolId).to.be.bignumber.equal(assetId);
      expect(resultEndBlock.toNumber()).to.be.equal(endBlock.toNumber());
      stakingPoolId6 = resultPoolId;

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId6,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.7  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero penalty locks.", async function() {
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(32));
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
          endBlock: endBlock,
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
        data: [resultPoolId, resultOwner, resultEndBlock]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.stakingRewards.RewardPoolCreated.is,
        api.tx.sudo.sudo(api.tx.stakingRewards.createRewardPool(poolConfig))
      );
      // After waiting for our event, we make sure the defined staking asset id
      // is the same as the pool id.
      expect(resultPoolId).to.be.bignumber.equal(assetId);
      expect(resultEndBlock.toNumber()).to.be.equal(endBlock.toNumber());
      stakingPoolId7 = resultPoolId;

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId7,
        resultOwner,
        walletPoolOwner.publicKey,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });
  })
  ;

  describe("2. Adding rewards to pool pots.", function() {
    it("2.1  [SHORT] I can, as pool owner, add rewards to staking rewards pool pot #1.1.", async function() {
      // Parameters
      const poolId = stakingPoolId1;
      const assetId = POOL_11_REWARD_ASSET_ID;
      const amount = Pica(100_000_000_000);
      const keepAlive = true;
      const walletBalanceBefore = await api.rpc.assets.balanceOf(assetId.toString(), walletPoolOwner.publicKey);

      const poolInfoBefore = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(poolId);
      const poolRewardAmountBefore = poolInfoBefore.unwrap().rewards.toJSON()[assetId.toString()];
      const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId); // ToDo
      expect(poolRewardStateBefore.isNone).to.be.false;
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
      debugger;
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

      const poolRewardStateBefore = await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId); // ToDo
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
        new BN(poolRewardAmountBefore.toString())
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
      await verifyPoolPotAddition(api, poolId, assetIds[0], amount, walletRewardAdder, walletBalancesBefore[0], new BN(poolRewardAmountsBefore[0].toString()));
      // @ts-ignore
      await verifyPoolPotAddition(api, poolId, assetIds[1], amount, walletRewardAdder, walletBalancesBefore[1], new BN(poolRewardAmountsBefore[1].toString()));
      // @ts-ignore
      await verifyPoolPotAddition(api, poolId, assetIds[2], amount, walletRewardAdder, walletBalancesBefore[2], new BN(poolRewardAmountsBefore[2].toString()));
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
        verifyPoolPotAddition(api, poolIds[0], assetIds[0], amount, walletRewardAdder, walletBalancesBefore[0], new BN(poolRewardAmountsBefore[0].toString())),
        // @ts-ignore
        verifyPoolPotAddition(api, poolIds[1], assetIds[1], amount, walletRewardAdder, walletBalancesBefore[1], new BN(poolRewardAmountsBefore[1].toString())),
        // @ts-ignore
        verifyPoolPotAddition(api, poolIds[2], assetIds[2], amount, walletRewardAdder, walletBalancesBefore[2], new BN(poolRewardAmountsBefore[2].toString()))
      ]);

    });
  });

  describe("3. Staking in the pools", function() {
    it("3.1  [SHORT] I can stake in the newly created rewards pool #1.1", async function() {
      const currentBlockNumber = (await api.query.system.number());
      if (currentBlockNumber.lt(new BN(pool11startBlock.toString())))
        await waitForBlocks(api, Number(pool11startBlock - currentBlockNumber.toNumber()));

      // Parameters
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      const durationPreset = 0;
      const stakeAmount = Pica(1_000);

      stakeTimestampPool11_1 = Number(await api.query.timestamp.now()) + 12;
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
      inflations.push(BigNumber(
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
        stakingPoolId1,
        walletStaker,
        userFundsBefore
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
      )).plus(BigNumber(inflations[0].toString()));
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

      inflations[0] = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      console.debug(`infAft ${inflations[0].toString()}`);

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [POOL_11_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        api.createType("u128", verifAmt4th.toString())
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
      // @ts-ignore
      // inflationAtStake32 = Number(stakeInfo.unwrap().reductions.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].toString());
      stakeTimestampPool11_2 = await api.query.timestamp.now();

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

      inflations.push(BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfo.unwrap().share.toString())
        ).toString()
      ));
      console.debug(`stake31Reduction ${inflations[1].toString()}`);
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
        stakingPoolId1,
        walletStaker2,
        userFundsBefore
      );
    });

    it("3.3 I can stake in the newly created rewards pool #1.2", async function() {
      // Getting funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 600;
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
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId2, stakeAmount, durationPreset)
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
      expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // Verifying the keepAlive parameter, reported by the event, is correct.
      expect(resultKeepAlive);
      fNFTCollectionId3 = resultFNFTCollectionId;
      fNFTInstanceId3 = resultFNFTInstanceId;
      const stakeInfo = await api.query.stakingRewards.stakes(resultFNFTCollectionId, resultFNFTInstanceId);
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId1)
      );
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_12_SHARE_ASSET_ID);
      inflations.push(BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfo.unwrap().share.toString())
        ).toString()
      ));
      await verifyPoolStaking(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeAmount.toString(),
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
        api.createType("u128", pbloPoolId),
        walletStaker,
        userFundsBefore
      );
    });

    it("3.6  I can stake in the created LP token pool #1.5.", async function() {

      // ToDo: Buggy!
      // Ticket: https://app.clickup.com/t/32009xc

      // Get funds before transaction
      // const userFundsBefore = await api.rpc.assets.balanceOf(pool15LpTokenId.toString(), walletStaker2.publicKey);

      // const {
      //   data: [resultWho, resultPabloPoolId, resultBaseAmount, resultQuoteAmount, resultMintedLp]
      // } = await sendAndWaitForSuccess(
      //   api,
      //   walletStaker,
      //   api.events.pablo.LiquidityAdded.is,
      //   api.tx.pablo.addLiquidity(pool15PabloPoolId, 100_000_000_000, 100_000_000_000, 500_000, true)
      // );

      // const stakeAmount = resultMintedLp.div(new BN(4));
      // const durationPreset = 604800;
      // Transaction
      // const {
      //   data: [
      //     resultPoolId,
      //     resultOwnerAccountId,
      //     resultAmount,
      //     resultDurationPreset,
      //     resultFNFTCollectionId,
      //     resultFNFTInstanceId,
      //     resultRewardMultiplier,
      //     resultKeepAlive
      //   ]
      // } = await sendAndWaitForSuccess(
      //   api,
      //   walletStaker2,
      //   api.events.stakingRewards.Staked.is,
      //   api.tx.stakingRewards.stake(pool15LpTokenId, stakeAmount, durationPreset)
      // );

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      // expect(resultPoolId).to.be.bignumber.equal(pool15LpTokenId);
      // // Verifying the pool owner, reported by the event, is reported correctly.
      // expect(resultOwnerAccountId.toString()).to.be.equal(
      //   api.createType("AccountId32", walletStaker2.publicKey).toString()
      // );
      // // Verifying the amount, reported by the event, is correct.
      // expect(resultAmount.toString()).to.equal(stakeAmount.toString());
      // // Verifying the durationPreset equals our requested durationPreset.
      // expect(resultDurationPreset.toString()).to.equal(durationPreset.toString());
      // // Verifying the keepAlive parameter, reported by the event, is correct.
      // expect(resultKeepAlive);
      // fNFTCollectionId6 = resultFNFTCollectionId;
      // fNFTInstanceId6 = resultFNFTInstanceId;
      //
      // await verifyPoolStaking(
      //   api,
      //   fNFTCollectionId6,
      //   fNFTInstanceId6,
      //   stakeAmount.toString(),
      //   pool15LpTokenId,
      //   walletStaker,
      //   userFundsBefore
      // );
    });

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
      inflations.push(BigNumber(
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
      inflations.push(BigNumber(
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
        stakingPoolId7,
        walletStaker,
        userFundsBefore
      );
    });
  });

  describe("4. Claiming from staked positions.", function() {
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
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake32,
        BigNumber(POOL_11_REWARD_RATE.toString())
      ));
      console.debug(`verifAmt4 ${verifAmt4th.toString()}`);

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

      inflations[1] = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      console.debug(`inf after ${BigNumber(inflations[1].toString())}`);


      await verifyPoolClaiming(
        api,
        fNFTCollectionId2,
        fNFTInstanceId2,
        [POOL_11_REWARD_ASSET_ID],
        walletStaker2,
        [userFundsBefore],
        api.createType("u128", verifAmt4th.toString())
      );
    });

    // ToDo: Add actual test
    it("4.3  I can claim from the arbitrary asset pool in #1.2 using the stake from #3.3 after the lock period has ended.", async function() {
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
      inflations[0] = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );

      const stakerSharePart = await getNthStakerSharesPart(
        api,
        POOL_11_SHARE_ASSET_ID,
        BigNumber(stakeInfoBefore.unwrap().share.toString()));
      console.debug(`getNthStakerSharesPart ${stakerSharePart.toString()}`);
      // Calculating claimable amount.
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake31,
        BigNumber(POOL_11_REWARD_RATE.toString())
      ));
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

      // Verification
      expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker.publicKey).toString());
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      const userFundsAfterA = await api.query.tokens.accounts(
        walletStaker.publicKey,
        POOL_11_REWARD_ASSET_ID.toString()
      );
      console.debug(`cA ${BigNumber(userFundsAfterA.free.toString()).minus(BigNumber(userFundsBeforeA.free.toString()))}`);

      inflations[0] = BigNumber(
        getStakeReduction(
          BigNumber(totalShareAssetIssuance.toString()),
          // @ts-ignore
          BigNumber(poolInfo.unwrap().rewards.toJSON()[POOL_11_REWARD_ASSET_ID.toString()].totalRewards.toString()),
          BigNumber(stakeInfoBefore.unwrap().share.toString())
        ).toString()
      );
      console.debug(`inf after ${BigNumber(inflations[0].toString())}`);

      await verifyPoolClaiming(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        [POOL_11_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        api.createType("u128", verifAmt4th.toString())
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
        api.createType("u128", claimableAmount)
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
        api.createType("u128", claimableAmount)
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
      console.debug(`inf ${BigNumber(inflations[3].toString())}`);

      // Calculating claimable amount.
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake37,
        BigNumber(POOL_16_REWARD_RATE.toString())
      )).plus(BigNumber(inflations[3].toFixed()));
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
        api.createType("u128", verifAmt4th.toString())
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
      console.debug(`inf ${BigNumber(inflations[4].toString())}`);

      // Calculating claimable amount.
      const verifAmt4th = (await getNthStakerRewardAlternativeTry4BasedBlocktime(
        api,
        stakerSharePart,
        blockNumAtStake38,
        BigNumber(POOL_17_REWARD_RATE.toString())
      )).plus(BigNumber(inflations[4].toFixed()));
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
        api.createType("u128", verifAmt4th.toString())
      );
    });
  });

  describe("5. Extending existing positions.", function() {
    it("5.1  [SHORT] I can extend the staked amount in pool #1.1 using the stake from #3.3", async function() {
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
  });

  describe("7. Unstaking positions.", function() {
    it("7.1  I can unstake my staking position before my lock period has ended and get slashed.", async function() {
      // Getting user funds before
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

    it("7.2  [SHORT] I can unstake my staking position after the locking period has ended without getting slashed.", async function() {
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
});
