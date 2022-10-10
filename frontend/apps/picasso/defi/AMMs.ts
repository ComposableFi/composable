import { AMM, AMM_ID } from "./types";
import { DEFI_CONFIG } from "./config";
import { getImageURL } from "@/utils/nextImageUrl";

export const AMM_IDS = DEFI_CONFIG.ammIds;
export const AMMs: { [key in AMM_ID]: AMM } = {
  uniswap: {
    id: "uniswap",
    icon: getImageURL("/tokens/eth-mainnet.svg"),
    label: "Uniswap"
  },
  sushiswap: {
    id: "sushiswap",
    icon: getImageURL("/tokens/weth-mainnet.svg"),
    label: "Sushiswap"
  },
  quickswap: {
    id: "quickswap",
    icon: getImageURL("/tokens/avalanche.svg"),
    label: "Quickswap"
  }
};
export const getAMM = (ammId: AMM_ID): AMM => AMMs[ammId];
