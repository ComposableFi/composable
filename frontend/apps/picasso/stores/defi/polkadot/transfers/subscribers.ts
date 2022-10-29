import { TRANSFER_ASSET_LIST } from "@/defi/config";

import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";
import { getDefaultToken } from "@/stores/defi/polkadot/transfers/utils";
import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";
import { getSubstrateNetworkAssetIdentifierKey } from "@/defi/polkadot/pallets/Assets";

function extractOptions(from: SubstrateNetworkId): TokenOption[] {
  const list = useStore.getState().substrateTokens.tokens;
  const balances = useStore.getState().substrateBalances.balances;
  const options = Object.values(list).reduce(
    (previousValue, currentValue) => {
      // no duplicates
      if (
        previousValue.find(
          (value: any) => value.tokenId === currentValue.symbol
        )
      ) {
        return previousValue;
      }

      // calculate balance for token
      const balance = balances[from][currentValue.id].balance

      // only include allowed assets
      if (
        !TRANSFER_ASSET_LIST[from].includes(
          currentValue.symbol.toLowerCase()
        )
      ) {
        return previousValue;
      }

      return [
        ...previousValue,
        {
          tokenId: currentValue.id,
          symbol: currentValue.symbol,
          icon: currentValue.icon,
          disabled: balance.lte(0),
          balance: balance,
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
    async (feeItem) => {
      const network = getSubstrateNetworkAssetIdentifierKey("picasso");
      const assetId = useStore.getState().substrateTokens.tokens[feeItem][network];

      if (!assetId) {
        return;
      }

      const ed = await api.query.currencyFactory.assetEd(assetId);

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
