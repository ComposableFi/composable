import { CustomRpcBalance } from "defi-interfaces";
import { StakingPositionHistory, StakingRewardPool } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import { fromChainUnits } from "../units";
import { FinancialNft } from "shared";
import BigNumber from "bignumber.js";

export async function fetchXTokenBalances(
  parachainApi: ApiPromise,
  myStakingPositionHistory: StakingPositionHistory[],
  stakingRewardPool: StakingRewardPool
): Promise<Record<string, Record<string, BigNumber>>> {
  let xTokenStore: Record<string, Record<string, BigNumber>> = {};
  try {
    for (const history of myStakingPositionHistory) {
      const { fnftCollectionId, fnftInstanceId } = history;
      const fNft = new FinancialNft(parachainApi, new BigNumber(fnftCollectionId), new BigNumber(fnftInstanceId))
      const accountId = fNft.getAccountId();
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