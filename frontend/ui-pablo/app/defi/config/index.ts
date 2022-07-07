export const DEFI_CONFIG = {
    networkIds: [1, 137, 42161, 43114, 1285, 250] as const, // important
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
      "pablo",
      "chaos",
    ] as const, // important
    ammIds: ["uniswap", "curve", "balancer"] as const,
    swapChartIntervals: [
      {
        symbol: "24h",
        name: "24 hours",
      },
      {
        symbol: "1w",
        name: "1 week",
      },
      {
        symbol: "1m",
        name: "1 month",
      },
      // {
      //   symbol: "1y",
      //   name: "1 year",
      // },
    ],
    bondChartIntervals: [
      {
        symbol: "24h",
        name: "24 hours",
      },
      {
        symbol: "1w",
        name: "1 week",
      },
      {
        symbol: "1m",
        name: "1 month",
      },
      {
        symbol: "1y",
        name: "1 year",
      },
    ],
  };
  