import {ApiPromise} from "@polkadot/api";
import {picassoEndpoint} from "./utils/constants";
import {getWallets, initializeApi} from "./utils/apiClient";
import {
  calculateExpectedReward,
  calculateUserShare,
  claimUserRewards, createMultipleUsers,
  createStakingPool, getDiffInNumber, getPoolRewardsPerPeriod, getTotalStakes,
  getUserStakes, queryReservedBalance,
  seedStakingPallet, sendTokens,
  stakeToPool, stakeToPoolByMultipleUsers, unstakeFromPool,
  waitFor5Blocks
} from "./utils/stakingUtils";
import {KeyringPair} from "@polkadot/keyring/types";
import {mintAssetsToWallets} from "./utils/mintingHelper";
import BigNumber from "bignumber.js";
import {queryTokenBalance} from "./utils/ibcUtils";
const expect = require('chai').expect;


describe.skip('Staking Tests', function () {
  this.timeout(3 * 60 * 1000);
  let picassoApi: ApiPromise;
  let testWallet: KeyringPair;
  let sudoKey: KeyringPair;
  const seedAmount = new BigNumber('10000000000000000');
  const rewardAmount = new BigNumber('5000000000000000');
  const period = 1000;
  const poolId = 4;
  const rewardTokenId = 4;
  let stakeAmount = new BigNumber('1000000000000000');
  let stakingUsers: KeyringPair[];

  before('Initializes apis', async () =>{
    picassoApi = await initializeApi(picassoEndpoint);
    ({sudoKey, testWallet} = getWallets('Staking'))
    stakingUsers = createMultipleUsers(50);
    await mintAssetsToWallets(
      picassoApi,
      [...stakingUsers, testWallet, sudoKey],
      sudoKey,
      [1, 4, 6, 130],
      '100000000000000000',
      'picasso'
    );
    await seedStakingPallet(picassoApi, sudoKey, rewardTokenId, seedAmount)
  });

  before('Creates staking pools', async () =>{
    await createStakingPool(picassoApi, sudoKey, poolId, rewardAmount, rewardTokenId, period);
  });

  it('Users can stake into the staking pool', async () =>{
    await stakeToPoolByMultipleUsers(picassoApi, stakingUsers, poolId);
    await stakeToPool(picassoApi, testWallet, poolId, stakeAmount);
    await waitFor5Blocks(picassoApi);
    const userStake = await getUserStakes(picassoApi, testWallet, poolId);
    const reservedBalance = await queryReservedBalance(picassoApi, testWallet.address, rewardTokenId);
    expect(userStake.toString()).to.be.eq(stakeAmount.toString());
    expect(reservedBalance).to.be.eq(stakeAmount.toString());
  });

  it('Users can claim rewards once rewards are accumulated', async () =>{
    const totalRewardPerPeriod = await getPoolRewardsPerPeriod(picassoApi, poolId, rewardTokenId);
    const totalStake = await getTotalStakes(picassoApi, poolId);
    const userShare = calculateUserShare(totalStake, stakeAmount);
    const userPreBalance = await queryTokenBalance(picassoApi, testWallet.address, rewardTokenId.toString());
    await claimUserRewards(picassoApi, testWallet, poolId, rewardTokenId);
    const userAfterBalance = await queryTokenBalance(picassoApi, testWallet.address, rewardTokenId.toString());
    const expectedDifference = calculateExpectedReward(totalRewardPerPeriod, userShare);
    const balanceDiff = getDiffInNumber(userAfterBalance, userPreBalance);
    expect(balanceDiff).to.be.within(expectedDifference - 0.01, expectedDifference + 0.01);
  });

  it('Users cant claim rewards before already claimed period ends', async () =>{
    const userPreBalance = await queryTokenBalance(picassoApi, testWallet.address, rewardTokenId.toString());
    await claimUserRewards(picassoApi, testWallet, poolId, rewardTokenId);
    const userAfterBalance = await queryTokenBalance(picassoApi, testWallet.address, rewardTokenId.toString());
    expect(userAfterBalance).to.be.eq(userPreBalance);
  });

  it('Users cant spend the staked amount', async () =>{
    const userPreBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    const transferAmount = new BigNumber(userPreBalance).plus(5000);
    const result =
      await sendTokens(picassoApi, testWallet, sudoKey.address, poolId, true, transferAmount);
    expect(result.data[0].toString()).to.be.eq('{"token":"FundsUnavailable"}');
  });

  it('Users can add more stakes into their stakes', async () =>{
    const additionAmount = stakeAmount.dividedBy(2);
    stakeAmount = stakeAmount.plus(additionAmount);
    await stakeToPool(picassoApi, testWallet, poolId, additionAmount);
    const userStake = await getUserStakes(picassoApi, testWallet, poolId);
    const reservedBalance = await queryReservedBalance(picassoApi, testWallet.address, rewardTokenId);
    expect(userStake.toString()).to.be.eq(stakeAmount.toString());
    expect(reservedBalance).to.be.eq(stakeAmount.toString());
  });

  it('Users can partially withdraw funds from staking accounts', async () =>{
    const removalAmount = stakeAmount.dividedBy(2);
    stakeAmount = removalAmount;
    const preBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    await unstakeFromPool(picassoApi, testWallet, poolId, removalAmount);
    const afterBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    expect((new BigNumber(afterBalance).minus(new BigNumber(preBalance))).toString()).to.be.eq(removalAmount.toString());
  });

  it('After partial withdrawal users can still claim rewards based on their shares', async () =>{
    const totalRewardPerPeriod = await getPoolRewardsPerPeriod(picassoApi, poolId, rewardTokenId);
    const totalStake = await getTotalStakes(picassoApi, poolId);
    const userShare = calculateUserShare(totalStake, stakeAmount);
    const userPreBalance = await queryTokenBalance(picassoApi, testWallet.address, rewardTokenId.toString());
    await claimUserRewards(picassoApi, testWallet, poolId, rewardTokenId);
    const userAfterBalance = await queryTokenBalance(picassoApi, testWallet.address, rewardTokenId.toString());
    const expectedDifference = calculateExpectedReward(totalRewardPerPeriod, userShare);
    const balanceDiff = getDiffInNumber(userAfterBalance, userPreBalance);
    expect(balanceDiff).to.be.above(expectedDifference);
  });

  it('Users can totally withdraw their funds on staking accounts', async () =>{
    const preBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    await unstakeFromPool(picassoApi, testWallet, poolId, stakeAmount);
    await claimUserRewards(picassoApi,testWallet, poolId, rewardTokenId);
    const afterBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    const userShare = await getUserStakes(picassoApi, testWallet, poolId);
    expect((new BigNumber(afterBalance).minus(new BigNumber(preBalance))).toString()).to.be.eq(stakeAmount.toString());
    expect(userShare.toString()).to.be.eq('0');
    await waitFor5Blocks(picassoApi);
  });

  it('After complete withdrawal users cant claim rewards', async () =>{
    const preBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    await claimUserRewards(picassoApi, testWallet, poolId, rewardTokenId);
    const afterBalance = await queryTokenBalance(picassoApi, testWallet.address, poolId.toString());
    expect((new BigNumber(afterBalance).minus(new BigNumber(preBalance))).toString()).to.be.eq('0');
  });

  after('Disconnects api', async () =>{
    await picassoApi.disconnect();
  })
})