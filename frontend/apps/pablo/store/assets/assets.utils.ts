import produce from "immer";
import { AssetsSlice, MockedAsset } from "./assets.types";

export const setApolloPrice = (
  assets: AssetsSlice["apollo"],
  assetId: string,
  price: string
) => {
  return produce(assets, draft => {
    draft[assetId] = price;
  });
};

export const putSupportedAssets = (
  assets: AssetsSlice["supportedAssets"],
  assetsList: MockedAsset[]
) => {
  return produce(assets, draft => {
    draft = [...assetsList];
  });
};

export const putAssetBalance = (
  assets: AssetsSlice["assetBalances"],
  network: string,
  assetId: string,
  balance: string
) => {
  return produce(assets, draft => {
    if (!assets[network]) {
      draft[network] = {};
    }

    draft[network][assetId] = balance;
  });
};
