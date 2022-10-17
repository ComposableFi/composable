import { LiquidityBootstrappingPool } from "@/defi/types";
import BigNumber from "bignumber.js";

export function calculateWeightAt(
  pool: LiquidityBootstrappingPool,
  current_block: BigNumber
): { baseWeight: BigNumber; quoteWeight: BigNumber } {
  let baseWeight = new BigNumber(0),
    quoteWeight = new BigNumber(0);

  let one = new BigNumber(1);

  let normalized_current_block = new BigNumber(current_block).minus(pool.sale.startBlock);
  let pointInSale = normalized_current_block.div(new BigNumber(pool.sale.endBlock).minus(pool.sale.startBlock))
  let weightRange = new BigNumber(pool.sale.initialWeight)
    .div(100)
    .minus(new BigNumber(pool.sale.finalWeight).div(100));

  baseWeight = new BigNumber(pool.sale.initialWeight)
    .div(100)
    .minus(pointInSale.times(weightRange));
  quoteWeight = one.minus(baseWeight);

  return {
    baseWeight,
    quoteWeight,
  };
}

export function lbpCalculatePriceAtBlock(
  auction: LiquidityBootstrappingPool,
  baseAum: BigNumber,
  quoteAum: BigNumber,
  blockNumber: BigNumber
): BigNumber {
  let { baseWeight, quoteWeight } = calculateWeightAt(auction, blockNumber);
  let baseNum = baseAum.div(baseWeight);
  let quoteNum = quoteAum.div(quoteWeight);
  return quoteNum.div(baseNum);
}