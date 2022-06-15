import { ParachainId } from "substrate-react/dist/dotsama/types";
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
    assetId: "pablo",
    symbol: "PAB",
    icon: "/tokens/pablo.svg",
    name: "PABLO",
    supportedNetwork: {
      karura: null,
      picasso: 2,
    },
  },
};

export const AssetsValidForNow: AssetId[] = ["pica", "kusd", "ksm"];

export const getAsset = (assetId: AssetId): AssetMetadata => Assets[assetId];
export const getAssetById = (
  network: ParachainId,
  assetId: number
): AssetMetadata | null => {
  for (const asset in Assets) {
    if (
      Assets[asset as AssetId].supportedNetwork[network as ParachainId] ===
      assetId
    ) {
      return Assets[asset as AssetId];
    }
  }
  return null;
};

export const getAssetOnChainId = (
  network: ParachainId,
  assetId: AssetId
): number | null => {
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
