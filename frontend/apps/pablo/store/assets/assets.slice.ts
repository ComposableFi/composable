import { StoreSlice } from "../types";
import { AssetsSlice, MockedAsset } from "./assets.types";
import { putAssetBalance, setApolloPrice } from "./assets.utils";
import { getImageURL } from "@/utils/nextImageUrl";

export const SUPPORTED_ASSETS: MockedAsset[] = [
  {
    decimals: 12,
    symbol: "PICA",
    icon: getImageURL("/tokens/pica.svg"),
    name: "Picasso",
    network: {
      karura: "none",
      picasso: "1"
    }
  },
  {
    decimals: 12,
    symbol: "KSM",
    icon: getImageURL("/tokens/ksm.svg"),
    name: "Kusama",
    network: {
      karura: "none",
      picasso: "4"
    }
  },
  {
    decimals: 12,
    symbol: "KUSD",
    icon: getImageURL("/tokens/usdc.svg"),
    name: "K-USD",
    network: {
      karura: "none",
      picasso: "129"
    }
  },
  {
    decimals: 12,
    symbol: "PBLO",
    icon: getImageURL("/tokens/pblo.svg"),
    name: "Pablo",
    network: {
      karura: "none",
      picasso: "5"
    }
  },
  {
    decimals: 12,
    symbol: "USDC",
    icon: getImageURL("/tokens/usdc.svg"),
    name: "USD Coin",
    network: {
      karura: "none",
      picasso: "131"
    }
  },
  {
    decimals: 12,
    symbol: "USDT",
    icon: getImageURL("/tokens/usdt.svg"),
    name: "USD Tether",
    network: {
      karura: "none",
      picasso: "130"
    }
  }
];

const createAssetsSlice: StoreSlice<AssetsSlice> = (set) => ({
  supportedAssets: SUPPORTED_ASSETS,
  assetBalances: {},
  apollo: {},
  updateApolloPrice: (assetId: string, price: string) =>
    set((prev: AssetsSlice) => ({
      apollo: setApolloPrice(prev.apollo, assetId, price)
    })),
  putAssetBalance: (networkId, assetId, balance) => set((prevSlice: AssetsSlice) => ({
    assetBalances: putAssetBalance(prevSlice.assetBalances, networkId, assetId, balance)
  }))
});

export default createAssetsSlice;
