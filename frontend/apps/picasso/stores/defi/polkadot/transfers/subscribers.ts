import { TRANSFER_ASSET_LIST } from "@/defi/config";
import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";
import { getDefaultToken } from "@/stores/defi/polkadot/transfers/utils";
import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";

function extractOptions(from: SubstrateNetworkId): TokenOption[] {
  const list = useStore.getState().substrateBalances.assets[from];
  const options = Object.values(list.assets).reduce(
    (previousValue, currentValue) => {
      // no duplicates
      if (
        previousValue.find(
          (value: any) => value.tokenId === currentValue.meta.symbol
        )
      ) {
        return previousValue;
      }

      // calculate balance for token
      const isNative =
        "supportedNetwork" in currentValue.meta &&
        currentValue.meta.supportedNetwork[from] === 1;
      const balance = isNative
        ? useStore.getState().substrateBalances.assets[from].native.balance
        : currentValue.balance;

      // only include allowed assets
      if (
        !TRANSFER_ASSET_LIST[from].includes(
          currentValue.meta.symbol.toLowerCase()
        )
      ) {
        return previousValue;
      }

      return [
        ...previousValue,
        {
          tokenId: currentValue.meta.assetId,
          symbol: currentValue.meta.symbol,
          icon: currentValue.meta.icon,
          // disabled: balance.lte(0),
          // balance: balance,
        },
      ];
    },
    [] as TokenOption[]
  );
  return options;
}

function setOptions(options: TokenOption[]) {
  useStore.setState({
    ...useStore.getState(),
    transfers: {
      ...useStore.getState().transfers,
      tokenOptions: options,
    },
  });
}

export const subscribeTokenOptions = () => {
  return useStore.subscribe(
    (store) => store.transfers.networks.from,
    (from) => {
      const options = extractOptions(from);

      setOptions(options);
    },
    {
      fireImmediately: true,
    }
  );
};

export const subscribeDefaultTransferToken = () => {
  return useStore.subscribe(
    (store) => store.transfers.tokenOptions,
    (tokenOptions) => {
      const defaultToken = getDefaultToken(tokenOptions);

      useStore.setState({
        ...useStore.getState(),
        transfers: {
          ...useStore.getState().transfers,
          selectedToken: defaultToken,
        },
      });
    },
    {
      fireImmediately: true,
    }
  );
};

export const subscribeFeeItemEd = async (api: ApiPromise) => {
  return useStore.subscribe(
    (store) => store.transfers.feeItem,
    (feeItem) => {
      const assetId = getAssetOnChainId("picasso", feeItem);

      if (!assetId) {
        return;
      }

      const ed = api.query.currencyFactory.assetEd(assetId);

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
    }
  );
};
