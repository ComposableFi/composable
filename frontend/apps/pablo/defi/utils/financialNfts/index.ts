import { CustomRpcBalance } from "@/../../packages/defi-interfaces";
import { StakingPositionHistory, StakingRewardPool } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import { BN } from "bn.js";
import { BLAKE_HASH_BIT_LENGTH, PALLET_TYPE_ID } from "../constants";
import { concatU8a } from "../misc";
import { fromChainUnits } from "../units";
import { hexToU8a } from "@polkadot/util";
import { blake2AsHex } from "@polkadot/util-crypto";
import BigNumber from "bignumber.js";

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

/** this will change later from pallet team */
export function createFinancialNftAccountId(
  api: ApiPromise,
  financialNFTCollectionId: string,
  financialNFTInstanceId: string
) {
  const palletId = api.consts.fnft.palletId.toU8a();
  const accountPrefix = concatU8a(PALLET_TYPE_ID, palletId);

  const collectionId = new BN(financialNFTCollectionId);
  const instanceId = new BN(financialNFTInstanceId);

  const tuple = api.createType("(u128, u64)", [
    collectionId,
    instanceId
  ]);
  /**
   * Only used here otherwise 
   * can be exported to constants file
   */
  const TRUNCATE_BITS = 20;
  const blakeHash = blake2AsHex(tuple.toU8a(), BLAKE_HASH_BIT_LENGTH);  
  const accountId = concatU8a(accountPrefix, hexToU8a(blakeHash).subarray(0, TRUNCATE_BITS));
  return api.createType("AccountId32", accountId);
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
    console.error('fetchXTokenBalances ', error);
  }
  return xTokenStore
}