import { ParachainId } from "substrate-react";
import { AssetId } from "./types";

export interface AssetMetadata {
  decimals: number;
  symbol: string;
  assetId: AssetId;
  icon: string;
  name: string;
  supportedNetwork: {
    [networkId in ParachainId]: number | null;
  };
}

export const Assets: {
  [assetId in AssetId]: AssetMetadata;
} = {
  pica: {
    decimals: 12,
    assetId: "pica",
    symbol: "PICA",
    icon: "/tokens/picasso.svg",
    name: "Picasso",
    supportedNetwork: {
      karura: null,
      picasso: 1,
    },
  },
  ksm: {
    decimals: 12,
    assetId: "ksm",
    symbol: "KSM",
    icon: "/networks/kusama.svg",
    name: "Kusama",
    supportedNetwork: {
      karura: null,
      picasso: 4,
    },
  },
  kusd: {
    decimals: 12,
    assetId: "kusd",
    symbol: "KUSD",
    icon: "/tokens/usd-coin-usdc.svg",
    name: "K-USD",
    supportedNetwork: {
      karura: null,
      picasso: 129,
    },
  },
  layr: {
    decimals: 12,
    assetId: "layr",
    symbol: "LAYR",
    icon: "/tokens/pablo.svg",
    name: "LAYER",
    supportedNetwork: {
      karura: null,
      picasso: 2,
    },
  },
  pablo: {
    decimals: 12,
    assetId: "layr",
    symbol: "LAYR",
    icon: "/tokens/pablo.svg",
    name: "LAYER",
    supportedNetwork: {
      karura: null,
      picasso: 201,
    },
  },
};
