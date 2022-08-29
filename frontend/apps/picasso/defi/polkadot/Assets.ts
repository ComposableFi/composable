import { AssetId, SubstrateNetworkId } from "./types";

export interface AssetMetadata {
  decimals: number;
  symbol: string;
  assetId: AssetId;
  icon: string;
  name: string;
  supportedNetwork: {
    [networkId in SubstrateNetworkId]: number | null;
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
      kusama: null,
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
      kusama: 1,
    },
  },
  kusd: {
    decimals: 12,
    assetId: "kusd",
    symbol: "KUSD",
    icon: "/tokens/usd-coin-usdc.svg",
    name: "K-USD",
    supportedNetwork: {
      karura: 129,
      picasso: 300_000_000_001, // After creating the asset id via assetRegistry, this value could be anything.
      kusama: null,
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
      kusama: null,
    },
  },
  pablo: {
    decimals: 12,
    assetId: "pablo",
    symbol: "PAB",
    icon: "/tokens/pablo.svg",
    name: "PABLO",
    supportedNetwork: {
      karura: null,
      picasso: 2,
      kusama: null,
    },
  },
  ausd: {
    decimals: 12,
    assetId: "ausd",
    symbol: "AUSD",
    icon: "/tokens/ausd.svg",
    name: "Acala USD",
    supportedNetwork: {
      karura: 2,
      picasso: null,
      kusama: null,
    },
  },
  kar: {
    decimals: 12,
    assetId: "kar",
    symbol: "KAR",
    icon: "/tokens/karura.svg",
    name: "Karura",
    supportedNetwork: {
      karura: 1,
      picasso: null,
      kusama: null,
    },
  },
};
