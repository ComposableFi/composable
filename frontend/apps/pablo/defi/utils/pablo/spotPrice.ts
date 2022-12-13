import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainUnits, toChainUnits } from "../units";

export async function fetchSpotPrice(
  api: ApiPromise,
  pair: {
    base: string;
    quote: string;
  },
  poolId: number
): Promise<BigNumber> {
  try {
    const pricesFor = await api.rpc.pablo.pricesFor(
      api.createType("PalletPabloPoolId", poolId.toString()),
      api.createType("CustomRpcCurrencyId", pair.base),
      api.createType("CustomRpcCurrencyId", pair.quote),
      api.createType("CustomRpcBalance", toChainUnits(1).toString())
    );

    const spotPrice = pricesFor.get("spotPrice");
    return fromChainUnits(spotPrice ? spotPrice.toString() : "0");
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
}