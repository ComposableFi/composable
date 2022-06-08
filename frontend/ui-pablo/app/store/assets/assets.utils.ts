import { AssetId } from "@/defi/polkadot/types";
import produce from "immer";
import { ParachainId } from "substrate-react/dist/dotsama/types";
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
  tokens: AssetsSlice["assets"],
  assetId: AssetId,
  parachainId: ParachainId,
  balance: string
) => {
  return produce(tokens, (draft) => {
    if (draft[assetId]) {
      draft[assetId].balance[parachainId] ??= "0";
      draft[assetId].balance[parachainId] = balance;
  }
  })
}