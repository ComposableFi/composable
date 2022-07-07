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
    const rpcResult = await (api.rpc as any).pablo.pricesFor(
      api.createType("PalletPabloPoolId", poolId.toString()),
      api.createType("CustomRpcCurrencyId", pair.base),
      api.createType("CustomRpcCurrencyId", pair.quote),
      api.createType("CustomRpcBalance", toChainUnits(1).toString())
    );

    return fromChainUnits(rpcResult.toJSON().spotPrice);
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
}

export async function fetchSpotPriceInverted(
  api: ApiPromise,
  pair: {
    base: string;
    quote: string;
  },
  poolId: number
): Promise<BigNumber> {
  try {
    const rpcResult = await (api.rpc as any).pablo.pricesFor(
      api.createType("PalletPabloPoolId", poolId.toString()),
      api.createType("CustomRpcCurrencyId", pair.quote),
      api.createType("CustomRpcCurrencyId", pair.base),
      api.createType("CustomRpcBalance", toChainUnits(1).toString())
    );

    return fromChainUnits(rpcResult.toJSON().spotPrice);
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
}
