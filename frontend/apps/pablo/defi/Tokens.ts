import { DEFI_CONFIG } from "./config";
import { Token, TokenId } from "./types";

export const TOKEN_IDS = DEFI_CONFIG.tokenIds;
export const TOKENS: { [key in TokenId]: Token } = {
  eth: {
    id: "eth",
    icon: "/tokens/eth-mainnet.svg",
    symbol: "ETH",
    name: "Ethereum",
  },
  matic: {
    id: "matic",
    icon: "/tokens/polygon-matic.svg",
    symbol: "MATIC",
    name: "Matic",
  },
  avax: {
    id: "avax",
    icon: "/tokens/avalanche.svg",
    symbol: "AVAX",
    name: "Avalanche",
  },
  weth: {
    id: "weth",
    icon: "/tokens/weth-mainnet.svg",
    symbol: "wETH",
    name: "wEthereum",
  },
  usdc: {
    id: "usdc",
    icon: "/tokens/usdc.svg",
    symbol: "USDC",
    name: "USDC",
  },
  dot: {
    id: "dot",
    icon: "/tokens/polkadot.svg",
    symbol: "DOT",
    name: "Polkadot",
  },
  uni: {
    id: "uni",
    icon: "/tokens/uniswap.svg",
    symbol: "UNI",
    name: "Uniswap",
  },
  ftm: {
    id: "ftm",
    icon: "/tokens/fantom.svg",
    symbol: "FTM",
    name: "Fantom",
  },
  pica: {
    id: "pica",
    icon: "/tokens/picasso.svg",
    symbol: "PICA",
    name: "Picasso",
  },
  movr: {
    id: "movr",
    icon: "/tokens/movr.svg",
    symbol: "MOVR",
    name: "Moonriver",
  },
  ksm: {
    id: "ksm",
    icon: "/tokens/dotsama-kusama.svg",
    symbol: "KSM",
    name: "Kusama",
  },
  pablo: {
    id: "pablo",
    icon: "/tokens/pablo.svg",
    symbol: "PAB",
    name: "Pablo",
  },
  chaos: {
    id: "chaos",
    icon: "/tokens/chaos.svg",
    symbol: "CHAOS",
    name: "Chaos",
  },
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
          hidden: true,
        },
      ]
    : []),
  ...TOKEN_IDS.map((tokenId) => ({
    value: tokenId,
    label: getToken(tokenId).name,
    shortLabel: getToken(tokenId).symbol,
    icon: getToken(tokenId).icon,
  })),
];
