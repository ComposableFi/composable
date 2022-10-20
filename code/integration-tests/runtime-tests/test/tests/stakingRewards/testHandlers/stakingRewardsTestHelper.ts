import { ApiPromise } from "@polkadot/api";
import { Option, u128, u32, u64 } from "@polkadot/types-codec";
import {
  ComposableTraitsStakingRewardPool,
  ComposableTraitsStakingStake,
  CustomRpcBalance
} from "@composable/types/interfaces";
import { expect } from "chai";
import { AccountId32 } from "@polkadot/types/interfaces";
import BN from "bn.js";
import { KeyringPair } from "@polkadot/keyring/types";
import { AnyNumber } from "@polkadot/types/types";

export async function verifyPoolCreationUsingQuery(
  api: ApiPromise,
  stakingPoolId: u128,
  resultOwner: AccountId32,
  walletPoolOwner: Uint8Array,
  rewardAssetIDs: u128[],
  maxRewards: u128,
  startBlock: u32,
  endBlock: u32,
  shareAssetId: u128,
  financialNftAssetId: u128,
  minimumStakingAmount: u128
) {
  // Now we're querying the pool info to verify details.
  const poolInfo = <Option<ComposableTraitsStakingRewardPool>>await api.query.stakingRewards.rewardPools(stakingPoolId);
  // Verifying pool owner is what we set it to, according to the query & the event result.
  expect(poolInfo.unwrap().owner.toString())
    .to.be.equal(resultOwner.toString())
    .to.be.equal(api.createType("AccountId32", walletPoolOwner).toString());
  // Verifying our pools rewards configuration is what we set it to, according to the query.
  poolInfo.unwrap().rewards.forEach(function (reward) {
    expect(reward.totalRewards).to.be.bignumber.equal(new BN(0));
    expect(reward.claimedRewards).to.be.bignumber.equal(new BN(0));
    expect(reward.totalDilutionAdjustment).to.be.bignumber.equal(new BN(0));
    expect(reward.maxRewards).to.be.bignumber.equal(maxRewards);
  });
  // Verifying the amount of claimed shares, according to the query, is 0.
  expect(poolInfo.unwrap().claimedShares).to.be.bignumber.equal(new BN(0));
  // Verifying the startBlock & endBlock, as reported by the query, is equal to what we set it to.
  expect(poolInfo.unwrap().startBlock).to.be.bignumber.equal(startBlock);
  expect(poolInfo.unwrap().endBlock).to.be.bignumber.equal(endBlock);
  // Verifying our shareAssetId, as reported by the query, is what we set it to.
  expect(poolInfo.unwrap().shareAssetId).to.be.bignumber.equal(shareAssetId);
  // Verifying our financialNftAssetId, as reported by the query, is what we set it to.
  expect(poolInfo.unwrap().financialNftAssetId).to.be.bignumber.equal(financialNftAssetId);
  // Verifying minimumStakingAmount, as reported by the query, is what we set it to.
  expect(poolInfo.unwrap().minimumStakingAmount).to.be.bignumber.equal(minimumStakingAmount);
}

export async function verifyPoolPotAddition(
  api: ApiPromise,
  stakingPoolId: u128,
  assetId: number,
  amount: number,
  walletPoolOwner: KeyringPair,
  walletBalanceBefore: CustomRpcBalance
) {
  // Querying `rewardsPotIsEmpty` now should report `None` type.
  const poolInfo = <Option<any>>await api.query.stakingRewards.rewardsPotIsEmpty(stakingPoolId, assetId);
  expect(poolInfo.isNone).to.be.true;

  // Balance checks
  const walletBalanceAfter = await api.rpc.assets.balanceOf(assetId.toString(), walletPoolOwner.publicKey);
  const expectedBalance = new BN(walletBalanceBefore.toString()).sub(new BN(amount));
  expect(expectedBalance).to.be.bignumber.equal(new BN(walletBalanceAfter.toString()));
}

export async function verifyPoolStaking(
  api: ApiPromise,
  fNFTCollectionId: u128,
  fNFTInstanceId: u64,
  stakeAmount: number | string,
  stakeAssetId: u128,
  walletStaker: KeyringPair,
  userFundsBefore: CustomRpcBalance
) {
  // Comparing with data from Query
  const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
    await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
  );
  // The query will report the stake amount equal to the amount we staked.
  expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(new BN(stakeAmount));
  // Checking funds
  const userFundsAfter = await api.rpc.assets.balanceOf(stakeAssetId.toString(), walletStaker.publicKey);
  // Making sure the amount funds left, of the staking asset, is exactly the amount,
  // subtracted by our staked amount.
  const expectedFunds = new BN(userFundsBefore.toString()).sub(new BN(stakeAmount));
  expect(expectedFunds).to.be.bignumber.equal(new BN(userFundsAfter.toString()));
}

export async function verifyPoolClaiming(
  api: ApiPromise,
  fNFTCollectionId: u128,
  fNFTInstanceId: u64,
  poolRewardAssetId: AnyNumber[] | u128[] | number[],
  walletStaker: KeyringPair,
  userFundsBefore: CustomRpcBalance[],
  claimableAmount: u128
) {
  // Checking funds
  for (const [index, assetId] of poolRewardAssetId.entries()) {
    const userFundsAfter = await api.rpc.assets.balanceOf(assetId.toString(), walletStaker.publicKey);
    const claimedAmount = new BN(userFundsAfter.toString()).sub(new BN(userFundsBefore[index].toString()));
    expect(claimedAmount).to.be.bignumber.equal(claimableAmount);
  }
}

