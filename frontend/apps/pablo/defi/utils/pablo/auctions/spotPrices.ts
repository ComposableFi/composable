import { LiquidityBootstrappingPool } from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fetchSpotPrice } from "../spotPrice";

export async function fetchAuctionSpotPrices(
  parachainApi: ApiPromise | undefined,
  lbpPools: LiquidityBootstrappingPool[]
): Promise<Record<string, BigNumber>> {
  const spotPricesRecord: Record<string, BigNumber> = {};

  try {
    if (!parachainApi || lbpPools.length == 0)
      throw new Error("Cannot fetch prices.");

    for (const pool of lbpPools) {
      const { base, quote } = pool.pair;
      const poolSpotPrice = await fetchSpotPrice(
        parachainApi,
        {
          base: base.toString(),
          quote: quote.toString(),
        },
        pool.poolId
      );

      const poolKey = pool.poolId.toString();
      spotPricesRecord[poolKey] = poolSpotPrice;
    }
  } catch (err: any) {
    console.error("Error: ", err.message);
  } finally {
    return spotPricesRecord;
  }
}
