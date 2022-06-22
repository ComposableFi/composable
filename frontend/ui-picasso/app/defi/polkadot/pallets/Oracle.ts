import { u128 } from "@polkadot/types-codec";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { store } from "@/stores/root";
import { updatePrice } from "@/stores/defi/polkadot/oracle/slice";
import { fromChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";

export async function fetchAssetPrice(assetId: u128, api: ApiPromise) {
  try {
    const prices: any = await api.query.oracle.prices(assetId); // TODO[type-gen]: replace any with proper type
    const jsonPrices = prices.toJSON();
    const price = jsonPrices
      ? fromChainIdUnit(new BigNumber(jsonPrices.price))
      : new BigNumber(0);
    // Dispatch an action to OracleStore to cache this value.
    store.dispatch(
      updatePrice({
        [+assetId.toString()]: {
          price,
          block: new BigNumber(jsonPrices.block.toString()),
        },
      })
    );

    return price;
  } catch (e) {
    console.error("Defaulting to zero", e);
    return new BigNumber(0);
  }
}
