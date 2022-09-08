import { SubstrateNetwork, SubstrateNetworkId } from "./types";

export const SUBSTRATE_NETWORKS: {
  [substrateNetworkId in SubstrateNetworkId]: SubstrateNetwork;
} = {
  kusama: {
    relayChain: "kusama",
    parachainId: 0,
    name: "Kusama",
    wsUrl: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA!,
    tokenId: "ksm",
    ss58Format: 2,
    subscanUrl: "",
    decimals: 12,
    symbol: "KSM",
    logo: "/networks/kusama.svg",
  },
  picasso: {
    relayChain: "kusama",
    parachainId: 2087,
    name: "Picasso",
    wsUrl: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019!,
    tokenId: "pica",
    ss58Format: 49,
    subscanUrl: "",
    decimals: 12,
    symbol: "PICA",
    logo: "/networks/picasso.svg",
  },
  karura: {
    relayChain: "kusama",
    parachainId: 2000,
    name: "Karura",
    wsUrl: process.env.SUBSTRATE_PROVIDER_URL_KARURA!,
    tokenId: "kar",
    ss58Format: 49,
    subscanUrl: "",
    decimals: 18,
    symbol: "KAR",
    logo: "/networks/karura.svg",
  },
};
export const SUBSTRATE_NETWORK_IDS: Array<SubstrateNetworkId> = Object.keys(
  SUBSTRATE_NETWORKS
) as Array<SubstrateNetworkId>;

export const getSubstrateNetwork = (
  networkId: SubstrateNetworkId
): SubstrateNetwork => SUBSTRATE_NETWORKS[networkId];
