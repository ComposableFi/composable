import { getImageURL } from "@/utils/nextImageUrl";
import { DEFI_CONFIG } from "./config";
import { Token, TokenId } from "./types";

export const TOKEN_IDS = DEFI_CONFIG.tokenIds;
export const TOKENS: { [key in TokenId]: Token } = {
  eth: {
    id: "eth",
    icon: getImageURL("/tokens/eth-mainnet.svg"),
    symbol: "ETH",
    name: "Ethereum"
  },
  matic: {
    id: "matic",
    icon: getImageURL("/tokens/polygon-matic.svg"),
    symbol: "MATIC",
    name: "Matic"
  },
  avax: {
    id: "avax",
    icon: getImageURL("/tokens/avalanche.svg"),
    symbol: "AVAX",
    name: "Avalanche"
  },
  weth: {
    id: "weth",
    icon: getImageURL("/tokens/weth-mainnet.svg"),
    symbol: "wETH",
    name: "wEthereum"
  },
  usdc: {
    id: "usdc",
    icon: getImageURL("/tokens/usdc.svg"),
    symbol: "USDC",
    name: "USDC"
  },
  dot: {
    id: "dot",
    icon: getImageURL("/tokens/polkadot.svg"),
    symbol: "DOT",
    name: "Polkadot"
  },
  uni: {
    id: "uni",
    icon: getImageURL("/tokens/uniswap.svg"),
    symbol: "UNI",
    name: "Uniswap"
  },
  ftm: {
    id: "ftm",
    icon: getImageURL("/tokens/fantom.svg"),
    symbol: "FTM",
    name: "Fantom"
  },
  pica: {
    id: "pica",
    icon: getImageURL("/tokens/picasso.svg"),
    symbol: "PICA",
    name: "Picasso"
  },
  movr: {
    id: "movr",
    icon: getImageURL("/tokens/movr.svg"),
    symbol: "MOVR",
    name: "Moonriver"
  },
  ksm: {
    id: "ksm",
    icon: getImageURL("/tokens/dotsama-kusama.svg"),
    symbol: "KSM",
    name: "Kusama"
  },
  pablo: {
    id: "pablo",
    icon: getImageURL("/tokens/pablo.svg"),
    symbol: "PAB",
    name: "Pablo"
  },
  chaos: {
    id: "chaos",
    icon: getImageURL("/tokens/chaos.svg"),
    symbol: "CHAOS",
    name: "Chaos"
  }
};

export const getToken = (tokenId: TokenId): Token => TOKENS[tokenId];

export const getTokenOptions = (noneTokenLabel?: string) => [
  ...(noneTokenLabel
    ? [
      {
        value: "none",
        label: noneTokenLabel,
        icon: undefined,
        disabled: true,
        hidden: true
      }
    ]
    : []),
  ...TOKEN_IDS.map((tokenId) => ({
    value: tokenId,
    label: getToken(tokenId).name,
    shortLabel: getToken(tokenId).symbol,
    icon: getToken(tokenId).icon
  }))
];
