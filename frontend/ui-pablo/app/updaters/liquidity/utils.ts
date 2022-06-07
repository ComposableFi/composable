import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { createPoolAccountId } from "@/utils/substrate";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fetchBalanceByAssetId } from "../balances/utils";
import { DEFAULT_NETWORK_ID } from "../constants";

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
    const poolAccount = createPoolAccountId(parachainApi, pool.poolId);
    const liqBase = await fetchBalanceByAssetId(
      parachainApi,
      DEFAULT_NETWORK_ID,
      poolAccount,
      pool.pair.base.toString()
    );
    const liqQuote = await fetchBalanceByAssetId(
      parachainApi,
      DEFAULT_NETWORK_ID,
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

  const baseAssetOnChainId = Number(transactions[0].baseAssetId);
  const quoteAssetOnChainId = Number(transactions[0].quoteAssetId);
  let baseAssetDecimals: number | BigNumber = 12,
    quoteAssetDecimals: number | BigNumber = 12;

  try {
    baseAssetDecimals = getAssetByOnChainId(
      DEFAULT_NETWORK_ID,
      baseAssetOnChainId
    ).decimals;
    quoteAssetDecimals = getAssetByOnChainId(
      DEFAULT_NETWORK_ID,
      quoteAssetOnChainId
    ).decimals;
  } catch (err) {
    baseAssetDecimals = 12;
    quoteAssetDecimals = 12;
    console.log(err);
  }

  baseAssetDecimals = new BigNumber(10).pow(baseAssetDecimals);
  quoteAssetDecimals = new BigNumber(10).pow(quoteAssetDecimals);

  transactions.forEach((tx) => {
    if (tx.transactionType === "ADD_LIQUIDITY") {
      baseAmountProvided = baseAmountProvided.plus(
        new BigNumber(tx.baseAssetAmount).div(baseAssetDecimals)
      );
      quoteAmountProvided = quoteAmountProvided.plus(
        new BigNumber(tx.quoteAssetAmount).div(quoteAssetDecimals)
      );
    } else if (tx.transactionType === "REMOVE_LIQUIDITY") {
      baseAmountProvided = baseAmountProvided.minus(
        new BigNumber(tx.baseAssetAmount).div(baseAssetDecimals)
      );
      quoteAmountProvided = quoteAmountProvided.minus(
        new BigNumber(tx.quoteAssetAmount).div(quoteAssetDecimals)
      );
    }
  });

  return {
    baseAmountProvided,
    quoteAmountProvided,
  };
}