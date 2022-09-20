import {ApiPromise} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import {
  sendAndWaitForMultipleEvents,
  sendAndWaitForSuccess,
  sendWithBatchAndWaitForSuccess
} from "@composable/utils/polkadotjs";
import {Pica} from "@composable/utils/mintingHelper";
import {AccountId32} from "@polkadot/types/interfaces/runtime";

export async function createConsProdPoolforFnft(
  api: ApiPromise,
  sudoKey: KeyringPair,
  owner: KeyringPair,
  baseAssetId: number,
  quoteAssetId: number,
  fee: number,
  baseWeight: number
) {
  const pool = api.createType("PalletPabloPoolInitConfiguration", {
    ConstantProduct: {
      owner: api.createType("AccountId32", owner.address),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", baseAssetId),
        quote: api.createType("u128", quoteAssetId)
      }),
      fee: api.createType("Permill", fee),
      baseWeight: api.createType("Permill", baseWeight)
    }
  });
  const events = await sendAndWaitForMultipleEvents(
    api,
    sudoKey,
    [api.events.fnft.FinancialNftCollectionCreated.is,
      api.events.stakingRewards.RewardPoolCreated.is,
      api.events.pablo.PoolCreated.is],
    api.tx.sudo.sudo(api.tx.pablo.create(pool)),
    false
  );
  const {data: [collectionId, who,]} = events[0];
  const {data: [rewardsPoolId, rewardPoolOwner,]} = events[1];
  const {data: [pabloPoolId, pabloPoolOwner,]} = events[2];
  return {
    fnftCollectionInfo: {
      fnftCollectionId: parseInt(collectionId.toString()),
      fnftCollectionOwner: who.toString()
    },
    rewardPoolInfo: {
      rewardPoolId: parseInt(rewardsPoolId.toString()),
      rewardPoolOwner: rewardPoolOwner.toString()
    },
    pabloPoolInfo: {
      poolId: parseInt(pabloPoolId.toString()),
      poolOwner: pabloPoolOwner.toString()
    }
  }
}

export async function createStableSwapPoolforFnft(
  api: ApiPromise,
  sudoKey: KeyringPair,
  owner: KeyringPair,
  baseAssetId: number,
  quoteAssetId: number,
  ampCoefficient: number,
  fee: number
) {
  const pool = api.createType("PalletPabloPoolInitConfiguration", {
    StableSwap: {
      owner: api.createType("AccountId32", owner.address),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", baseAssetId),
        quote: api.createType("u128", quoteAssetId)
      }),
      amplification_coefficient: api.createType("u16", ampCoefficient),
      fee: api.createType("Permill", fee)
    }
  });
  const events = await sendAndWaitForMultipleEvents(
    api,
    sudoKey,
    [api.events.fnft.FinancialNftCollectionCreated.is,
      api.events.stakingRewards.RewardPoolCreated.is,
      api.events.pablo.PoolCreated.is],
    api.tx.sudo.sudo(api.tx.pablo.create(pool)),
    false
  );
  const {data: [collectionId, who,]} = events[0];
  const {data: [rewardsPoolId, rewardPoolOwner,]} = events[1];
  const {data: [pabloPoolId, pabloPoolOwner,]} = events[2];
  return {
    fnftCollectionInfo: {
      fnftCollectionId: parseInt(collectionId.toString()),
      fnftCollectionOwner: who.toString()
    },
    rewardPoolInfo: {
      rewardPoolId: parseInt(rewardsPoolId.toString()),
      rewardPoolOwner: rewardPoolOwner.toString()
    },
    pabloPoolInfo: {
      poolId: parseInt(pabloPoolId.toString()),
      poolOwner: pabloPoolOwner.toString()
    }
  }
}

export async function addFundsToThePools(api: ApiPromise, pools: number[], funderWallet: KeyringPair) {
  const txs = [];
  for (const poolId of pools) {
    const pool = api.createType("u128", poolId);
    const baseAmount = api.createType("u128", Pica(250000));
    const quoteAmount = api.createType("u128", Pica(250000));
    const keepAlive = api.createType("bool", true);
    const minMintAmount = api.createType("u128", 0);
    txs.push(api.tx.pablo.addLiquidity(pool, baseAmount, quoteAmount, minMintAmount, keepAlive));
  }
  await sendWithBatchAndWaitForSuccess(
    api,
    funderWallet,
    api.events.system.NewAccount.is,
    txs,
    false
  );
}

export async function addRewardsToRewardPools(
  api: ApiPromise,
  rewardPools: number[],
  assetIdP: number,
  funderWallet: KeyringPair,
  amount: number) {
  const txs = [];
  for (const rewardPool of rewardPools) {
    const pool = api.createType("u128", rewardPool);
    const assetId = api.createType("u128", assetIdP);
    const addAmount = api.createType("u128", Pica(amount));
    const keepAlive = api.createType("bool", false);
    txs.push(api.tx.stakingRewards.addToRewardsPot(
      pool,
      assetId,
      addAmount,
      keepAlive)
    );
  }
  await sendWithBatchAndWaitForSuccess(
    api,
    funderWallet,
    api.events.stakingRewards.RewardsPotIncreased.is,
    txs,
    false
  );
}

