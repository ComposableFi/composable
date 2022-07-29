import { u128 } from "@polkadot/types-codec";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";
import { useStore } from "@/stores/root";
import { CurrencyId } from "defi-interfaces";

export async function fetchAssetPrice(assetId: CurrencyId, api: ApiPromise) {
  try {
    const prices: any = await api.query.oracle.prices(assetId.toString()); // TODO[type-gen]: replace any with proper type
    const jsonPrices = prices.toJSON();
    const price = jsonPrices
      ? fromChainIdUnit(new BigNumber(jsonPrices.price))
      : new BigNumber(0);

    useStore.setState({
      oracle: {
        prices: {
          [+assetId.toString()]: {
            price,
            block: new BigNumber(jsonPrices.block.toString())
          }
        }
      }
    });

    return price;
  } catch (e) {
    console.error("Defaulting to zero", e);
    return new BigNumber(0);
  }
}
