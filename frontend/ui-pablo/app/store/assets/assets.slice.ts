import { Assets } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { ParachainNetworks } from "substrate-react";
import { ParachainId } from "substrate-react/dist/dotsama/types";
import { StoreSlice } from "../types";
import { AssetsSlice } from "./assets.types";
import { updateAssetPrice, updateBalance } from "./assets.utils";

const EMPTY_ASSETS_MAP: AssetsSlice["assets"] = Object.entries(Assets)
  .map(([assetId, metadata]) => {
    return {
      assetId: assetId,
      price: 1,
      decimals: metadata.decimals,
      balance: Object.keys(ParachainNetworks).reduce((acc, curr) => {
        return { ...acc, [curr]: "0" };
      }, {}),
      symbol: metadata.symbol,
      icon: metadata.icon,
    };
  })
  .reduce((prev, curr) => {
    return {
      ...prev,
      [curr.assetId]: curr,
    };
  }, {} as AssetsSlice["assets"]);

const createAssetsSlice: StoreSlice<AssetsSlice> = (set) => ({
  assets: EMPTY_ASSETS_MAP,
  updateAssetPrice: (assetId: AssetId, price: number) =>
    set((prev: AssetsSlice) => ({
      assets: updateAssetPrice(prev.assets, assetId, price),
    })),
  updateAssetBalance: (
    assetId: AssetId,
    parachainId: ParachainId,
    balance: string
  ) =>
    set((prev: AssetsSlice) => ({
      assets: updateBalance(prev.assets, assetId, parachainId, balance),
    })),
});

export default createAssetsSlice;
