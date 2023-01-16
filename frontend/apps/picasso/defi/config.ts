import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenId } from "tokens";

export const DEFI_CONFIG = {
  networkIds: [1, 137, 42161, 43114, 1285, 250] as const, // important
  tokenIds: [
    "eth",
    "matic",
    "avax",
    "weth",
    "usdc",
    "dot",
    "uni",
    "ftm",
    "pica",
    "movr",
    "ksm",
    "xPICA",
    "pablo",
  ] as const, // important
  ammIds: ["uniswap", "sushiswap", "quickswap"] as const,
};

export type AllowedTransferList = {
  [key in SubstrateNetworkId]: Record<SubstrateNetworkId, Array<TokenId>>;
};

export const PICASSO_SUPPORTED_TRANSFERS: Array<SubstrateNetworkId> = [
  "kusama",
  "picasso",
  "statemine",
];

export const TRANSFER_ASSET_LIST: AllowedTransferList = {
  karura: {
    picasso: ["ksm", "ausd", "kusd", "kar"],
    kusama: [],
    karura: [],
    statemine: [],
  },
  kusama: {
    picasso: ["ksm"],
    kusama: [],
    karura: [],
    statemine: [],
  },
  picasso: {
    picasso: [],
    kusama: ["ksm"],
    karura: ["ksm", "ausd", "kusd", "kar"],
    statemine: ["usdt"],
  },
  statemine: {
    picasso: ["usdt"],
    kusama: [],
    karura: [],
    statemine: [],
  },
};

export const STATEMINE_SUBSCAN_URL = "https://statemine.subscan.io/";
export const KUSAMA_SUBSCAN_URL = "https://kusama.subscan.io/";
export const PICASSO_SUBSCAN_URL = "https://picasso.subscan.io/";
export const KARURA_SUBSCAN_URL = "https://karura.subscan.io/";

export const PICASSO_STATEMINE_KSM_TRANSFER_FEE = 10_000_000_000;