export async function stakeLpTokens(
  api: ApiPromise,
  rewardPool: number,
  lpProviderWallet: KeyringPair,
  lpAmount: number,
  lockTime: number): Promise<AccountId32> {
  const poolId = api.createType("u128", rewardPool);
  const amountParam = api.createType("u128", Pica(lpAmount));
  const duration = api.createType("u64", lockTime);
  const {data: [, who,]} = await sendAndWaitForSuccess(
    api,
    lpProviderWallet,
    api.events.tokens.Endowed.is,
    api.tx.stakingRewards.stake(poolId, amountParam, duration)
  );
  return who;
}

export async function getLockedToken(api: ApiPromise, accountId: AccountId32, stakedTokenId: number) {
  const tokensRaw = await api.query.tokens.locks(accountId, stakedTokenId);
  const locked = tokensRaw[0].amount.toBigInt() / BigInt(10 ** 12);
  return Number(locked);
}

export async function getCollectionInstances(api: ApiPromise, fnftCollectionId: number) {
  const lastId = await api.query.fnft.financialNftId(fnftCollectionId);
  return lastId.toNumber() - 1;
}

export async function getCollectionInstanceOwner(api: ApiPromise, fnftCollectionId: number, fnftInstanceId: number) {
  const rawInfo = await api.query.fnft.instance(fnftCollectionId, fnftInstanceId);
  return rawInfo.unwrap()[0].toString();
}

export async function extendStakingPosition(
  api: ApiPromise,
  lpProvider: KeyringPair,
  fnftCollectionId: number,
  instanceId: number,
  amountToBeAdded: number) {
  const collectionId = api.createType("u128", fnftCollectionId);
  const instance = api.createType("u64", instanceId);
  const amount = api.createType("u128", Pica(amountToBeAdded));
  await sendAndWaitForSuccess(
    api,
    lpProvider,
    api.events.stakingRewards.StakeAmountExtended.is,
    api.tx.stakingRewards.extend(collectionId, instance, amount)
  );
}

export async function createStakingRewardPool(
  api: ApiPromise,
  poolOwner: KeyringPair,
  stakeAssetId: number,
  rewardAsset: number) {
  const rewardConfigs = api.createType("ComposableTraitsStakingRewardConfig", {
    assetId: api.createType("u128", rewardAsset),
    maxRewards: api.createType("u128", Pica(5000000)),
    rewardRate: {
      period: {
        PerSecond: api.createType("u128", 100000)
      },
      amount: Pica(1)
    }
  });
  const lockConfig = api.createType("u64", 5 * 60);
  const lockMap = new Map();
  lockMap.set(lockConfig, api.createType("Perbill", 100000000));
  lockMap.set(api.createType("u64", 10 * 60), api.createType("Perbill", 300000000));
  const configMap = new Map();
  configMap.set(api.createType("u128", 1), rewardConfigs);
  const financialNftAssetId = stakeAssetId + 200000;
  const poolConfig = api.createType("ComposableTraitsStakingRewardPoolConfiguration", {
    RewardRateBasedIncentive: {
      owner: api.createType("AccountId32", poolOwner.address),
      assetId: api.createType("u128", stakeAssetId),
      endBlock: api.createType("u32", 25000),
      rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", configMap),
      lock: api.createType("ComposableTraitsStakingLockLockConfig", {
        durationPresets: api.createType("BTreeMap<u64, Perbill>", lockMap),
        unlockPenalty: api.createType("Perbill", 500000000)
      }),
      shareAssetId: api.createType("u128", stakeAssetId + 100000),
      financialNftAssetId: api.createType("u128", stakeAssetId + 200000)
    }
  })
  const {data: [poolId,]} = await sendAndWaitForSuccess(
    api,
    poolOwner,
    api.events.stakingRewards.RewardPoolCreated.is,
    api.tx.sudo.sudo(
      api.tx.stakingRewards.createRewardPool(poolConfig)),
    false
  )
  return {
    poolId: poolId.toNumber(),
    fnftAssetId: financialNftAssetId
  };
}


export async function getUserFnfts(api: ApiPromise, stakerWallet: KeyringPair) {
  const instancesRaw = await api.query.fnft.ownerInstances(stakerWallet.address);
  return Array.from(instancesRaw.unwrap());
}

export async function unstakeAndBurn(
  api: ApiPromise, stakerWallet: KeyringPair,
  fnftCollectionId: number,
  fnftCollectionInstance: number) {
  const fnftCollectionIdP = api.createType("u128", fnftCollectionId);
  const fnftInstanceP = api.createType("u128", fnftCollectionInstance);
  const {data: [owner, collectionId, instanceId]} = await sendAndWaitForSuccess(
    api,
    stakerWallet,
    api.events.stakingRewards.Unstaked.is,
    api.tx.stakingRewards.unstake(fnftCollectionIdP, fnftInstanceP)
  );
  return {
    owner: owner,
    fnftCollectionId: collectionId.toNumber(),
    fnftInstanceId: instanceId.toNumber()
  };
}

export async function getRewardPoolOwner(api: ApiPromise, rewardPoolId: number) {
  const rawPoolData = await api.query.stakingRewards.rewardPools(rewardPoolId);
  const poolData = rawPoolData.unwrap();
  return poolData.owner.toString();
}

export async function getCollectionOwnerAndAdmin(api: ApiPromise, fnftCollectionIdParam: number) {
  const rawCollectionData = await api.query.fnft.collection(fnftCollectionIdParam);
  const owner = rawCollectionData.unwrap()[0].toString();
  const admin = rawCollectionData.unwrap()[1].toString();
  return {
    owner: owner,
    admin: admin
  }
}