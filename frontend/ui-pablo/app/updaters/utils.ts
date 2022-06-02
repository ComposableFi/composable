import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";

/* See task https://app.clickup.com/t/2u9un3m
 * how to create AccountId for derived Accounts
 * within a pallet
 */
export function createPoolAccountId(poolId: number): string {
    enum PalletIds {
      PalletsId = "0x6d6f646c",
      Pablo = "70616c6c5f706162",
    }

    const end = new Array(27).fill("0").join("");
    const start = new Array(13).fill("0").join("");
    // JS eats the leading zero below 10
    let poolIdStr = poolId < 16 ? "0" + poolId.toString(16) : poolId.toString(16)
    const startWithId = (poolIdStr + start).substring(0, 13);

    return (
      PalletIds.PalletsId.toString() +
      PalletIds.Pablo.toString() +
      (startWithId + end)
    ).substring(0, 66);
}
/**
 * Check if pair is valid
 * @param asset1 Asset id | "none"
 * @param asset2 Asset id | "none"
 * @returns boolean
 */
export function isValidAssetPair(asset1: AssetId | "none", asset2: AssetId | "none"): boolean {
    return asset1 !== "none" && asset2 !== "none";
}

export function toTokenUnits(amount: number | string, decimals: number = 12): BigNumber {
  return new BigNumber(amount).times(new BigNumber(10).pow(decimals))
}

export const processLiquidityTransactionsByAddress = (transactions: any[]): {base: BigNumber, quote: BigNumber} => {
  const mapp = transactions.map(
    (tx: {
      baseAssetId: string;
      baseAssetAmount: string;
      quoteAssetAmount: string;
      quoteAssetId: string;
      receivedTimestamp: string;
      transactionType: "ADD_LIQUIDITY" | "REMOVE_LIQUIDITY";
      who: string;
      pool: {
        poolId: string;
      };
    }) => {
      const baseAssetId = Number(tx.baseAssetId);
      const quoteAssetId = Number(tx.quoteAssetId);
      const receivedTimestamp = Number(tx.receivedTimestamp);
      const poolId = Number(tx.pool.poolId);

      const bDecs = new BigNumber(10).pow(
        getAssetByOnChainId("picasso", baseAssetId).decimals ?? 12
      );
      const qDecs = new BigNumber(10).pow(
        getAssetByOnChainId("picasso", quoteAssetId).decimals ?? 12
      );

      const baseAssetAmount = new BigNumber(tx.baseAssetAmount)
        .div(bDecs)
        .toString();
      const quoteAssetAmount = new BigNumber(tx.baseAssetAmount)
        .div(qDecs)
        .toString();

      return {
        transactionType: tx.transactionType,
        baseAssetId,
        quoteAssetId,
        receivedTimestamp,
        poolId,
        baseAssetAmount,
        quoteAssetAmount,
      };
    }
  ) as {
    transactionType: "ADD_LIQUIDITY" | "REMOVE_LIQUIDITY";
    baseAssetId: number;
    quoteAssetId: number;
    receivedTimestamp: number;
    poolId: number;
    baseAssetAmount: string;
    quoteAssetAmount: string;
  }[];

  let baseProvided = mapp.reduce((p: any, c: any) => {
    const agg = new BigNumber(p);
    if (c.transactionType === "ADD_LIQUIDITY") {
      return agg.plus(c.baseAssetAmount).toString();
    } else {
      return agg.minus(c.baseAssetAmount).toString();
    }
  }, "0");

  let quoteProvided = mapp.reduce((p: any, c: any) => {
    const agg = new BigNumber(p);
    if (c.transactionType === "ADD_LIQUIDITY") {
      return agg.plus(c.baseAssetAmount).toString();
    } else {
      return agg.minus(c.baseAssetAmount).toString();
    }
  }, "0");

  return {
    base: new BigNumber(baseProvided),
    quote: new BigNumber(quoteProvided)
  }
}