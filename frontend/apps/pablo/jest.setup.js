// Optional: configure or set up a testing framework before each test.
// If you delete this file, remove `setupFilesAfterEnv` from `jest.config.js`

// Used for __tests__/testing-library.js
// Learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom/extend-expect'
import { loadEnvConfig } from '@next/env'
import { setGlobalConfig } from '@storybook/testing-react';
import * as globalStorybookConfig from 'pablo-storybook/.storybook/preview';
setGlobalConfig(globalStorybookConfig);

const loadEnvironments = () => loadEnvConfig(process.cwd())

loadEnvironments();

jest.mock("notistack", () => ({
  useSnackbar: jest.fn().mockImplementation(() => ({
    closeSnackbar: jest.fn(),
    enqueueSnackbar: jest.fn(),
  })),
}));

jest.mock('react-apexcharts', () => {
  return {
    __esModule: true,
    default: () => {
      return <div />
    },
  }
})

jest.mock("@/defi/hooks/useLiquidityPoolDetails", () => {
  const BigNumber = require('bignumber.js');
  return { useLiquidityPoolDetails: jest.fn().mockImplementation(() => ({
    poolStats: {
      totalVolume: "0",
      totalValueLocked: "0",
      apr: "0",
      _24HrFee: "0",
      _24HrVolume: "0",
      _24HrTransactionCount: 0,
      dailyRewards: [],
      _24HrFeeValue: "0",
      _24HrVolumeValue: "0",
      totalVolumeValue: "0",
    },
    baseAsset: undefined,
    quoteAsset: undefined,
    pool: undefined,
    tokensLocked: {
      tokenAmounts: {
          baseAmount: new BigNumber(0),
          quoteAmount: new BigNumber(0),
      },
      value: {
          baseValue: new BigNumber(0),
          quoteValue: new BigNumber(0),
          totalValueLocked: new BigNumber(0),
      },
    },
    lpBalance: new BigNumber(0)
  }))}
})

jest.mock("@/defi/hooks/overview/usePabloHistoricalTotalValueLocked", () => {
  return { usePabloHistoricalTotalValueLocked: jest.fn().mockImplementation(() => ({
    chartSeries: [],
    selectedInterval: "24h",
    setSelectedInterval: jest.fn().mockImplementation(() => {}),
    durationLabels: []
  }))}
})


jest.mock("@/defi/hooks/useUserProvidedLiquidityByPool.ts", () => {
  const BigNumber = require('bignumber.js');
  return { useUserProvidedLiquidityByPool: jest.fn().mockImplementation(() => ({
      tokenAmounts: {
        baseAmount: new BigNumber(0),
        quoteAmount: new BigNumber(0)
      },
      value: {
          baseValue: new BigNumber(0),
          quoteValue: new BigNumber(0)
      },

  }))}
})

jest.mock("@/defi/subsquid/bonds/helpers", () => {
  return { fetchTotalPurchasedBondsByOfferIds: jest.fn().mockImplementation(() => (Promise.resolve({})))}
})

jest.mock("@/defi/subsquid/auctions/helpers", () => {
  const BigNumber = require('bignumber.js');
  return { 
    fetchInitialBalance: jest.fn().mockImplementation(() => (Promise.resolve({
    baseBalance: new BigNumber(0), quoteBalance: new BigNumber(0)
  }))),
  fetchAuctionTrades: jest.fn().mockImplementation(() => (Promise.resolve([]))),
  fetchLbpStats: jest.fn().mockImplementation(() => (Promise.resolve({
    totalLiquidity: new BigNumber(0), totalVolume: new BigNumber(0)
  })))
}
})

jest.mock("@/defi/hooks/usePoolTvlChart", () => ({
  usePoolTvlChart: jest.fn().mockImplementation(() => ({
    seriesIntervals: [],
    chartSeries: [],
    selectedInterval: "24h",
    setSelectedInterval: jest.fn()
  }))
}))

jest.mock("@/defi/subsquid/stakingRewards/helpers", () => {
  return {
    fetchSubsquid: jest.fn().mockImplementation(() => Promise.resolve({
      pabloOverviewStats: {
        averageLockMultiplier: 1,
        averageLockTime: 1,
        totalValueLocked: "1"
      },
      stakingPositions: [],
      pabloTransactions: []
    }))
  }
})

jest.mock("@/defi/subsquid/bonds/helpers", () => ({
  fetchTotalPurchasedBondsByOfferIds: jest.fn().mockImplementation(() => (Promise.resolve({})))
}))

jest.mock("@/defi/utils/pablo/auctions/subsquidHelpers", () => {
  return { fetchTrades: jest.fn().mockImplementation(() => (Promise.resolve([])))}
})

jest.mock("@/defi/utils/pablo/pools/stats", () => {
  return { fetchPoolStats: jest.fn().mockImplementation(() => (Promise.resolve([]))), calculatePoolStats: jest.fn().mockImplementation(() => (undefined))}
})

jest.isolateModules(() => {
  const preloadAll = require('jest-next-dynamic');
  beforeAll(async () => {
    await preloadAll();
  });
});
