import { AssetId } from "@/defi/polkadot/types";
import produce from "immer";
import { ParachainId, RelayChainId } from "substrate-react/dist/dotsama/types";
import { AssetsSlice } from "./assets.types";

export const updateBalance = (
  tokens: AssetsSlice["balances"],
  assetId: AssetId,
  chainId: ParachainId | RelayChainId,
  balance: string
) => {
  return produce(tokens, (draft) => {
    if (draft[assetId] && draft[assetId][chainId]) {
      draft[assetId][chainId] = balance;
    }
  })
}

export const setApolloPrice = (
  assets: AssetsSlice["apollo"],
  assetId: string,
  price: string
) => {
  return produce(assets, (draft) => {
    if (draft[assetId]) {
      draft[assetId] = price
    }
  });
};
