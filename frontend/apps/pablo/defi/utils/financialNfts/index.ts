import { ApiPromise } from "@polkadot/api";
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
