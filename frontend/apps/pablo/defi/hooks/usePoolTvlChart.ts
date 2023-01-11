import { DEFI_CONFIG } from "@/defi/config";
import { useEffect, useState } from "react";
import useStore from "@/store/useStore";
import { flow, pipe } from "fp-ts/function";
import * as TE from "fp-ts/TaskEither";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/ReadonlyArray";
import { fetchPabloTVLChartForPool } from "@/defi/subsquid/pools/queries";
import { parseLockedValue } from "@/components/Organisms/overview/parseLockedValue";
import { usePicaPriceDiscovery } from "@/defi/hooks/overview/usePicaPriceDiscovery";
import BigNumber from "bignumber.js";
import { Range } from "@/defi/subsquid/overview";

export const usePoolTvlChart = (poolId: string) => {
  const getTokenById = useStore((store) => store.substrateTokens.getTokenById);
  const hasFetchedTokens = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const hasFetchedPools = useStore((store) => store.pools.isLoaded);
  const [isLoading, setIsLoading] = useState(false);
  const [seriesIntervals, setSeriesIntervals] = useState<string[]>([]);
  const [chartSeries, setChartSeries] = useState<Array<[number, number]>>([]);
  const [selectedInterval, setSelectedInterval] = useState(
    DEFI_CONFIG.swapChartIntervals[0]
  );
  const picaPrice = usePicaPriceDiscovery();

  useEffect(() => {
    if (hasFetchedTokens && hasFetchedPools && picaPrice.gt(0)) {
      const task = pipe(
        TE.fromIO(() => setIsLoading(true)),
        TE.chain(
          fetchPabloTVLChartForPool(poolId, selectedInterval.range as Range)
        ),
        TE.chainFirst(() => TE.fromIO(() => setIsLoading(false)))
      );

      task().then(
        flow(
          E.fold(
            (e) => setChartSeries(e.pabloTVL),
            (a) => {
              const chartData = pipe(
                A.fromArray(a.pabloTVL),
                A.map((item) => {
                  const date = Date.parse(item.date);
                  const value = parseLockedValue(getTokenById, picaPrice)(
                    new BigNumber(0),
                    {
                      assetId: item.assetId,
                      amount: item.totalValueLocked,
                    }
                  );
                  return [date, value.toNumber()] as const;
                }),
                A.toArray
              );

              setChartSeries(chartData as [number, number][]);
            }
          )
        )
      );
    }
  }, [picaPrice, hasFetchedTokens, hasFetchedPools, selectedInterval, poolId]);

  return {
    isLoading,
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    seriesIntervals,
  };
};
