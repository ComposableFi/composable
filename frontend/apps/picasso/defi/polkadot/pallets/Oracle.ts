import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";
import { useStore } from "@/stores/root";
import { CurrencyId } from "defi-interfaces";

export async function subscribeAssetPrice(
  assetId: CurrencyId,
  api: ApiPromise
) {
  try {
    const unsub: any = await api.query.oracle.prices(
      assetId.toString(),
      (prices: any) => {
        const price = prices.isNone
          ? new BigNumber(0)
          : fromChainIdUnit(new BigNumber(prices.price.toString()));
        useStore.setState({
          oracle: {
            prices: {
              [+assetId.toString()]: {
                price,
                block: new BigNumber(prices.block.toString()),
              },
            },
          },
        });

        return unsub;
      }
    );
  } catch (e) {
    return () => {};
  }
}

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
            block: new BigNumber(jsonPrices.block.toString()),
          },
        },
      },
    });

    return price;
  } catch (e) {
    console.error("Defaulting to zero", e);
    return new BigNumber(0);
  }
}
