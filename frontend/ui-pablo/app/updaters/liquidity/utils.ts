import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";

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