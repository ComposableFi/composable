import { TRANSFER_ASSET_LIST } from "@/defi/config";

import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";
import { useStore } from "@/stores/root";

function extractOptions(from: SubstrateNetworkId): TokenOption[] {
  const list = useStore.getState().substrateTokens.tokens;
  const balances = useStore.getState().substrateBalances.balances;
  return Object.values(list).reduce((previousValue, currentValue) => {
    if (
      previousValue.find((value: any) => value.tokenId === currentValue.symbol)
    ) {
      return previousValue;
    }
    // calculate balance for token
    const balance = balances[from][currentValue.id].balance;

    // only include allowed assets
    if (
      !TRANSFER_ASSET_LIST[from].includes(currentValue.symbol.toLowerCase())
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
  }, [] as TokenOption[]);
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
