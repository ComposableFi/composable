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
  "usdt",
]; // important

export const TOKEN_IDS = tokenIds;
export const TOKENS: { [key in TokenId]: Token } = {
  eth: {
    id: "eth",
    icon: "/tokens/eth-mainnet.svg",
    symbol: "ETH",
    decimalsToDisplay: 4,
  },
  matic: {
    id: "matic",
    icon: "/tokens/polygon-matic.svg",
    symbol: "MATIC",
    decimalsToDisplay: 4,
  },
  avax: {
    id: "avax",
    icon: "/tokens/avalanche.svg",
    symbol: "AVAX",
    decimalsToDisplay: 4,
  },
  weth: {
    id: "weth",
    icon: "/tokens/weth-mainnet.svg",
    symbol: "wETH",
    decimalsToDisplay: 4,
  },
  usdc: {
    id: "usdc",
    icon: "/tokens/usd-coin-usdc.svg",
    symbol: "USDC",
    decimalsToDisplay: 4,
  },
  dot: {
    id: "dot",
    icon: "/tokens/polkadot.svg",
    symbol: "DOT",
    decimalsToDisplay: 4,
  },
  uni: {
    id: "uni",
    icon: "/tokens/uniswap.svg",
    symbol: "UNI",
    decimalsToDisplay: 4,
  },
  ftm: {
    id: "ftm",
    icon: "/tokens/fantom.svg",
    symbol: "FTM",
    decimalsToDisplay: 4,
  },
  pica: {
    id: "pica",
    icon: "/tokens/picasso.svg",
    symbol: "PICA",
    decimalsToDisplay: 4,
  },
  movr: {
    id: "movr",
    icon: "/tokens/movr.svg",
    symbol: "MOVR",
    decimalsToDisplay: 4,
  },
  ksm: {
    id: "ksm",
    icon: "/tokens/dotsama-kusama.svg",
    symbol: "KSM",
    decimalsToDisplay: 4,
  },
  chaos: {
    id: "chaos",
    icon: "/tokens/chaos.svg",
    symbol: "CHAOS",
    decimalsToDisplay: 4,
  },
  pablo: {
    id: "pablo",
    icon: "/tokens/pablo.svg",
    symbol: "PABLO",
    decimalsToDisplay: 4,
  },
  angl: {
    id: "angl",
    icon: "/tokens/angular.svg",
    symbol: "ANGL",
    decimalsToDisplay: 4,
  },
  kar: {
    id: "kar",
    icon: "/tokens/karura.svg",
    symbol: "KAR",
    decimalsToDisplay: 6,
  },
  ausd: {
    id: "ausd",
    icon: "/tokens/ausd.svg",
    symbol: "AUSD",
    decimalsToDisplay: 4,
  },
  kusd: {
    id: "kusd",
    icon: "/tokens/kusd.svg",
    symbol: "KUSD",
    decimalsToDisplay: 4,
  },
  usdt: {
    id: "usdt",
    icon: "/tokens/usdt.svg",
    symbol: "USDT",
    decimalsToDisplay: 4,
  },
};
export const getToken = (tokenId: TokenId): Token =>
  TOKENS[tokenId.toLowerCase()];
