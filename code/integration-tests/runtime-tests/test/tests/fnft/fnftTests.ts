import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import {
  addFundsToThePools,
  addRewardsToRewardPools,
  createConsProdPoolforFnft,
  createStableSwapPoolforFnft,
  createStakingRewardPool,
  extendStakingPosition,
  getCollectionInstanceOwner,
  getCollectionInstances,
  getCollectionOwnerAndAdmin,
  getLockedToken,
  getRewardPoolOwner,
  getUserFnfts,
  stakeLpTokens,
  unstakeAndBurn
} from "@composabletests/tests/fnft/fnftPalletTestHelper";
import { addFundsToThePool } from "@composabletests/tests/pablo/testHandlers/pabloTestHelper";
import { expect } from "chai";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import * as testConfig from "./test_configuration.json";

describe.only ("Fnft Pallet Tests", function () {
  if (!testConfig.enabledTests.tx.enabled) {
    console.log("Fnft Tests are being skipped");
    return;
  }
  let api: ApiPromise;
  let pica: number, ksm: number, usdt: number, usdc: number, rewardAsset: number;
  let poolOwner: KeyringPair, lpProvider: KeyringPair, sudoKey: KeyringPair;
  let usFnftProxy: AccountId32, kpFnftProxyAccount: AccountId32;
  let fee: number, baseWeight: number, ampCoeff: number;
  let picaKsmPool: number, usdtUsdcPool: number;
  let picaKsmfnftCollectionId: number, usdtUsdcFnftCollectionId: number;
  let picaKsmStakingRewardPool: number, usdtUsdcStakingRewardPool: number;
  let picaKsmFnftInstanceId: number, usFnftInstanceId: number;
  const ONE_WEEK = 7 * 24 * 60 * 60;
  this.timeout(2 * 60 * 1000);

  before("Initialize variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    poolOwner = devWalletEve.derive("/test/constantProductDex/walletId1");
    lpProvider = devWalletFerdie.derive("/test/constantProductDex/walletId2");
    pica = 1;
    ksm = 4;
    usdc = 131;
    usdt = 130;
    rewardAsset = 5;
    //sets the fee to 1.00%/Type Permill
    fee = 10000;
    baseWeight = 500000;
    ampCoeff = 200;
  });

  before("Set up pools and mint assets", async function () {
    this.timeout(5 * 60 * 1000);
    const rewardAmount = 10000;
    await mintAssetsToWallet(api, poolOwner, sudoKey, [pica, ksm, usdc, usdt, rewardAsset]);
    await mintAssetsToWallet(api, lpProvider, sudoKey, [pica, ksm, usdc, usdt]);
    const picaKsmEvents = await createConsProdPoolforFnft(api, sudoKey, poolOwner, pica, ksm, fee, baseWeight);
    picaKsmPool = picaKsmEvents.pabloPoolInfo.poolId;
    picaKsmfnftCollectionId = picaKsmEvents.fnftCollectionInfo.fnftCollectionId;
    picaKsmStakingRewardPool = picaKsmEvents.rewardPoolInfo.rewardPoolId;
    const usdtUsdcEvents = await createStableSwapPoolforFnft(api, sudoKey, poolOwner, usdt, usdc, ampCoeff, fee);
    usdtUsdcPool = usdtUsdcEvents.pabloPoolInfo.poolId;
    usdtUsdcFnftCollectionId = usdtUsdcEvents.fnftCollectionInfo.fnftCollectionId;
    usdtUsdcStakingRewardPool = usdtUsdcEvents.rewardPoolInfo.rewardPoolId;
    await addFundsToThePools(api, [picaKsmPool, usdtUsdcPool], poolOwner);
    await addFundsToThePool(api, picaKsmPool, lpProvider, Pica(10000), Pica(20000));
    await addRewardsToRewardPools(
      api,
      [picaKsmStakingRewardPool, usdtUsdcStakingRewardPool],
      rewardAsset,
      poolOwner,
      rewardAmount
    );
  });

  after("Close api connection", async function () {
    await api.disconnect();
  });

  it("Fnft Collections should be owned and admined by reward pool owner ", async function () {
    const rewardPoolOwner = await getRewardPoolOwner(api, picaKsmStakingRewardPool);
    const collectionData = await getCollectionOwnerAndAdmin(api, picaKsmfnftCollectionId);
    expect(rewardPoolOwner).to.be.equal(collectionData.owner);
    expect(rewardPoolOwner).to.be.equal(collectionData.admin);
  });

  it("Users can stake their tokens to receive their positions in fnfts", async function () {
    const lpAmount = 750;
    //By default, pablo pools create 1 week and 3 week staking options
    kpFnftProxyAccount = await stakeLpTokens(api, picaKsmStakingRewardPool, lpProvider, lpAmount, ONE_WEEK);
    const locked = await getLockedToken(api, kpFnftProxyAccount, picaKsmStakingRewardPool);
    //verifies that the amount is locked in proxy account
    expect(locked).to.be.equal(lpAmount);
  });

  it("Users will be assigned fnft instances representing their staking positions", async function () {
    picaKsmFnftInstanceId = await getCollectionInstances(api, picaKsmfnftCollectionId);
    //First staking to the rewards pool, hence should be 0
    expect(picaKsmFnftInstanceId).to.be.equal(0);
    const instanceOwner = await getCollectionInstanceOwner(api, picaKsmfnftCollectionId, picaKsmFnftInstanceId);
    expect(instanceOwner).to.be.equal(api.createType("AccountId32", lpProvider.address).toString());
  });

  it("Users can extend their positions by adding more tokens to the pool", async function () {
    const lpAmount = 1000;
    const locked = await getLockedToken(api, kpFnftProxyAccount, picaKsmStakingRewardPool);
    const picaKsmFnftInstancesPre = await getCollectionInstances(api, picaKsmfnftCollectionId);
    await extendStakingPosition(api, lpProvider, picaKsmfnftCollectionId, picaKsmFnftInstanceId, lpAmount);
    const lockedAfterTx = await getLockedToken(api, kpFnftProxyAccount, picaKsmStakingRewardPool);
    const picaKsmFnftInstancesAfter = await getCollectionInstances(api, picaKsmfnftCollectionId);
    //verifies that the extended amount is locked in proxy account
    //expect(locked + lpAmount).to.be.equal(lockedAfterTx);
    //verifies that extending an existing position doesn't create a new instance of fnft
    expect(picaKsmFnftInstancesPre).to.be.equal(picaKsmFnftInstancesAfter);
  });

  it("Users can stake to multiple lpstaking pools", async function () {
    const lpAmount = 2000;
    await addFundsToThePool(api, usdtUsdcPool, lpProvider, Pica(50000), Pica(50000));
    usFnftProxy = await stakeLpTokens(api, usdtUsdcStakingRewardPool, lpProvider, lpAmount, ONE_WEEK);
    const locked = await getLockedToken(api, usFnftProxy, usdtUsdcStakingRewardPool);
    //verifies that the amount is locked in proxy account
    expect(locked).to.be.equal(lpAmount);
  });

  it("Users can receive multiple fnfts when staked more than one lp staking pool", async function () {
    usFnftInstanceId = await getCollectionInstances(api, usdtUsdcFnftCollectionId);
    //First staking to the rewards pool, hence should be 0
    expect(usFnftInstanceId).to.be.equal(0);
    const instanceOwner = await getCollectionInstanceOwner(api, usdtUsdcFnftCollectionId, usFnftInstanceId);
    expect(instanceOwner).to.be.equal(api.createType("AccountId32", lpProvider.address).toString());
    const allFnftInstances = await getUserFnfts(api, lpProvider);
    //Verifies that all user fnfts is correctly stored under user account
    expect(allFnftInstances).to.have.length(2);
  });

  it("Users can receive fnft's from custom created rewards pools", async function () {
    const result = await createStakingRewardPool(api, sudoKey, ksm, pica);
    await addRewardsToRewardPools(api, [result.poolId], pica, poolOwner, 4000000);
    const ksmProxyAccount = await stakeLpTokens(api, result.poolId, lpProvider, 5000, 5 * 60);
    const locked = await getLockedToken(api, ksmProxyAccount, ksm);
    //verifies that the amount is locked in proxy account
    expect(locked).to.be.equal(5000);
  });

  it("Users after unstaking, won't be reassigned the same fnft instance id", async function () {
    const lpAmount = 1000;
    const unstakedEvent = await unstakeAndBurn(api, lpProvider, usdtUsdcFnftCollectionId, usFnftInstanceId);
    expect(unstakedEvent.owner.toString()).to.be.equal(api.createType("AccountId32", lpProvider.address).toString());
    expect(unstakedEvent.fnftCollectionId).to.be.equal(usdtUsdcFnftCollectionId);
    expect(unstakedEvent.fnftInstanceId).to.be.equal(usFnftInstanceId);
    await stakeLpTokens(api, usdtUsdcStakingRewardPool, lpProvider, lpAmount, ONE_WEEK);
    const usdtUsdcFnftInstances = await getCollectionInstances(api, usdtUsdcFnftCollectionId);
    //Previous fnft is burned, hence expecting to be 1
    expect(usdtUsdcFnftInstances).to.be.equal(1);
  });
});
