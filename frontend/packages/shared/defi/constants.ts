import BigNumber from "bignumber.js";
import { stringToU8a } from "@polkadot/util";
import { SubstrateNetwork, SubstrateNetworkId } from "..";
import { getEnvironment } from "../endpoints";

export const PERMILL_UNIT = new BigNumber(1_000_000);
export const PERBILL_UNIT = new BigNumber(1_000_000_000);
export const PALLET_ID = "modl";
export const PALLET_TYPE_ID = stringToU8a(PALLET_ID);
export const FEE_MULTIPLIER = 1.5;
export const DESTINATION_FEE_MULTIPLIER = 5;

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
    subscanUrl: "https://kusama.subscan.io/",
    decimals: 12,
    symbol: "KSM",
    logo: "/networks/kusama.svg",
  },
  picasso: {
    relayChain: "kusama",
    parachainId: 2087,
    name: "Picasso",
    wsUrl: getEnvironment("picasso"),
    tokenId: "pica",
    ss58Format: 49,
    subscanUrl: "https://picasso.subscan.io/",
    decimals: 12,
    symbol: "PICA",
    logo: "/networks/picasso.svg",
  },
  karura: {
    relayChain: "kusama",
    parachainId: 2000,
    name: "Karura",
    wsUrl: getEnvironment("karura"),
    tokenId: "kar",
    ss58Format: 8,
    subscanUrl: "https://karura.subscan.io/",
    decimals: 12,
    symbol: "KAR",
    logo: "/networks/karura.svg",
  },
  statemine: {
    relayChain: "kusama",
    parachainId: 1000,
    name: "Statemine",
    wsUrl: getEnvironment("statemine"),
    tokenId: "ksm",
    ss58Format: 2,
    subscanUrl: "https://statemine.subscan.io/",
    symbol: "KSM",
    logo: "/networks/statemine.svg",
    decimals: 12,
  },
};
