import { getEnvironment } from "shared/endpoints";
import { SubstrateNetwork, SubstrateNetworkId } from "./types";

export const SUBSTRATE_NETWORKS: {
  [substrateNetworkId in SubstrateNetworkId]: SubstrateNetwork;
} = {
  kusama: {
    relayChain: "kusama",
    parachainId: 0,
    name: "Kusama",
    wsUrl: getEnvironment("kusama"),
    tokenId: "ksm",
    ss58Format: 2,
    subscanUrl: "",
    decimals: 12,
    symbol: "KSM",
    logo: "/networks/kusama.svg"
  },
  picasso: {
    relayChain: "kusama",
    parachainId: 2087,
    name: "Picasso",
    wsUrl: getEnvironment("picasso"),
    tokenId: "pica",
    ss58Format: 49,
    subscanUrl: "",
    decimals: 12,
    symbol: "PICA",
    logo: "/networks/picasso.svg"
  },
  karura: {
    relayChain: "kusama",
    parachainId: 2000,
    name: "Karura",
    wsUrl: getEnvironment("karura"),
    tokenId: "kar",
    ss58Format: 8,
    subscanUrl: "",
    decimals: 12,
    symbol: "KAR",
    logo: "/networks/karura.svg"
  },
  statemine: {
    relayChain: "kusama",
    parachainId: 1000,
    name: "Statemine",
    wsUrl: getEnvironment("statemine"),
    tokenId: "ksm",
    ss58Format: 2,
    subscanUrl: "",
    symbol: "KSM",
    logo: "/networks/statemine.svg",
    decimals: 12
  }
};
export const SUBSTRATE_NETWORK_IDS: Array<SubstrateNetworkId> = Object.keys(
  SUBSTRATE_NETWORKS
) as Array<SubstrateNetworkId>;

export const getSubstrateNetwork = (
  networkId: SubstrateNetworkId
): SubstrateNetwork => SUBSTRATE_NETWORKS[networkId];
