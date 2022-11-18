export type Token = {
  id: TokenId;
  icon: string;
  symbol: string;
  decimalsToDisplay: number;
  coingeckoid: string | null;
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
  "pblo",
  "angl",
  "chaos",
  "usdt",
  "kar",
  "ausd",
  "kusd",
  "xpblo"
] as const; // important

export const TOKEN_IDS = tokenIds;
export type TokenId = typeof tokenIds[number];
export const TOKENS: { [key in TokenId]: Token } = {
  eth: {
    id: "eth",
    icon: "/tokens/eth-mainnet.svg",
    symbol: "ETH",
    decimalsToDisplay: 4,
    coingeckoid: "ethereum"
  },
  matic: {
    id: "matic",
    icon: "/tokens/polygon-matic.svg",
    symbol: "MATIC",
    decimalsToDisplay: 4,
    coingeckoid: "matic-network"
  },
  avax: {
    id: "avax",
    icon: "/tokens/avalanche.svg",
    symbol: "AVAX",
    decimalsToDisplay: 4,
    coingeckoid: "avalanche-2"
  },
  weth: {
    id: "weth",
    icon: "/tokens/weth-mainnet.svg",
    symbol: "wETH",
    decimalsToDisplay: 4,
    coingeckoid: "ethereum"
  },
  usdc: {
    id: "usdc",
    icon: "/tokens/usd-coin-usdc.svg",
    symbol: "USDC",
    decimalsToDisplay: 4,
    coingeckoid: "usd-coin"
  },
  dot: {
    id: "dot",
    icon: "/tokens/polkadot.svg",
    symbol: "DOT",
    decimalsToDisplay: 4,
    coingeckoid: "polkadot"
  },
  uni: {
    id: "uni",
    icon: "/tokens/uniswap.svg",
    symbol: "UNI",
    decimalsToDisplay: 4,
    coingeckoid: "uniswap"
  },
  ftm: {
    id: "ftm",
    icon: "/tokens/fantom.svg",
    symbol: "FTM",
    decimalsToDisplay: 4,
    coingeckoid: "fantom"
  },
  pica: {
    id: "pica",
    icon: "/tokens/picasso.svg",
    symbol: "PICA",
    decimalsToDisplay: 4,
    coingeckoid: null
  },
  movr: {
    id: "movr",
    icon: "/tokens/movr.svg",
    symbol: "MOVR",
    decimalsToDisplay: 4,
    coingeckoid: "moonriver"
  },
  ksm: {
    id: "ksm",
    icon: "/tokens/dotsama-kusama.svg",
    symbol: "KSM",
    decimalsToDisplay: 4,
    coingeckoid: "kusama"
  },
  chaos: {
    id: "chaos",
    icon: "/tokens/chaos.svg",
    symbol: "CHAOS",
    decimalsToDisplay: 4,
    coingeckoid: null,
  },
  pblo: {
    id: "pblo",
    icon: "/tokens/pablo.svg",
    symbol: "PBLO",
    decimalsToDisplay: 4,
    coingeckoid: null
  },
  xpblo: {
    id: "xpblo",
    icon: "/tokens/pablo.svg",
    symbol: "XPBLO",
    decimalsToDisplay: 4,
    coingeckoid: null
  },
  angl: {
    id: "angl",
    icon: "/tokens/angular.svg",
    symbol: "ANGL",
    decimalsToDisplay: 4,
    coingeckoid: null
  },
  kar: {
    id: "kar",
    icon: "/tokens/karura.svg",
    symbol: "KAR",
    decimalsToDisplay: 6,
    coingeckoid: "karura"
  },
  ausd: {
    id: "ausd",
    icon: "/tokens/ausd.svg",
    symbol: "AUSD",
    decimalsToDisplay: 4,
    coingeckoid: "acala-dollar-acala"
  },
  kusd: {
    id: "kusd",
    icon: "/tokens/kusd.svg",
    symbol: "KUSD",
    decimalsToDisplay: 4,
    coingeckoid: "acala-dollar"
  },
  usdt: {
    id: "usdt",
    icon: "/tokens/usdt.svg",
    symbol: "USDT",
    decimalsToDisplay: 4,
    coingeckoid: "tether"
  },
};

export const getToken = (tokenId: TokenId): Token =>TOKENS[tokenId];
