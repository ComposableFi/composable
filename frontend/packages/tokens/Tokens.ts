import { getImageURL } from "picasso/utils/nextImageUrl";

export type TokenId = typeof tokenIds[number];
export type Token = {
  id: TokenId;
  icon: string;
  symbol: string;
  decimalsToDisplay: number;
};

const tokenIds = [
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
  "pablo",
  "angl",
  "chaos",
  "usdt"
]; // important

export const TOKEN_IDS = tokenIds;
export const TOKENS: { [key in TokenId]: Token } = {
  eth: {
    id: "eth",
    icon: getImageURL("/tokens/eth-mainnet.svg"),
    symbol: "ETH",
    decimalsToDisplay: 4
  },
  matic: {
    id: "matic",
    icon: getImageURL("/tokens/polygon-matic.svg"),
    symbol: "MATIC",
    decimalsToDisplay: 4
  },
  avax: {
    id: "avax",
    icon: getImageURL("/tokens/avalanche.svg"),
    symbol: "AVAX",
    decimalsToDisplay: 4
  },
  weth: {
    id: "weth",
    icon: getImageURL("/tokens/weth-mainnet.svg"),
    symbol: "wETH",
    decimalsToDisplay: 4
  },
  usdc: {
    id: "usdc",
    icon: getImageURL("/tokens/usd-coin-usdc.svg"),
    symbol: "USDC",
    decimalsToDisplay: 4
  },
  dot: {
    id: "dot",
    icon: getImageURL("/tokens/polkadot.svg"),
    symbol: "DOT",
    decimalsToDisplay: 4
  },
  uni: {
    id: "uni",
    icon: getImageURL("/tokens/uniswap.svg"),
    symbol: "UNI",
    decimalsToDisplay: 4
  },
  ftm: {
    id: "ftm",
    icon: getImageURL("/tokens/fantom.svg"),
    symbol: "FTM",
    decimalsToDisplay: 4
  },
  pica: {
    id: "pica",
    icon: getImageURL("/tokens/picasso.svg"),
    symbol: "PICA",
    decimalsToDisplay: 4
  },
  movr: {
    id: "movr",
    icon: getImageURL("/tokens/movr.svg"),
    symbol: "MOVR",
    decimalsToDisplay: 4
  },
  ksm: {
    id: "ksm",
    icon: getImageURL("/tokens/dotsama-kusama.svg"),
    symbol: "KSM",
    decimalsToDisplay: 4
  },
  chaos: {
    id: "chaos",
    icon: getImageURL("/tokens/chaos.svg"),
    symbol: "CHAOS",
    decimalsToDisplay: 4
  },
  pablo: {
    id: "pablo",
    icon: getImageURL("/tokens/pablo.svg"),
    symbol: "PABLO",
    decimalsToDisplay: 4
  },
  angl: {
    id: "angl",
    icon: getImageURL("/tokens/angular.svg"),
    symbol: "ANGL",
    decimalsToDisplay: 4
  },
  kar: {
    id: "kar",
    icon: getImageURL("/tokens/karura.svg"),
    symbol: "KAR",
    decimalsToDisplay: 6
  },
  ausd: {
    id: "ausd",
    icon: getImageURL("/tokens/ausd.svg"),
    symbol: "AUSD",
    decimalsToDisplay: 4
  },
  kusd: {
    id: "kusd",
    icon: getImageURL("/tokens/kusd.svg"),
    symbol: "KUSD",
    decimalsToDisplay: 4
  },
  usdt: {
    id: "usdt",
    icon: getImageURL("/tokens/usdt.svg"),
    symbol: "USDT",
    decimalsToDisplay: 4
  }
};
export const getToken = (tokenId: TokenId): Token => TOKENS[tokenId];
