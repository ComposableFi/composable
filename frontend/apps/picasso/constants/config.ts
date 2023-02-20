import { NetworkId } from "@/constants/types";
import { ParachainId, SubstrateNetworkId } from "shared";
import { TokenId } from "tokens";
import BigNumber from "bignumber.js";

export const getNetwork = (networkId: NetworkId) =>
  config.evm.networks[networkId];

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
  stakingRewards: {
    demoMode: false,
    durationPresetOptions: [
      {
        label: "No lock date",
        value: 0,
      },
      {
        label: "2 weeks",
        value: 604800 * 2,
      },
      {
        label: "1 month",
        value: 604800 * 4,
      },
      {
        label: "1 month and 2 weeks",
        value: 604800 * 6,
      },
      {
        label: "2 months",
        value: 604800 * 8,
      },
      {
        label: "2 months and 2 weeks",
        value: 604800 * 10,
      },
      {
        label: "3 months",
        value: 604800 * 12,
      },
      {
        label: "3 months and 2 weeks",
        value: 604800 * 14,
      },
      {
        label: "4 months",
        value: 604800 * 16,
      },
      {
        label: "4 months and 2 weeks",
        value: 604800 * 18,
      },
      {
        label: "5 months",
        value: 604800 * 20,
      },
      {
        label: "5 months and 2 weeks",
        value: 604800 * 22,
      },
      {
        label: "6 months",
        value: 604800 * 24,
      },
    ],
    picaRewardPools: {
      owner: "",
      assetId: "1",
      totalShares: new BigNumber(10000000),
      endBlock: new BigNumber(10000),
      lock: {
        durationPresets: {
          "0": "1000000000",
          "1209600": "1100000000",
          "2419200": "1200000000",
          "3628800": "1250000000",
          "4838400": "1300000000",
          "6048000": "1350000000",
          "7257600": "1400000000",
          "8467200": "1500000000",
          "9676800": "1600000000",
          "10886400": "1700000000",
          "12096000": "1800000000",
          "13305600": "1900000000",
          "14515200": "2000000000",
        },
        unlockPenalty: "500000000",
      },
      shareAssetId: "1001",
      financialNftAssetId: "2001",
      minimumStakingAmount: new BigNumber(10),
    },
    picaPortfolios: [
      {
        assetId: "1",
        instanceId: "1",
        shareAssetId: "1001",
        collectionId: "2001",
        endTimestamp: "1675348134561",
        id: "3",
        multiplier: new BigNumber(0.01),
        share: new BigNumber(1234),
        stake: new BigNumber(1231),
        unlockPenalty: new BigNumber(0.5),
      },
      {
        assetId: "1",
        instanceId: "123",
        shareAssetId: "1001",
        collectionId: "2001",
        endTimestamp: "1695348134561",
        id: "132",
        multiplier: new BigNumber(0.01),
        share: new BigNumber(4321),
        stake: new BigNumber(12231),
        unlockPenalty: new BigNumber(0.5),
      },
    ],
  },
};

export default config;
