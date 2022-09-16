import { CustomRpcBalance } from "@/../../packages/defi-interfaces";
import { StakingPositionHistory, StakingRewardPool } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import { bnToU8a, stringToU8a, u8aToHex } from "@polkadot/util";
import BigNumber from "bignumber.js";
import { BN } from "bn.js";
import { PALLET_TYPE_ID } from "../constants";
import { concatU8a } from "../misc";
import { fromChainUnits } from "../units";

export async function fetchOwnedFinancialNfts(
  parachainApi: ApiPromise,
  account: string
) {
  let ownedNfts: Record<string, Array<string>> = {};

  try {
    const encodedResponse = await parachainApi.query.fnft.ownerInstances(
      account
    );
    ownedNfts = (
      encodedResponse.toJSON() as [number | string, number | string][]
    ).reduce((agg, [collectionId, instanceId]) => {
      const key = new BigNumber(collectionId).toString();
      const val = new BigNumber(instanceId).toString();
      if (agg[key]) {
        agg[collectionId].push(val);
      } else {
        agg[collectionId] = [val];
      }
      return agg;
    }, {} as Record<string, Array<string>>);
  } catch (error: any) {
    console.log(error.message);
  }

  return ownedNfts;
}

export function createFinancialNftAccountId(
  parachainApi: ApiPromise,
  financialNFTCollectionId: string,
  financialNFTInstanceId: string
) {
  console.log(financialNFTInstanceId)
  const palletId = parachainApi.consts.fnft.palletId.toU8a();
  const accountPrefix = concatU8a(PALLET_TYPE_ID, palletId);
  const collectionId = bnToU8a(new BN(financialNFTCollectionId));
  const instanceId = bnToU8a(new BN(financialNFTInstanceId));
  const accountSuffix = parachainApi.createType("(u128, u64)", [
    collectionId,
    instanceId
  ]).toU8a().subarray(0, 40);
  const accountId = concatU8a(accountPrefix, accountSuffix);
  return parachainApi.createType("AccountId32", accountId);
}

export async function fetchXTokenBalances(
  parachainApi: ApiPromise,
  myStakingPositionHistory: StakingPositionHistory[],
  stakingRewardPool: StakingRewardPool
): Promise<Record<string, Record<string, BigNumber>>> {
  let xTokenStore: Record<string, Record<string, BigNumber>> = {};
  try {
    for (const history of myStakingPositionHistory) {
      const { fnftCollectionId, fnftInstanceId } = history;
      const accountId = createFinancialNftAccountId(parachainApi, fnftCollectionId, fnftInstanceId);
      let xTokenBalance: CustomRpcBalance = await parachainApi.rpc.assets.balanceOf(stakingRewardPool.shareAssetId, accountId);
      let xTokenBalanceBn = fromChainUnits(xTokenBalance.toString());
      if (!(fnftCollectionId in xTokenStore)) {
        xTokenStore[fnftCollectionId] = { [fnftInstanceId]: xTokenBalanceBn }
      } else if (!(fnftInstanceId in xTokenStore[fnftCollectionId])) {
        xTokenStore[fnftCollectionId][fnftInstanceId] = xTokenBalanceBn
      }
    }
  } catch (error: any) {
    console.error(error.message);
  }
  return xTokenStore
}