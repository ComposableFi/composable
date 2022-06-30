import { Assets } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { StoreSlice } from "../types";
import { AssetsSlice } from "./assets.types";
import { ParachainId } from "substrate-react/dist/dotsama/types";
import { ParachainNetworks, RelayChainNetworks } from "substrate-react";
import { setApolloPrice, updateBalance } from "./assets.utils";

const EMPTY_ASSETS_MAP: AssetsSlice["assets"] = Object.entries(Assets)
  .map(([assetId, metadata]) => {
    return {
      assetId: assetId,
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

const EMPTY_BALANCES_MAP: AssetsSlice["balances"] = Object.keys(Assets).reduce((p, c) => {
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
  balances: EMPTY_BALANCES_MAP,
  apollo: {},
  updateApolloPrice: (assetId: string, price: string) =>
    set((prev: AssetsSlice) => ({
      apollo: setApolloPrice(prev.apollo, assetId, price),
    })),
  updateAssetBalance: (
    assetId: AssetId,
    parachainId: ParachainId,
    balance: string
  ) =>
    set((prev: AssetsSlice) => ({
      balances: updateBalance(prev.balances, assetId, parachainId, balance),
    })),
});

export default createAssetsSlice;
