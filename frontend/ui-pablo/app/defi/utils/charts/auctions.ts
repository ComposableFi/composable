import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";
import BigNumber from "bignumber.js";
import { DAYS } from "../constants";
import { getCurrentWeights } from "../pablo/auctions";

export function calculatePredictedChartSeries(
  lastTransactionTimestamp: number,
  baseBalance: BigNumber,
  quoteBalance: BigNumber,
  pool: LiquidityBootstrappingPool,
  current_block_number: BigNumber,
  blocks_per_day = 7200
): [number, number][] {
  let series: [number, number][] = [];

  let nextPoint = lastTransactionTimestamp;
  let blockNumber = current_block_number;
  // const priceWFee = new BigNumber(1).div(new BigNumber(1).minus(new BigNumber(pool.feeConfig.feeRate).div(100)))

  while (nextPoint < pool.sale.end) {
    const weightAtT = getCurrentWeights(pool, new BigNumber(blockNumber));
    const quoteWeight = quoteBalance.div(weightAtT.quoteWeight);
    const baseWeight = baseBalance.div(weightAtT.baseWeight);

    const price = quoteWeight.div(baseWeight);
    series.push([nextPoint, price.toNumber()]);
    nextPoint += 1 * DAYS;
    blockNumber = blockNumber.plus(blocks_per_day)
  }

  return series;
}
