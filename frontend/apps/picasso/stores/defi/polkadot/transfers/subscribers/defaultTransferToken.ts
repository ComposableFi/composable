import { useStore } from "@/stores/root";
import { getDefaultToken } from "@/stores/defi/polkadot/transfers/utils";

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
