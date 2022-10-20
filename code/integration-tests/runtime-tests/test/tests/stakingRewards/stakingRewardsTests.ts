import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { ComposableTraitsStakingRewardPool, ComposableTraitsStakingStake } from "@composable/types/interfaces";
import { Option, u128, u64, Vec } from "@polkadot/types-codec";
import BN from "bn.js";
import { before } from "mocha";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import {
  getClaimOfStake,
  verifyPoolClaiming,
  verifyPoolCreationUsingQuery,
  verifyPoolPotAddition,
  verifyPoolStaking,
  verifyPositionExtension,
  verifyPositionSplitting,
  verifyPositionUnstaking
} from "@composabletests/tests/stakingRewards/testHandlers/stakingRewardsTestHelper";
import { ISubmittableResult, ITuple } from "@polkadot/types/types";
import { SubmittableExtrinsic } from "@polkadot/api/types";

/**
 * Staking Rewards Pallet Tests
 */
describe("tx.stakingRewards Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;
  this.retries(0);

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

  const POOL_11_BASE_ASSET_ID = 10000;
  const POOL_11_SHARE_ASSET_ID = 10000001;
  const POOL_11_REWARD_ASSET_ID = 10001;

  const POOL_12_BASE_ASSET_ID = 20000;
  const POOL_12_SHARE_ASSET_ID = 20000001;
  const POOL_12_REWARD_ASSET_ID_1 = 20001;
  const POOL_12_REWARD_ASSET_ID_2 = 20002;
  const POOL_12_REWARD_ASSET_ID_3 = 20003;

  const POOL_13_BASE_ASSET_ID = 30000;
  const POOL_13_SHARE_ASSET_ID = 30000001;
  const POOL_13_REWARD_ASSET_ID = 30001;

  const POOL_14_BASE_ASSET_ID = 40000;
  const POOL_14_SHARE_ASSET_ID = 40000001;
  const POOL_14_REWARD_ASSET_ID = 40001;

  const POOL_15_PBLO_BASE_ASSET_ID = 50000;
  const POOL_15_PBLO_QUOTE_ASSET_ID = 50001;
  const POOL_15_SHARE_ASSET_ID = 21474836476;
  let pool15LpTokenId: u128, pool15PabloPoolId: u128;

  const POOL_16_BASE_ASSET_ID = 60000;
  const POOL_16_SHARE_ASSET_ID = 60000001;
  const POOL_16_REWARD_ASSET_ID = 60001;

  const POOL_17_BASE_ASSET_ID = 70000;
  const POOL_17_SHARE_ASSET_ID = 70000001;
  const POOL_17_REWARD_ASSET_ID = 70001;

  const PICA_ASSET_ID = 1;
  const PBLO_ASSET_ID = 5;

  let stakingPoolId1: u128,
    stakingPoolId2: u128,
    stakingPoolId3: u128,
    stakingPoolId4: u128,
    stakingPoolId5: u128,
    stakingPoolId6: u128,
    stakingPoolId7: u128;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
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

  before("Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(
      api,
      walletStaker,
      sudoKey,
      [
        PICA_ASSET_ID,
        PBLO_ASSET_ID,
        POOL_11_BASE_ASSET_ID,
        POOL_12_BASE_ASSET_ID,
        POOL_13_BASE_ASSET_ID,
        POOL_14_BASE_ASSET_ID,
        POOL_15_PBLO_BASE_ASSET_ID,
        POOL_15_PBLO_QUOTE_ASSET_ID,
        POOL_16_BASE_ASSET_ID,
        POOL_17_BASE_ASSET_ID
      ],
      1_000_000_000_000_000n
    );
    await mintAssetsToWallet(
      api,
      walletStaker2,
      sudoKey,
      [PICA_ASSET_ID, POOL_11_BASE_ASSET_ID],
      1_000_000_000_000_000n
    );
    await mintAssetsToWallet(
      api,
      walletPoolOwner,
      sudoKey,
      [
        PICA_ASSET_ID,
        PBLO_ASSET_ID,
        POOL_11_BASE_ASSET_ID,
        POOL_11_REWARD_ASSET_ID,
        POOL_12_BASE_ASSET_ID,
        POOL_12_REWARD_ASSET_ID_1,
        POOL_12_REWARD_ASSET_ID_2,
        POOL_12_REWARD_ASSET_ID_3,
        POOL_13_REWARD_ASSET_ID,
        POOL_14_REWARD_ASSET_ID,
        POOL_16_REWARD_ASSET_ID,
        POOL_17_REWARD_ASSET_ID
      ],
      1_000_000_000_000_000n
    );
    await mintAssetsToWallet(
      api,
      walletRewardAdder,
      sudoKey,
      [1, PBLO_ASSET_ID, POOL_11_REWARD_ASSET_ID],
      1_000_000_000_000_000n
    );
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("1. Creation of reward pools.", function () {
    it("1.1  [SHORT] I can, as sudo, create a new Staking Rewards pool, for any arbitrary asset ID, with a single reward asset.", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
      const assetId = api.createType("u128", POOL_11_BASE_ASSET_ID);
      const maxRewards = api.createType("u128", 100 * 10 ** 12);
      const rewardPeriodPerSecond = "10";
      const amount = (10 ** 12).toString();
      const durationPreset = {
        "12": "1200000000",
        "600": "1200000000",
        "1200": "1500000000"
      };
      const unlockPenalty = 100000000;
      const shareAssetId = POOL_11_SHARE_ASSET_ID;
      const financialNftAssetId = 10000002;
      const minimumStakingAmount = 10 ** 12;
      // Creating pool config parameter
      const lock = api.createType("ComposableTraitsStakingLockLockConfig", {
        durationPresets: durationPreset,
        unlockPenalty: unlockPenalty
      });
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          endBlock: endBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // Reward Asset ID
            "10001": {
              maxRewards: maxRewards,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount
              }
            }
          }),
          lock: lock,
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
        [api.createType("u128", POOL_11_REWARD_ASSET_ID)],
        maxRewards,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.2  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with multiple reward assets.", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(16));
      const assetId = api.createType("u128", POOL_12_BASE_ASSET_ID);
      const maxRewards1 = (100 * 10 ** 12).toString();
      const maxRewards2 = (100 * 10 ** 12).toString();
      const maxRewards3 = (100 * 10 ** 12).toString();
      const rewardPeriodPerSecond1 = "10";
      const rewardPeriodPerSecond2 = "100";
      const rewardPeriodPerSecond3 = "1000";
      const amount1 = (0.1 * 10 ** 12).toString();
      const amount2 = (0.5 * 10 ** 12).toString();
      const amount3 = (10 ** 12).toString();
      const durationPreset = {
        "600": "1200000000",
        "1200": "1500000000"
      };
      const unlockPenalty = "100000000";
      const shareAssetId = POOL_12_SHARE_ASSET_ID;
      const financialNftAssetId = 20000002;
      const minimumStakingAmount = 10 ** 12;
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
              maxRewards: maxRewards1,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond1
                },
                amount: amount1
              }
            },
            "20002": {
              maxRewards: maxRewards2,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond2
                },
                amount: amount2
              }
            },
            "20003": {
              maxRewards: maxRewards3,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond3
                },
                amount: amount3
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
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
        [
          api.createType("u128", POOL_12_REWARD_ASSET_ID_3),
          api.createType("u128", POOL_12_REWARD_ASSET_ID_2),
          api.createType("u128", POOL_13_REWARD_ASSET_ID)
        ],
        api.createType("u128", maxRewards1),
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.3  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with single duration preset.", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
      const assetId = api.createType("u128", POOL_13_BASE_ASSET_ID);
      const maxRewards = (100 * 10 ** 12).toString();
      const rewardPeriodPerSecond = "10";
      const amount = (10 ** 12).toString();
      const durationPreset = {
        "600": "1200000000",
        "1200": "1500000000"
      };
      const unlockPenalty = "100000000";
      const shareAssetId = POOL_13_SHARE_ASSET_ID;
      const financialNftAssetId = 30000002;
      const minimumStakingAmount = 10 ** 12;
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
              maxRewards: maxRewards,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
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

      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId3,
        resultOwner,
        walletPoolOwner.publicKey,
        [api.createType("u128", POOL_13_REWARD_ASSET_ID)],
        api.createType("u128", maxRewards),
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.4  I can, using governance, create a new Staking Rewards pool for any arbitrary asset ID.", async function () {
      this.timeout(5 * 60 * 1000);

      // ToDo: Postponed for now!
      // The whole block has been commented, due to compiler issues with the skip!
      this.skip();

      /*
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
      const assetId = api.createType("u128", POOL_14_BASE_ASSET_ID);
      const maxRewards = api.createType("u128", 100 * 10 ** 12);
      const rewardPeriodPerSecond = "10";
      const amount = (0.1 * 10 ** 12).toString();
      const durationPreset = {
        "600": "1200000000",
        "1200": "1500000000"
      };
      const unlockPenalty = "100000000";
      const shareAssetId = POOL_14_SHARE_ASSET_ID;
      const financialNftAssetId = 40000002;
      const minimumStakingAmount = 10 ** 12;
      // Creating pool config parameter
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          endBlock: endBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "40001": {
              maxRewards: maxRewards,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
            unlockPenalty: unlockPenalty
          },
          shareAssetId: shareAssetId,
          financialNftAssetId: financialNftAssetId,
          minimumStakingAmount: minimumStakingAmount
        }
      });

      // Transaction
      const proposalHash = api.tx.stakingRewards.createRewardPool(poolConfig).toHex();
      // Creating Proposal
      const {
        data: [proposalId]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.externalProposeMajority(proposalHash))
      );
      expect(proposalId).to.not.be.an("Error");
      // Scheduling Proposal
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.fastTrack(proposalHash, 8, 0))
      );
      expect(result.isOk).to.be.true;

      // Voting Proposal
      await Promise.all([
        sendAndWaitForSuccess(
          api,
          sudoKey,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            api.createType("u128", proposalId),
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletPoolOwner,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            api.createType("u128", proposalId),
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletStaker,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            api.createType("u128", proposalId),
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletStaker2,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            api.createType("u128", proposalId),
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        )
      ]);
      await waitForBlocks(api);
      const vote1 = await api.query.democracy.votingOf(sudoKey.publicKey);
      const vote2 = await api.query.democracy.votingOf(walletPoolOwner.publicKey);
      const vote3 = await api.query.democracy.votingOf(walletStaker.publicKey);
      const vote4 = await api.query.democracy.votingOf(walletStaker2.publicKey);

      expect(vote1.asDirect.votes.length).to.equal(1);
      expect(vote2.asDirect.votes.length).to.equal(1);
      expect(vote3.asDirect.votes.length).to.equal(1);
      expect(vote4.asDirect.votes.length).to.equal(1);

      // Waiting for succession
      let success = false;
      let resultOwner: AccountId32 | undefined = undefined;
      do {
        const currentEvents = await api.query.system.events();
        currentEvents.forEach(event => {
          if (event.event.section.toString() === "democracy") {
            if (event.event.method.toString() === "Passed") {
              success = true;
              currentEvents.forEach(function (event) {
                if (event.event.method.toString() === "RewardPoolCreated") {
                  stakingPoolId4 = <u128>event.event.data[0];
                  resultOwner = <AccountId32>event.event.data[1];
                }
              });
              return;
            }
            if (event.event.method.toString() === "NotPassed") throw new Error("Referendum failed");
          }
        });
      } while (!success);
      expect(success).to.be.true;

      if (!resultOwner) throw new AssertionError("Pool creation w/ governance: Result Owner was undefined!");
      // Verifications
      await verifyPoolCreationUsingQuery(
        api,
        stakingPoolId1,
        resultOwner,
        walletPoolOwner.publicKey,
        [api.createType("u128", POOL_11_REWARD_ASSET_ID)],
        maxRewards,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );*/
    });

    it("1.5  I can create a Pablo pool using sudo & an LP token pool will get automatically created.", async function () {
      this.timeout(2 * 60 * 1000);

      const wallet = walletPoolOwner;
      const baseAssetId = POOL_15_PBLO_BASE_ASSET_ID;
      const quoteAssetId = POOL_15_PBLO_QUOTE_ASSET_ID;
      const fee = 10000;
      const baseWeight = 500000;
      const pool = api.createType("PalletPabloPoolInitConfiguration", {
        ConstantProduct: {
          owner: api.createType("AccountId32", wallet.address),
          pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
            base: api.createType("u128", baseAssetId),
            quote: api.createType("u128", quoteAssetId)
          }),
          fee: api.createType("Permill", fee),
          baseWeight: api.createType("Permill", baseWeight)
        }
      });
      //const pabloPoolId = await createConsProdPool(api, sudoKey, wallet, baseAssetId, quoteAssetId, fee, baseWeight);
      const {
        data: [resultPoolId]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.pablo.PoolCreated.is,
        api.tx.sudo.sudo(api.tx.pablo.create(pool))
      );

      const pabloPoolInfo = await api.query.pablo.pools(resultPoolId);
      const lpTokenId = pabloPoolInfo.unwrap().asConstantProduct.lpToken;
      pool15PabloPoolId = resultPoolId;
      pool15LpTokenId = lpTokenId;
    });

    it("1.6  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero time locks.", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
      const assetId = api.createType("u128", POOL_16_BASE_ASSET_ID);
      const maxRewards = api.createType("u128", 100 * 10 ** 12);
      const rewardPeriodPerSecond = "10";
      const amount = (0.1 * 10 ** 12).toString();
      const durationPreset = {
        "0": "1000000000"
      };
      const unlockPenalty = "100000000";
      const shareAssetId = POOL_16_SHARE_ASSET_ID;
      const financialNftAssetId = 60000002;
      const minimumStakingAmount = 10 ** 12;
      // Creating pool config parameter
      const lock = api.createType("ComposableTraitsStakingLockLockConfig", {
        durationPresets: durationPreset,
        unlockPenalty: unlockPenalty
      });
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          endBlock: endBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "60001": {
              maxRewards: maxRewards,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
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
        [api.createType("u128", 10001)],
        maxRewards,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });

    it("1.7  I can, as sudo, create a new Staking Rewards pool for any arbitrary asset ID with zero penalty locks.", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const currentBlockNumber = await api.query.system.number();
      const startBlock = api.createType("u32", currentBlockNumber.addn(4));
      const endBlock = api.createType("u32", currentBlockNumber.addn(24));
      const assetId = api.createType("u128", POOL_17_BASE_ASSET_ID);
      const maxRewards = api.createType("u128", 100 * 10 ** 12);
      const rewardPeriodPerSecond = "10";
      const amount = (0.1 * 10 ** 12).toString();
      const durationPreset = {
        "1200": "1000000000"
      };
      const unlockPenalty = "0";
      const shareAssetId = POOL_17_SHARE_ASSET_ID;
      const financialNftAssetId = 70000002;
      const minimumStakingAmount = 10 ** 12;
      // Creating pool config parameter
      const lock = api.createType("ComposableTraitsStakingLockLockConfig", {
        durationPresets: durationPreset,
        unlockPenalty: unlockPenalty
      });
      const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
        RewardRateBasedIncentive: {
          owner: walletPoolOwner.publicKey,
          assetId: assetId,
          startBlock: startBlock,
          endBlock: endBlock,
          rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", {
            // The dict keys are the reward asset IDs!
            "70001": {
              maxRewards: maxRewards,
              rewardRate: {
                period: {
                  PerSecond: rewardPeriodPerSecond
                },
                amount: amount
              }
            }
          }),
          lock: {
            durationPresets: durationPreset,
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
        [api.createType("u128", 10001)],
        maxRewards,
        startBlock,
        endBlock,
        api.createType("u128", shareAssetId),
        api.createType("u128", financialNftAssetId),
        api.createType("u128", minimumStakingAmount)
      );
    });
  });

  describe("2. Adding rewards to pool pots.", function () {
    it("2.1  [SHORT] I can, as pool owner, add rewards to staking rewards pool pot #1.1.", async function () {
      this.timeout(2 * 60 * 1000);

      // Parameters
      const poolId = stakingPoolId1;
      const assetId = POOL_11_REWARD_ASSET_ID;
      const amount = 100 * 10 ** 12;
      const keepAlive = true;
      const walletBalanceBefore = await api.rpc.assets.balanceOf(assetId.toString(), walletPoolOwner.publicKey);

      // If the rewards pot is empty, and we query `rewardsPotIsEmpty`, we should get an empty object as result,
      // which is not to be confused with `None` type.
      const poolInfoBefore = <Option<any>>await api.query.stakingRewards.rewardsPotIsEmpty(poolId, assetId);
      expect(poolInfoBefore.isNone).to.be.false;

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
      expect(new BN(amount)).to.be.bignumber.equal(resultAmount);

      await verifyPoolPotAddition(api, poolId, assetId, amount, walletPoolOwner, walletBalanceBefore);
    });

    it("2.2  I can, as pool owner, add all reward assets to another staking rewards pool with multiple reward pots #1.2.", async function () {
      this.timeout(2 * 60 * 1000);

      // Parameters
      const poolId = stakingPoolId2;
      const assetIds = [POOL_12_REWARD_ASSET_ID_1, POOL_12_REWARD_ASSET_ID_2, POOL_12_REWARD_ASSET_ID_3];
      const amount = 100 * Math.pow(10, 12);
      const keepAlive = true;

      // Transaction
      const transactions: SubmittableExtrinsic<"promise", ISubmittableResult>[] = [];
      const walletBalanceFunctions: Promise<any>[] = [];
      assetIds.forEach(function (asset) {
        transactions.push(api.tx.stakingRewards.addToRewardsPot(poolId, asset, amount, keepAlive));
        walletBalanceFunctions.push(api.rpc.assets.balanceOf(asset.toString(), walletPoolOwner.publicKey));
      });
      const walletBalancesBefore = await Promise.all(walletBalanceFunctions);
      await waitForBlocks(api);
      const result = await sendWithBatchAndWaitForSuccess(
        api,
        walletPoolOwner,
        api.events.stakingRewards.RewardsPotIncreased.is,
        transactions,
        false
      );

      // Verification
      const walletBalancesAfter = await Promise.all(walletBalanceFunctions);

      // ToDo: Verification fails!
      // Bug reported: https://app.clickup.com/t/31mqxrg
      walletBalancesAfter.forEach(function (balance, i) {
        const expectedFunds = new BN(walletBalancesBefore[i].toString()).sub(new BN(amount));
        // Following assertion fails! Was reported as a bug.
        expect(new BN(balance.toString())).to.be.bignumber.equal(expectedFunds);
      });
    });

    it("2.3  I can, as pool owner, add rewards to multiple staking pools at once.", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const poolIds = [stakingPoolId3, stakingPoolId6, stakingPoolId7];
      const assetIds = [
        POOL_13_REWARD_ASSET_ID,
        //POOL_14_REWARD_ASSET_ID,
        POOL_16_REWARD_ASSET_ID,
        POOL_17_REWARD_ASSET_ID
      ];
      const amount = 100 * Math.pow(10, 12);
      const keepAlive = false;

      // Transaction
      const transactions: SubmittableExtrinsic<"promise", ISubmittableResult>[] = [];
      const walletBalanceFunctions: Promise<any>[] = [];
      assetIds.forEach(function (asset, i) {
        transactions.push(api.tx.stakingRewards.addToRewardsPot(poolIds[i], asset, amount, keepAlive));
        walletBalanceFunctions.push(api.rpc.assets.balanceOf(asset.toString(), walletPoolOwner.publicKey));
      });
      const walletBalancesBefore = await Promise.all(walletBalanceFunctions);
      await waitForBlocks(api);
      const result = await sendWithBatchAndWaitForSuccess(
        api,
        walletPoolOwner,
        api.events.stakingRewards.RewardsPotIncreased.is,
        [
          api.tx.stakingRewards.addToRewardsPot(poolIds[0], assetIds[0], amount, keepAlive),
          api.tx.stakingRewards.addToRewardsPot(poolIds[1], assetIds[1], amount, keepAlive),
          api.tx.stakingRewards.addToRewardsPot(poolIds[2], assetIds[2], amount, keepAlive)
          //api.tx.stakingRewards.addToRewardsPot(poolIds[3], assetIds[3], amount, keepAlive)
        ],
        false
      );

      // Verification

      // Querying balances
      // ToDo: Verification fails!
      // Bug reported: https://app.clickup.com/t/31mqxrg
      const walletBalancesAfter = await Promise.all(walletBalanceFunctions);
      walletBalancesAfter.forEach(function (balance, i) {
        const expectedFunds = new BN(walletBalancesBefore[i].toString()).sub(new BN(amount));
        expect(new BN(balance.toString())).to.be.bignumber.equal(expectedFunds);
      });
    });
  });

  describe("3. Staking in the pools", function () {
    it("3.1  [SHORT] I can stake in the newly created rewards pool #1.1", async function () {
      this.timeout(2 * 60 * 1000);
      // Parameters
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      const durationPreset = 600;
      const stakeAmount = (50 * Math.pow(10, 12)).toString();

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

    it("3.2  Another user can stake in the newly created rewards pool #1.1", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker2.publicKey);
      // Parameters
      const durationPreset = 1200;
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
        walletStaker2,
        api.events.stakingRewards.Staked.is,
        api.tx.stakingRewards.stake(stakingPoolId1, stakeAmount, durationPreset)
      );

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
        stakeAmount,
        stakingPoolId1,
        walletStaker2,
        userFundsBefore
      );
    });

    it("3.3 I can stake in the newly created rewards pool #1.2", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 600;
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

      await verifyPoolStaking(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeAmount,
        stakingPoolId2,
        walletStaker,
        userFundsBefore
      );
    });

    it("3.4  I can stake in the preconfigured PICA pool", async function () {
      this.timeout(2 * 60 * 1000);

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

    it("3.5  I can stake in the preconfigured PBLO pool", async function () {
      this.timeout(2 * 60 * 1000);

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

    it("3.6  I can stake in the created LP token pool #1.5.", async function () {
      this.timeout(2 * 60 * 1000);

      // ToDo: Buggy!
      // Ticket: https://app.clickup.com/t/32009xc

      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(pool15LpTokenId.toString(), walletStaker2.publicKey);

      const {
        data: [resultWho, resultPabloPoolId, resultBaseAmount, resultQuoteAmount, resultMintedLp]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker,
        api.events.pablo.LiquidityAdded.is,
        api.tx.pablo.addLiquidity(pool15PabloPoolId, 100_000_000_000, 100_000_000_000, 500_000, true)
      );

      const stakeAmount = resultMintedLp.div(new BN(4));
      const durationPreset = 604800;
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
        api.tx.stakingRewards.stake(pool15LpTokenId, stakeAmount, durationPreset)
      );

      // Verification
      // Verifying the poolId, reported by the event, is reported correctly.
      expect(resultPoolId).to.be.bignumber.equal(pool15LpTokenId);
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
      fNFTCollectionId6 = resultFNFTCollectionId;
      fNFTInstanceId6 = resultFNFTInstanceId;

      await verifyPoolStaking(
        api,
        fNFTCollectionId6,
        fNFTInstanceId6,
        stakeAmount.toString(),
        pool15LpTokenId,
        walletStaker,
        userFundsBefore
      );
    });

    it("3.7  I can stake in the newly created pool with 0 time locks. #1.6", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_16_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 0;
      const stakeAmount = (10 ** 12).toString();

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

    it("3.8  I can stake in the newly created pool with 0 unlock penalty. #1.7", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_17_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const durationPreset = 1200;
      const stakeAmount = (10 ** 12).toString();

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

  describe("4. Claiming from staked positions.", function () {
    it("4.1  [SHORT] I can claim from the arbitrary asset pool in #1.1 using the stake from #3.1, during the lock period.", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_11_REWARD_ASSET_ID.toString(),
        walletStaker.publicKey
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId1;
      const fNFTInstanceId = fNFTInstanceId1;
      const stakingPoolId = stakingPoolId1;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);
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
        [POOL_11_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        api.createType("u128", claimableAmount)
      );
    });

    it("4.2  I can claim from the arbitrary asset pool in #1.1 using the stake from #3.2, during the lock period.", async function () {
      this.timeout(2 * 60 * 1000);
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
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_11_SHARE_ASSET_ID);
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
        data: [resultStakeOwner, resultFNFTCollectionId, resultFNFTInstanceId]
      } = await sendAndWaitForSuccess(
        api,
        walletStaker2,
        api.events.stakingRewards.Claimed.is,
        api.tx.stakingRewards.claim(fNFTCollectionId2, fNFTInstanceId2)
      );

      // Verification
      expect(resultStakeOwner.toString()).to.be.equal(
        api.createType("AccountId32", walletStaker2.publicKey).toString()
      );
      expect(resultFNFTCollectionId.toString()).to.equal(fNFTCollectionId.toString());
      expect(resultFNFTInstanceId.toString()).to.equal(fNFTInstanceId.toString());

      await verifyPoolClaiming(
        api,
        fNFTCollectionId2,
        fNFTInstanceId2,
        [POOL_11_REWARD_ASSET_ID],
        walletStaker2,
        [userFundsBefore],
        api.createType("u128", claimableAmount)
      );
    });

    it("4.3  I can claim from the arbitrary asset pool in #1.2 using the stake from #3.3 after the lock period has ended.", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore1 = await api.rpc.assets.balanceOf(
        POOL_12_REWARD_ASSET_ID_1.toString(),
        walletStaker2.publicKey
      );
      const userFundsBefore2 = await api.rpc.assets.balanceOf(
        POOL_12_REWARD_ASSET_ID_2.toString(),
        walletStaker2.publicKey
      );
      const userFundsBefore3 = await api.rpc.assets.balanceOf(
        POOL_12_REWARD_ASSET_ID_3.toString(),
        walletStaker2.publicKey
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId3;
      const fNFTInstanceId = fNFTInstanceId3;
      const stakingPoolId = stakingPoolId3;

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
        [POOL_12_REWARD_ASSET_ID_1, POOL_12_REWARD_ASSET_ID_2, POOL_12_REWARD_ASSET_ID_3],
        walletStaker,
        [userFundsBefore1, userFundsBefore2, userFundsBefore3],
        api.createType("u128", claimableAmount)
      );
    });

    it("4.4  I can claim from the PICA pool using my stake in #3.4, after the lock period has ended.", async function () {
      this.timeout(2 * 60 * 1000);

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

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(1001);
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
        api.tx.stakingRewards.claim(fNFTCollectionId, fNFTInstanceId)
      );

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

    it("4.5  I can claim from the PBLO pool using my stake in #3.5, after the lock period has ended.", async function () {
      this.timeout(2 * 60 * 1000);

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

    it("4.6  I can claim from the LP token pool using my stake in #3.6, after the lock period has ended.", async function () {
      this.timeout(2 * 60 * 1000);

      // LP staking does not seem to work!
      // Please see test 3.6 for further information!
      // ToDo: Trade to have claimable funds!

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

    it("4.7  I can claim from the 0 time lock pool using my stake in #3.7", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_16_REWARD_ASSET_ID.toString(),
        walletStaker2.publicKey
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId7;
      const fNFTInstanceId = fNFTInstanceId7;
      const stakingPoolId = stakingPoolId6;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_16_SHARE_ASSET_ID);
      // Calculating claimable amount.
      const claimableAmount = getClaimOfStake(
        api,
        stakeInfoBefore.unwrap(),
        poolInfo.unwrap(),
        POOL_16_REWARD_ASSET_ID.toString(),
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
        [POOL_16_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        api.createType("u128", claimableAmount)
      );
    });

    it("4.8  I can claim from the 0 unlock penalty pool using my stake in #3.8.", async function () {
      this.timeout(2 * 60 * 1000);
      // Get funds before transaction
      const userFundsBefore = await api.rpc.assets.balanceOf(
        POOL_17_REWARD_ASSET_ID.toString(),
        walletStaker2.publicKey
      );

      // Setting Parameters
      const fNFTCollectionId = fNFTCollectionId8;
      const fNFTInstanceId = fNFTInstanceId8;
      const stakingPoolId = stakingPoolId7;

      // Getting stake info before transaction, to calculate claimable amount.
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
      );
      // Getting pool info before transaction, to calculate claimable amount.
      const poolInfo = <Option<ComposableTraitsStakingRewardPool>>(
        await api.query.stakingRewards.rewardPools(stakingPoolId)
      );
      // Getting total issuance of the defined share asset, used for claimable amount calculation.
      const totalShareAssetIssuance = await api.query.tokens.totalIssuance(POOL_17_SHARE_ASSET_ID);
      // Calculating claimable amount.
      const claimableAmount = getClaimOfStake(
        api,
        stakeInfoBefore.unwrap(),
        poolInfo.unwrap(),
        POOL_17_REWARD_ASSET_ID.toString(),
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
        [POOL_17_REWARD_ASSET_ID],
        walletStaker,
        [userFundsBefore],
        api.createType("u128", claimableAmount)
      );
    });
  });

  describe("5. Extending existing positions.", function () {
    it("5.1  [SHORT] I can extend the staked amount in pool #1.1 using the stake from #3.3", async function () {
      this.timeout(2 * 60 * 1000);
      // Querying stake
      const stakeInfoBefore = <Option<ComposableTraitsStakingStake>>(
        await api.query.stakingRewards.stakes(fNFTCollectionId3, fNFTInstanceId3)
      );
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const amount = 2 * 10 ** 12;

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
      // Verifying the fNFTCollectionId & instance ID are correct.
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

    it("5.2  I can extend the lock time in pool #1.1 using the stake from #3.1", async function () {
      this.timeout(2 * 60 * 1000);
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
      // Verifying the fNFTCollectionId & instance ID are correct.
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

  describe("6. Splitting existing positions.", function () {
    it("6.1  [SHORT] I can split my staking position into 2 separate positions", async function () {
      this.timeout(2 * 60 * 1000);
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

    it("6.2  I can split my already split position again", async function () {
      this.timeout(2 * 60 * 1000);

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
      expect(resultPositions.length).to.be.equal(3);
      await verifyPositionSplitting(
        api,
        fNFTCollectionId3,
        fNFTInstanceId3,
        stakeInfoBefore,
        0.3,
        0.7,
        resultPositions[2][0],
        resultPositions[2][1]
      );
      allSplitPositions = resultPositions;
    });
  });

  describe("7. Unstaking positions.", function () {
    it("7.1  I can unstake my staking position before my lock period has ended and get slashed.", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting user funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_12_BASE_ASSET_ID.toString(), walletStaker2.publicKey);
      const stakeAmount = (10 ** 12).toString();
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
      expect(resultSlash.unwrap()).to.be.bignumber.greaterThan(new BN(0));

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        POOL_11_BASE_ASSET_ID,
        walletStaker2,
        userFundsBefore,
        stakeAmount,
        true,
        resultSlash.unwrap()
      );
    });

    it("7.2  [SHORT] I can unstake my staking position after the locking period has ended without getting slashed.", async function () {
      this.timeout(4 * 60 * 1000);

      // ToDo: Bugged!
      // Ticket: https://app.clickup.com/t/3200gvc

      // Waiting a few blocks to safely unstake funds.
      await waitForBlocks(api, 3);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_11_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId1;
      const fNFTInstanceId = fNFTInstanceId1;
      const stakeAmount = (10 ** 12).toString();

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
      expect(resultSlash.isNone).to.be.true;

      await verifyPositionUnstaking(
        api,
        fNFTCollectionId,
        fNFTInstanceId,
        POOL_11_BASE_ASSET_ID,
        walletStaker,
        userFundsBefore,
        stakeAmount
      );
    });

    it("7.3  I can unstake my staking position from PICA pool after the locking period has ended without getting slashed.", async function () {
      this.timeout(4 * 60 * 1000);

      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("1", walletStaker.publicKey);
      // Parameters
      /*const poolInfo = await api.query.stakingRewards.rewardPools(1);
      const shareAssetId = poolInfo.unwrap()["shareAssetId"];
      const financialNftAssetId = poolInfo.unwrap()["financialNftAssetId"];*/
      const fNFTCollectionId = fNFTCollectionId4Pica;
      const fNFTInstanceId = fNFTInstanceId4Pica;
      const stakeAmount = (10 ** 12).toString();

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

    it("7.4  I can unstake my staking position from PBLO pool after the locking period has ended without getting slashed.", async function () {
      this.timeout(4 * 60 * 1000);

      // ToDo: Fix when preconfigured pools have their rewards configuration!
      throw new Error("Pre- configured pools don't have any reward configuration yet!");

      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker.publicKey);
      // Parameters
      /*const poolInfo = await api.query.stakingRewards.rewardPools(5);
      const shareAssetId = poolInfo.unwrap()["shareAssetId"];
      const financialNftAssetId = poolInfo.unwrap()["financialNftAssetId"];*/
      const fNFTCollectionId = fNFTCollectionId5Pblo;
      const fNFTInstanceId = fNFTInstanceId5Pblo;
      const stakeAmount = (10 ** 12).toString();

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

    it("7.5  I can unstake my staking position from the LP token pool after the locking period has ended without getting slashed.", async function () {
      this.timeout(4 * 60 * 1000);

      // ToDo: LP Token staking does not seem to work!
      // Check 3.6 for further information.

      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf("5", walletStaker.publicKey); // ToDo: LP Token Pool Reward Asset ID.
      // Parameters
      const fNFTCollectionId = fNFTCollectionId6;
      const fNFTInstanceId = fNFTInstanceId6;
      const stakeAmount = (10 ** 12).toString();

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

    it("7.6  I can unstake my staking position from the 0 time lock pool without getting slashed.", async function () {
      this.timeout(4 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_16_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId7;
      const fNFTInstanceId = fNFTInstanceId7;
      const stakeAmount = (10 ** 12).toString();

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

    it("7.7  I can unstake my position from the 0 unlock penalty pool without getting slashed.", async function () {
      this.timeout(4 * 60 * 1000);
      // Getting funds before
      const userFundsBefore = await api.rpc.assets.balanceOf(POOL_17_BASE_ASSET_ID.toString(), walletStaker.publicKey);
      // Parameters
      const fNFTCollectionId = fNFTCollectionId8;
      const fNFTInstanceId = fNFTInstanceId8;
      const stakeAmount = (10 ** 12).toString();

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

    it("7.8  I can unstake all split positions.", async function () {
      this.timeout(4 * 60 * 1000);
      for (const position of allSplitPositions) {
        // Getting funds before
        const userFundsBefore = await api.rpc.assets.balanceOf(
          POOL_11_BASE_ASSET_ID.toString(),
          walletStaker.publicKey
        );
        // Parameters
        const fNFTCollectionId = position[0];
        const fNFTInstanceId = position[1];
        const stakeAmount = (10 ** 12).toString();

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
        expect(resultOwner.toString()).to.be.equal(api.createType("AccountId32", walletStaker2.publicKey).toString());
        expect(resultFNFTCollectionId).to.be.bignumber.equal(api.createType("u128", fNFTCollectionId));
        expect(resultFNFTInstanceId).to.be.bignumber.equal(api.createType("u64", fNFTInstanceId));

        await verifyPositionUnstaking(
          api,
          api.createType("u128", fNFTCollectionId),
          api.createType("u64", fNFTInstanceId),
          POOL_11_BASE_ASSET_ID,
          walletStaker,
          userFundsBefore,
          stakeAmount
        );
      }
    });
  });
});
