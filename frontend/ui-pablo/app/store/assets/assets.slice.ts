import { Assets } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { ParachainNetworks, RelayChainNetworks } from "substrate-react";
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

const EMPTY_BALANCES_MAP: AssetsSlice["assetBalances"] = Object.keys(Assets).reduce((p, c) => {
  let zeroBalance = Object.keys(RelayChainNetworks).concat(Object.keys(ParachainNetworks)).reduce((p, c) => {
    return {
      ...p,
      [c]: "0"
    }
  }, {})
  return {
    ...p,
    [c]: zeroBalance
  }
}, {} as any)

const createAssetsSlice: StoreSlice<AssetsSlice> = (set) => ({
  assets: EMPTY_ASSETS_MAP,
  assetBalances: EMPTY_BALANCES_MAP,
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
      assetBalances: updateBalance(prev.assetBalances, assetId, parachainId, balance),
    })),
});

export default createAssetsSlice;
