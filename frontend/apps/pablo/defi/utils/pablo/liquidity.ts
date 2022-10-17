import { PabloTransactions } from "@/defi/subsquid/pools/queries";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import {
  createPabloPoolAccountId,
  fetchAssetBalance,
  fetchBalanceByAssetId,
  fromChainUnits,
} from "@/defi/utils";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export async function fetchPoolLiquidity(
  parachainApi: ApiPromise,
  pools: {
    poolId: string | number;
    pair: {
      base: number;
      quote: number;
    };
  }[]
): Promise<Record<string, { baseAmount: BigNumber; quoteAmount: BigNumber }>> {
  let liquidityRecord: Record<
    string,
    { baseAmount: BigNumber; quoteAmount: BigNumber }
  > = {};

  for (const pool of pools) {
    try {
      const poolAccountId = createPabloPoolAccountId(parachainApi, Number(pool.poolId));
      const baseLiq = await fetchAssetBalance(parachainApi, poolAccountId, pool.pair.base.toString())
      const quoteLiq = await fetchAssetBalance(parachainApi, poolAccountId, pool.pair.quote.toString())

      liquidityRecord[pool.poolId] = {
        baseAmount: baseLiq,
        quoteAmount: quoteLiq
      }
    } catch (err: any) {
      console.error(err);
    }
  }

  return liquidityRecord;
}

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
    console.error(err)
    setTokenAmountInLiquidityPool(pool.poolId, {
      baseAmount: "0",
      quoteAmount: "0",
    });
  }
}

export function calculateProvidedLiquidity(
  transactions: PabloTransactions[]
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
    if (tx.event.eventType === "ADD_LIQUIDITY") {
      baseAmountProvided = baseAmountProvided.plus(
        fromChainUnits(tx.baseAssetAmount)
      );
      quoteAmountProvided = quoteAmountProvided.plus(
        fromChainUnits(tx.quoteAssetAmount)
      );
    } else if (tx.event.eventType === "REMOVE_LIQUIDITY") {
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

export function fromRemoveLiquiditySimulationResult(result: { assets: { [assetId: number | string]: string } } ): Record<string, BigNumber> {
  let liquidityRecord: Record<string, BigNumber> = {};

  for (const key in result.assets) {
    liquidityRecord[key] = fromChainUnits(result.assets[key]);
  }

  return liquidityRecord;
}