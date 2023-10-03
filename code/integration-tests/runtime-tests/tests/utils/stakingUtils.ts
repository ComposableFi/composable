import {ApiPromise, Keyring} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import BigNumber from "bignumber.js";
import {sendAndWaitForSuccess} from "./txClient";
import {stakingPalletAddress} from "./constants";
import {waitForSeconds} from "./ibcUtils";


export async function seedStakingPallet(api: ApiPromise, sudoKey: KeyringPair, tokenId: number, transferAmount: BigNumber) {
  await sendTokens(api, sudoKey, stakingPalletAddress, tokenId, false, transferAmount);
}

export async function sendTokens(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWalletAddress: string,
  tokenId: number,
  expectedToFail: boolean,
  transferAmount: BigNumber
) {
  const asset = api.createType('u128', tokenId);
  const dest = api.createType('MultiAddress', {
    id: api.createType('AccountId', receiverWalletAddress)
  });
  const amount = api.createType('Compact<u128>', transferAmount.toString());
  const keepAlive = api.createType('bool', 'No');
  let event;
  if (expectedToFail) {
    event = api.events.system.ExtrinsicFailed.is;
  } else {
    event = api.events.system.ExtrinsicSuccess.is;
  }
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    // @ts-ignore
    event,
    api.tx.assets.transfer(asset, dest, amount, keepAlive),
    expectedToFail
  )
}

export async function createStakingPool(
  api: ApiPromise,
  sudoKey: KeyringPair,
  stakingTokenId: number,
  rewardAmount: BigNumber,
  rewardTokenId: number,
  period: number
) {
  const poolCurrencyId = api.createType('u128', stakingTokenId);
  const rewardCurrencyId = api.createType('u128', rewardTokenId);
  const periodCount = api.createType('u32', period);
  const amount = api.createType('u128', rewardAmount.toString());
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.farming.RewardScheduleUpdated.is,
    api.tx.sudo.sudo(api.tx.farming.updateRewardSchedule(poolCurrencyId, rewardCurrencyId, periodCount, amount)),
    false
  );
}

export async function stakeToPool(api: ApiPromise, stakerWallet: KeyringPair, poolId: number, stakeAmount: BigNumber) {
  const poolCurrencyId = api.createType('u128', poolId);
  const amount = api.createType('u128', stakeAmount.toString());
  return await sendAndWaitForSuccess(
    api,
    stakerWallet,
    api.events.farmingRewards.DepositStake.is,
    api.tx.farming.deposit(poolCurrencyId, amount),
    false
  )
}

export async function waitFor5Blocks(api: ApiPromise) {
  let currentBlock = (await api.query.system.number()).toNumber();
  while (currentBlock % 5 !== 0) {
    await waitForSeconds(12);
    currentBlock = (await api.query.system.number()).toNumber();
  }
}

export async function getPoolRewardsPerPeriod(api: ApiPromise, poolId: number, rewardTokenId: number) {
  const pool = api.createType('u128', poolId);
  const rewardToken = api.createType('u128', rewardTokenId);
  const {perPeriod} = await api.query.farming.rewardSchedules(pool, rewardToken);
  return new BigNumber(perPeriod.toString());
}

export async function getTotalStakes(api: ApiPromise, poolId: number) {
  const pool = api.createType('u128', poolId);
  const totalStaked = await api.query.farmingRewards.totalStake(pool);
  return new BigNumber(totalStaked.toString()).dividedBy(new BigNumber(10 ** 18));
}

export async function getUserStakes(api: ApiPromise, stakerWallet: KeyringPair, poolId: number) {
  const totalStake = await api.query.farmingRewards.stake([poolId, stakerWallet.address]);
  return new BigNumber(totalStake.toString()).dividedBy(new BigNumber(10 ** 18));
}

export function calculateUserShare(totalStake: BigNumber, userStake: BigNumber) {
  return totalStake.dividedBy(userStake);
}

export function calculateExpectedReward(perPeriodRewards: BigNumber, userShare: BigNumber) {
  return ((perPeriodRewards.dividedBy(userShare)).dividedBy(10 ** 12)).toNumber();
}

export async function claimUserRewards(api: ApiPromise, stakerWallet: KeyringPair, poolId: number, rewardTokenId: number) {
  const poolCurrencyId = api.createType('u128', poolId);
  const rewardCurrencyId = api.createType('u128', rewardTokenId);
  return await sendAndWaitForSuccess(
    api,
    stakerWallet,
    api.events.farming.RewardClaimed.is,
    api.tx.farming.claim(poolCurrencyId, rewardCurrencyId),
    false
  )
}

export async function queryReservedBalance(api: ApiPromise, walletAddress: string, tokenId: number) {
  const {reserved} = await api.query.tokens.accounts(walletAddress, tokenId);
  return reserved.toString();
}

export function createMultipleUsers(nbOfUsers: number) {
  const keyring = new Keyring({type: 'sr25519'});
  const users = [];
  for (let i = 0; i < nbOfUsers; i++) {
    users.push(keyring.createFromUri(`//Alice/${i}`));
  }
  return users;
}

export async function stakeToPoolByMultipleUsers(api: ApiPromise, users: KeyringPair[], poolId: number) {
  await Promise.all(users.map(async user => {
    const randomMultiplier = Math.floor(Math.random() * 100);
    const stakeAmount = new BigNumber(randomMultiplier).multipliedBy(10 ** 12);
    await stakeToPool(api, user, poolId, stakeAmount);
  }));
}

export function getDiffInNumber(afterBalance: string, preBalance: string) {
  const afterBN = new BigNumber(afterBalance);
  const preBN = new BigNumber(preBalance);
  return ((afterBN.minus(preBN)).dividedBy(10 ** 12)).toNumber();
}

export async function unstakeFromPool(api: ApiPromise, stakerWallet: KeyringPair, poolId: number, unstakeAmount: BigNumber) {
  const poolCurrencyId = api.createType('u128', poolId);
  const amount = api.createType('u128', unstakeAmount.toString());
  await sendAndWaitForSuccess(
    api,
    stakerWallet,
    api.events.farmingRewards.WithdrawStake.is,
    api.tx.farming.withdraw(poolCurrencyId, amount),
    false
  )
}