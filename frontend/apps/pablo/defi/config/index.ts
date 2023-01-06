export const DEFI_CONFIG = {
  substrateNetworks: ["picasso", "karura", "kusama"] as const,
  networkIds: [1, 137, 42161, 43114, 1285, 250] as const, // important
  ammIds: ["uniswap", "curve", "balancer"] as const,
  swapChartIntervals: [
    {
      symbol: "24h",
      name: "24 hours",
      range: "day",
    },
    {
      symbol: "1w",
      name: "1 week",
      range: "week",
    },
    {
      symbol: "1m",
      name: "1 month",
      range: "month",
    },
    {
      symbol: "1y",
      name: "1 year",
      range: "year",
    },
  ],
  bondChartIntervals: [
    {
      symbol: "24h",
      name: "24 hours",
      range: "day",
    },
    {
      symbol: "1w",
      name: "1 week",
      range: "week",
    },
    {
      symbol: "1m",
      name: "1 month",
      range: "month",
    },
    {
      symbol: "1y",
      name: "1 year",
      range: "year",
    },
  ],
} as const;
