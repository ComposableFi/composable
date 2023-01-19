import {
  ParachainId,
  ParachainNetwork,
  RelayChainId,
  RelaychainNetwork,
} from "./types";

export type ParachainNetworks = {
  [parachainId in ParachainId]: ParachainNetwork;
};
export const ParachainNetworks: ParachainNetworks = {
  statemine: {
    name: "Statemine",
    wsUrl: "rpc=wss://statemine.public.curie.radiumblock.xyz/ws",
    tokenId: "ksm",
    prefix: 2,
    accountType: "*25519",
    subscanUrl: "",
    decimals: 12,
    color: "#113911",
    symbol: "KSM",
    logo: "https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/statemine/logo.svg",
    parachainId: 1000,
    relayChain: "kusama",
  },
  picasso: {
    name: "Picasso",
    wsUrl: "wss://picasso-rpc.composable.finance",
    tokenId: "pica",
    prefix: 49,
    accountType: "*25519",
    subscanUrl: "",
    decimals: 12,
    color: "#B09A9F",
    symbol: "PICA",
    logo: "https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/picasso/logo.svg",
    parachainId: 2087,
    relayChain: "kusama",
  },
  karura: {
    name: "Karura",
    wsUrl: "wss://karura-rpc-0.aca-api.network",
    tokenId: "kar",
    prefix: 8,
    accountType: "*25519",
    subscanUrl: "https://karura.subscan.io/",
    decimals: 12,
    color: "#ff4c3b",
    symbol: "KAR",
    logo: "https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/karura/logo.svg",
    parachainId: 2000,
    relayChain: "kusama",
  },
};

export const RelayChainNetworks: {
  [relaychainId in RelayChainId]: RelaychainNetwork;
} = {
  kusama: {
    name: "Kusama",
    color: "#000000",
    prefix: 2,
    logo: "https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/kusama/logo.svg",
    networkId: "kusama",
    accountType: "*25519",
    wsUrl: "wss://kusama-rpc.polkadot.io",
    subscanUrl: "https://kusama.subscan.io/",
    decimals: 12,
    tokenId: "ksm",
    symbol: "KSM",
  },
  polkadot: {
    name: "Polkadot",
    color: "#e6007a",
    prefix: 0,
    logo: "https://raw.githubusercontent.com/TalismanSociety/chaindata/2778d4b989407a2e9fca6ae897fe849561f74afe/assets/polkadot/logo.svg",
    networkId: "polkadot",
    accountType: "*25519",
    wsUrl: "wss://rpc.polkadot.io",
    subscanUrl: "https://polkadot.subscan.io/",
    decimals: 10,
    tokenId: "dot",
    symbol: "DOT",
  },
};

export const getParachainNetwork = (
  parachainId: ParachainId
): ParachainNetwork => ParachainNetworks[parachainId];
export const getRelaychainNetwork = (
  relaychainId: RelayChainId
): RelaychainNetwork => RelayChainNetworks[relaychainId];
