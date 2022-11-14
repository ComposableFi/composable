import { TRANSFER_ASSET_LIST } from "@/defi/config";

import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";
import { useStore } from "@/stores/root";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";

function extractOptions(
  from: SubstrateNetworkId,
  to: SubstrateNetworkId
): TokenOption[] {
  const list = useStore.getState().substrateTokens.tokens;
  const balances = useStore.getState().substrateBalances.balances;
  return Object.values(list).reduce(
    (previousValue, currentValue: TokenMetadata) => {
      if (
        previousValue.find((value: any) => value.tokenId === currentValue.id)
      ) {
        return previousValue;
      }
      // calculate balance for token
      const balance = balances[from][currentValue.id].balance;

      // only include allowed assets
      if (!TRANSFER_ASSET_LIST[from][to].includes(currentValue.id)) {
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
    (store) => ({
      from: store.transfers.networks.from,
      to: store.transfers.networks.to,
    }),
    ({ from, to }) => {
      const options = extractOptions(from, to);

      setOptions(options);
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => a.from === b.from && a.to === b.to,
    }
  );
};
