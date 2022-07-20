import { LiquidityBootstrappingPool } from "@/defi/types";
import BigNumber from "bignumber.js";

export function caluclateWeightAt(
    pool: LiquidityBootstrappingPool,
    current_block: BigNumber
  ): { baseWeight: BigNumber; quoteWeight: BigNumber } {
    let baseWeight = new BigNumber(0),
      quoteWeight = new BigNumber(0);
  
    let one = new BigNumber(1);
    let pointInSale = new BigNumber(current_block).div(
      new BigNumber(pool.sale.endBlock).minus(pool.sale.startBlock)
    );
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
  