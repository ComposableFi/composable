import { DEFI_CONFIG } from "./config";
import { Network, NetworkId } from "./types";

export const NETWORK_IDS = DEFI_CONFIG.networkIds;
export const NETWORKS: {
    [networkId in NetworkId]: Network;
  } = {
    1: {
      name: "Ethereum",
      rpcUrl: process.env.RPC_URL_1!,
      infoPageUrl: "https://etherscan.io/tx/",
      infoPage: "Etherscan",
      logo: "/networks/mainnet.svg",
      backgroundColor: "#364683",
      defaultTokenSymbol: "ETH",
      publicRpcUrl: "",
      nativeToken: "eth",
    },
    42161: {
      name: "Arbitrum",
      rpcUrl: process.env.RPC_URL_42161!,
      infoPageUrl: "https://arbiscan.io/tx/",
      infoPage: "Arbiscan",
      logo: "/networks/arbitrum.svg",
      backgroundColor: "#23A9C7",
      defaultTokenSymbol: "ETH",
      publicRpcUrl: "https://arb1.arbitrum.io/rpc",
      nativeToken: "eth",
    },
    137: {
      name: "Polygon",
      rpcUrl: process.env.RPC_URL_137!,
      infoPageUrl: "https://polygonscan.com/tx/",
      infoPage: "Polygonscan",
      logo: "/networks/polygon.svg",
      backgroundColor: "#8D49FF",
      defaultTokenSymbol: "MATIC",
      publicRpcUrl: "https://rpc-mainnet.maticvigil.com/",
      nativeToken: "matic",
    },
    43114: {
      name: "Avalanche",
      rpcUrl: process.env.RPC_URL_43114!,
      infoPageUrl: "https://cchain.explorer.avax.network/tx/",
      infoPage: "Avax Scan",
      logo: "/networks/avalanche.svg",
      backgroundColor: "#C73738",
      defaultTokenSymbol: "AVAX",
      publicRpcUrl: "https://api.avax.network/ext/bc/C/rpc",
      nativeToken: "avax",
    },
    1285: {
      name: "Moonriver",
      rpcUrl: process.env.RPC_URL_1285!,
      infoPageUrl: "https://blockscout.moonriver.moonbeam.network/tx/",
      infoPage: "Moonriver Blockscout",
      logo: "/networks/moonriver.svg",
      backgroundColor: "#F3B406",
      defaultTokenSymbol: "MOVR",
      publicRpcUrl: "https://rpc.moonriver.moonbeam.network",
      nativeToken: "movr",
    },
    250: {
      name: "Fantom",
      rpcUrl: process.env.RPC_URL_250!,
      infoPageUrl: "https://ftmscan.com/tx/",
      infoPage: "Fantom Scan",
      logo: "/networks/fantom.svg",
      backgroundColor: "#4172CC",
      defaultTokenSymbol: "FTM",
      publicRpcUrl: "https://rpc.ftm.tools",
      nativeToken: "ftm",
    },
  };
  export const getNetwork = (networkId: NetworkId): Network => NETWORKS[networkId];