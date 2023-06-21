import { PabloTransactions } from "@/defi/subsquid/pools/queries";
import {
  fromChainUnits,
} from "@/defi/utils";
import BigNumber from "bignumber.js";

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