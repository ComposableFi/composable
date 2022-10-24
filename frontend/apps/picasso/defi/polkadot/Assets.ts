import { AssetId, SubstrateNetworkId } from "./types";
import { ParachainId, RelayChainId } from "substrate-react";

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
      picasso: 12884901886, // After creating the asset id via assetRegistry, this value could be anything.
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
  usdc: {
    decimals: 12,
    assetId: "usdc",
    symbol: "USDC",
    icon: "/tokens/usd-coin-usdc.svg",
    name: "USDCoin",
    supportedNetwork: {
      karura: null,
      picasso: 100,
      kusama: null,
    },
  },
  usdt: {
    decimals: 12,
    assetId: "usdt",
    symbol: "USDT",
    icon: "/tokens/usdt.svg",
    name: "USDT",
    supportedNetwork: {
      karura: null,
      picasso: null,
      kusama: null,
    },
  },
};

export const AssetsValidForNow: AssetId[] = ["pica", "kusd", "ksm"];

export const getAsset = (assetId: AssetId): AssetMetadata => Assets[assetId];
export const getAssetById = (
  network: ParachainId | Extract<RelayChainId, "kusama">,
  assetId: number
): AssetMetadata | null => {
  for (const asset in Assets) {
    if (Assets[asset as AssetId].supportedNetwork[network] === assetId) {
      return Assets[asset as AssetId];
    }
  }
  return null;
};

export const getAssetOnChainId = (
  network: ParachainId | Extract<RelayChainId, "kusama">,
  assetId: AssetId | ""
): number | null => {
  if (!assetId) return null;
  return Assets[assetId].supportedNetwork[network];
};

export const getAssetOptions = (noneTokenLabel?: string) => [
  ...(noneTokenLabel
    ? [
        {
          value: "none",
          label: noneTokenLabel,
          icon: undefined,
          disabled: true,
          hidden: true,
        },
      ]
    : []),
  ...Object.values(Assets).map((asset) => ({
    value: asset.assetId,
    label: asset.name,
    shortLabel: asset.symbol,
    icon: asset.icon,
  })),
];
