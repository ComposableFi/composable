import { NetworkId } from "@/constants/types";
import { ParachainId, SubstrateNetworkId } from "shared";
import { TokenId } from "tokens";

const config = {
  governanceUrl: "https://picasso.polkassembly.io/",
  twitterUrl: "https://twitter.com/picasso_network",
  mediumUrl: "https://medium.com/picasso-network",
  discordUrl: "https://discord.gg/composable",
  pabloUrl: "https://app.pablo.finance",
  appName: "Composable Finance Picasso Parachain",
  transfers: {
    statemineKsmTransferFee: 10_000_000_000,
    transferAssetList: {
      karura: {
        picasso: ["ksm", "ausd", "kusd", "kar"],
        kusama: [],
        karura: [],
        statemine: [],
      },
      kusama: {
        picasso: ["ksm"],
        kusama: [],
        karura: [],
        statemine: [],
      },
      picasso: {
        picasso: [],
        kusama: ["ksm"],
        karura: ["ksm", "ausd", "kusd", "kar"],
        statemine: ["usdt"],
      },
      statemine: {
        picasso: ["usdt"],
        kusama: [],
        karura: [],
        statemine: [],
      },
    } as {
      [key in SubstrateNetworkId]: {
        [key in SubstrateNetworkId]: TokenId[];
      };
    },
    picassoSupportedTransfers: [
      "kusama",
      "picasso",
      "statemine",
    ] as SubstrateNetworkId[],
  },
  statemineSubscanUrl: "https://statemine.subscan.io/",
  kusamaSubscanUrl: "https://kusama.subscan.io/",
  picassoSubscanUrl: "https://picasso.subscan.io/",
  karuraSubscanUrl: "https://karura.subscan.io/",
  evm: {
    networkIds: [1, 137, 42161, 43114, 1285, 250] as const,
    tokenIds: [
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
      "xPICA",
      "pablo",
    ] as const,
    ammIds: ["uniswap", "sushiswap", "quickswap"] as const,
    amms: {
      uniswap: {
        id: "uniswap",
        icon: "/tokens/eth-mainnet.svg",
        label: "Uniswap",
      },
      sushiswap: {
        id: "sushiswap",
        icon: "/tokens/weth-mainnet.svg",
        label: "Sushiswap",
      },
      quickswap: {
        id: "quickswap",
        icon: "/tokens/avalanche.svg",
        label: "Quickswap",
      },
    },
    defaultNetworkId: 1,
    networks: {
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
    },
  },
  defaultNetworkId: "picasso" as ParachainId,
};

export default config;

export const getNetwork = (networkId: NetworkId) =>
  config.evm.networks[networkId];
