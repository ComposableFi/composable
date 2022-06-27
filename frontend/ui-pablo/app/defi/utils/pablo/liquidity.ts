import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import {
  createPabloPoolAccountId,
  fetchBalanceByAssetId,
  fromChainUnits,
} from "@/defi/utils";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export async function fetchAndUpdatePoolLiquidity(
  pool: ConstantProductPool | StableSwapPool,
  setTokenAmountInLiquidityPool: (
    poolId: number,
    amounts: {
      baseAmount?: string | undefined;
      quoteAmount?: string | undefined;
    }
  ) => void,
  parachainApi: ApiPromise
): Promise<void> {
  try {
    console.log("fetchAndUpdatePoolLiquidity: " + pool.poolId);
    const poolAccount = createPabloPoolAccountId(parachainApi, pool.poolId);
    const liqBase = await fetchBalanceByAssetId(
      parachainApi,
      poolAccount,
      pool.pair.base.toString()
    );
    const liqQuote = await fetchBalanceByAssetId(
      parachainApi,
      poolAccount,
      pool.pair.quote.toString()
    );
    setTokenAmountInLiquidityPool(pool.poolId, {
      baseAmount: liqBase,
      quoteAmount: liqQuote,
    });
  } catch (err) {
    setTokenAmountInLiquidityPool(pool.poolId, {
      baseAmount: "0",
      quoteAmount: "0",
    });
  }
}

export function calcaulateProvidedLiquidity(
  transactions: {
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
  }[]
): { baseAmountProvided: BigNumber; quoteAmountProvided: BigNumber } {
  let baseAmountProvided = new BigNumber(0);
  let quoteAmountProvided = new BigNumber(0);

  if (!transactions.length) {
    return {
      baseAmountProvided,
      quoteAmountProvided,
    };
  }

  transactions.forEach((tx) => {
    if (tx.transactionType === "ADD_LIQUIDITY") {
      baseAmountProvided = baseAmountProvided.plus(
        fromChainUnits(tx.baseAssetAmount)
      );
      quoteAmountProvided = quoteAmountProvided.plus(
        fromChainUnits(tx.quoteAssetAmount)
      );
    } else if (tx.transactionType === "REMOVE_LIQUIDITY") {
      baseAmountProvided = baseAmountProvided.minus(
        fromChainUnits(tx.baseAssetAmount)
      );
      quoteAmountProvided = quoteAmountProvided.minus(
        fromChainUnits(tx.quoteAssetAmount)
      );
    }
  });

  return {
    baseAmountProvided,
    quoteAmountProvided,
  };
}
