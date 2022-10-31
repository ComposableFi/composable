import { ApiPromise } from "@polkadot/api";
import { useStore } from "@/stores/root";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

export const subscribeFeeItemEd = async (api: ApiPromise) => {
  return useStore.subscribe(
    (store) => ({
      feeItem: store.transfers.feeItem,
      sourceChain: store.transfers.networks.from,
      isLoaded: store.substrateTokens.isLoaded,
    }),
    async ({ feeItem, isLoaded, sourceChain }) => {
      if (!isLoaded) return;
      const assetId =
        useStore.getState().substrateTokens.tokens[feeItem].chainId[
          sourceChain
        ];

      if (!assetId) {
        return;
      }

      try {
        const ed = await api.query.currencyFactory.assetEd(assetId.toString());
        const existentialString = ed.toString();
        const existentialValue = fromChainIdUnit(
          new BigNumber(existentialString)
        );
        useStore.setState({
          ...useStore.getState(),
          transfers: {
            ...useStore.getState().transfers,
            feeItemEd: existentialValue.isNaN()
              ? new BigNumber(0)
              : existentialValue,
          },
        });
      } catch (e) {
        console.log("Error while fetching existential deposit");
      }
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.feeItem === b.feeItem &&
        a.sourceChain === b.sourceChain &&
        a.isLoaded === b.isLoaded,
    }
  );
};
