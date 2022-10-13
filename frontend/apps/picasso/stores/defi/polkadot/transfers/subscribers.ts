import { useStore } from "@/stores/root";
import { TRANSFER_ASSET_LIST } from "@/defi/config";
import { getDefaultToken } from "@/stores/defi/polkadot/transfers/utils";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";
import { SubstrateNetworkId } from "@/defi/polkadot/types";

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
