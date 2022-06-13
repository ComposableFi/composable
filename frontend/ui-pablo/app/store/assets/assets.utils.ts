import { AssetId } from "@/defi/polkadot/types";
import produce from "immer";
import { ParachainId, RelayChainId } from "substrate-react/dist/dotsama/types";
import { AssetsSlice } from "./assets.types";

export const updateAssetPrice = (
  tokens: AssetsSlice["assets"],
  assetId: AssetId,
  price: number
) => {
  return produce(tokens, (draft) => {
    if (draft[assetId]) {
        draft[assetId].price = price
    }
    });
};

export const updateBalance = (
  tokens: AssetsSlice["assetBalances"],
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