import { ApiPromise } from "@polkadot/api";
import { fromPica } from "../../fromPica";
import { stringToBigNumber } from "../../stringToBigNumber";

export const fetchApolloPriceByAssetId = async (
  api: ApiPromise,
  assetId: string | number
): Promise<string> => {
  try {
    let data = await api.query.oracle.prices(assetId);
    const decoded: any = data.toJSON();
    console.log("Oracle Price: ", decoded);
    return decoded.price;
  } catch (err: any) {
    return "0";
  }
};

export async function getAppoloPriceInUSD(
  parachainApi: ApiPromise,
  currencyId: string | number
) {
  const principalApolloPrice = await fetchApolloPriceByAssetId(
    parachainApi,
    currencyId
  );
  return fromPica(stringToBigNumber(principalApolloPrice));
}