export async function verifyPositionExtension(
  api: ApiPromise,
  fNFTCollectionId: u128,
  fNFTInstanceId: u64,
  stakeInfoBefore: Option<ComposableTraitsStakingStake>,
  amount: number,
  walletStaker: KeyringPair,
  poolBaseAssetId: number,
  userFundsBefore: CustomRpcBalance,
  shareAssetId: u128
) {
  // Querying stake
  const stakeInfoAfter = <Option<ComposableTraitsStakingStake>>(
    await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId)
  );
  // Making sure the newly reported stake amount is equal to the previous amount as well as our added amount.
  const expectedStakeAmount = stakeInfoBefore.unwrap().stake.add(new BN(amount));
  expect(stakeInfoAfter.unwrap().stake).to.be.bignumber.equal(expectedStakeAmount);
  // Making sure the share amount is equal to ???.
  const expectedShareAmount = await api.query.tokens.totalIssuance(shareAssetId);
  expect(stakeInfoAfter.unwrap().share).to.be.bignumber.equal(expectedShareAmount);

  // Checking funds
  const userFundsAfter = await api.rpc.assets.balanceOf(poolBaseAssetId.toString(), walletStaker.publicKey);
  const expectedFunds = new BN(userFundsBefore.toString()).add(new BN(amount));
  expect(new BN(userFundsAfter.toString())).to.be.bignumber.closeTo(expectedFunds, expectedFunds.div(new BN(100))); // Within 1% due to slashing discrepancy.
}

export async function verifyPositionSplitting(
  api: ApiPromise,
  originalFNFTCollectionId: u128,
  originalFNFTInstanceId: u64,
  stakeInfoBefore: Option<ComposableTraitsStakingStake>,
  splitA: number,
  splitB: number,
  splitAIndex: u128 | number | AnyNumber,
  splitBIndex: u128 | number | AnyNumber
) {
  // Verification
  // Querying stake info
  const stakeInfo1After = <Option<ComposableTraitsStakingStake>>(
    await api.query.stakingRewards.stakes(originalFNFTCollectionId, originalFNFTInstanceId)
  );
  const stakeInfo2After = <Option<ComposableTraitsStakingStake>>(
    await api.query.stakingRewards.stakes(splitAIndex, splitBIndex)
  );
  const expectedStakeAmount1 = stakeInfoBefore.unwrap().stake.muln(splitA);
  const expectedShareAmount1 = stakeInfoBefore.unwrap().stake.muln(splitA);
  const expectedStakeAmount2 = stakeInfoBefore.unwrap().stake.muln(splitB);
  const expectedShareAmount2 = stakeInfoBefore.unwrap().stake.muln(splitB);

  const stakeRange1 = expectedStakeAmount1.div(new BN(1000)); // within .1%
  const shareRange1 = expectedShareAmount1.div(new BN(1000));
  const stakeRange2 = expectedStakeAmount2.div(new BN(1000));
  const shareRange2 = expectedShareAmount2.div(new BN(1000));
  expect(stakeInfo1After.unwrap().stake).to.be.bignumber.closeTo(expectedStakeAmount1, stakeRange1);
  expect(stakeInfo1After.unwrap().share).to.be.bignumber.closeTo(expectedShareAmount1, shareRange1);
  expect(stakeInfo2After.unwrap().stake).to.be.bignumber.closeTo(expectedStakeAmount2, stakeRange2);
  expect(stakeInfo2After.unwrap().share).to.be.bignumber.closeTo(expectedShareAmount2, shareRange2);
}

export async function verifyPositionUnstaking(
  api: ApiPromise,
  fNFTCollectionId: u128,
  fNFTInstanceId: u64,
  poolBaseAssetId: number,
  walletStaker: KeyringPair,
  userFundsBefore: CustomRpcBalance,
  stakedAmount: number | string,
  slashed = false,
  slashAmount = api.createType("u128", 0)
) {
  // Expecting wallets stake to return nothing.
  const stakeInfoAfter = await api.query.stakingRewards.stakes(fNFTCollectionId, fNFTInstanceId).catch(function (e) {
    return e;
  });
  expect(stakeInfoAfter.toString()).to.equal("");

  // Checking user funds
  const userFundsAfter = await api.rpc.assets.balanceOf(poolBaseAssetId.toString(), walletStaker.publicKey);
  // If the user was slashed, we need to take the slashing amount into consideration.
  if (slashed) {
    expect(slashAmount).to.be.bignumber.greaterThan(new BN(0));
    const expectedFunds = new BN(userFundsBefore.toString()).add(new BN(stakedAmount).sub(new BN(slashAmount)));
    expect(new BN(userFundsAfter.toString())).to.be.bignumber.greaterThan(expectedFunds);
  } else {
    const expectedFunds = new BN(userFundsBefore.toString()).add(new BN(stakedAmount));
    expect(new BN(userFundsAfter.toString())).to.be.bignumber.equal(expectedFunds);
  }
}

export function getClaimOfStake(
  api: ApiPromise,
  stakeInfo: ComposableTraitsStakingStake,
  stakingRewardPool: ComposableTraitsStakingRewardPool,
  rewardAssetId: string,
  totalShareAssetIssuance: BN
) {
  if (totalShareAssetIssuance.eqn(0)) {
    return new BN(0);
  } else {
    const inflation = new BN(stakeInfo.reductions[rewardAssetId] * Math.pow(10, -12)) || new BN(0);
    let totalRewards: u128 | undefined = undefined;
    stakingRewardPool.rewards.forEach(function (reward) {
      if (reward.totalRewards) totalRewards = reward.totalRewards;
    });

    if (totalRewards == undefined) totalRewards = api.createType("u128", 0);
    const share: BN = stakeInfo.share;
    const myShare = totalRewards.mul(share).div(totalShareAssetIssuance);
    return myShare.sub(inflation);
  }
}
