import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainUnits, toChainUnits } from "../units";
import { PoolConfig } from "@/store/pools/types";

export async function fetchSpotPrice(
  api: ApiPromise,
  pool: PoolConfig,
  baseAssetId: string,
  quoteAssetId: string,
  decimals: number = 12
): Promise<BigNumber> {
  try {
    const pricesFor = await api.rpc.pablo.pricesFor(
      api.createType("PalletPabloPoolId", pool.poolId.toString()),
      api.createType("CustomRpcCurrencyId", baseAssetId),
      api.createType("CustomRpcCurrencyId", quoteAssetId),
      api.createType("CustomRpcBalance", toChainUnits(1).toString())
    );

    const spotPrice = pricesFor.get("spotPrice");
    return fromChainUnits(spotPrice ? spotPrice.toString() : "0", decimals);
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
}
