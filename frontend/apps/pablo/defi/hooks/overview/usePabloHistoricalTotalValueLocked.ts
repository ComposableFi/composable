import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { DEFI_CONFIG } from "@/defi/config";
import { fetchPabloOverviewTVLChart, Range } from "@/defi/subsquid/overview";
import { pipe } from "fp-ts/lib/function";
import * as TE from "fp-ts/TaskEither";
import * as E from "fp-ts/Either";
import { flow } from "fp-ts/function";
import * as A from "fp-ts/ReadonlyArray";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { parseLockedValue } from "@/components/Organisms/overview/parseLockedValue";
import { usePicaPriceDiscovery } from "@/defi/hooks/overview/usePicaPriceDiscovery";

const durationLabels: string[] = [];

export function usePabloHistoricalTotalValueLocked(): {
  chartSeries: [number, number][];
  selectedInterval: string;
  setSelectedInterval: Dispatch<SetStateAction<Range>>;
  durationLabels: string[];
  isLoading: boolean;
} {
  const [selectedInterval, setSelectedInterval] = useState<Range>(
    DEFI_CONFIG.swapChartIntervals[0].range as Range
  );
  const hasFetchedTokens = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const [chartSeries, setChartSeries] = useState<[number, number][]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const picaPrice = usePicaPriceDiscovery();
  const getTokenById = useStore((store) => store.substrateTokens.getTokenById);

  useEffect(() => {
    const task = pipe(
      TE.fromIO(() => setIsLoading(true)),
      TE.chain(fetchPabloOverviewTVLChart(selectedInterval)),
      TE.chainFirst(() => TE.fromIO(() => setIsLoading(false)))
    );
    if (hasFetchedTokens) {
      task().then(
        flow(
          E.match(
            () => setChartSeries([]),
            (tvl) => {
              const chart = pipe(
                A.fromArray(tvl.totalValueLocked),
                A.map((item) => {
                  const tvl = item.lockedValues.reduce(
                    parseLockedValue(getTokenById, picaPrice),
                    new BigNumber(0)
                  );
                  const date = Date.parse(item.date);
                  return [date, tvl.toNumber()] as const;
                }),
                A.toArray
              );
              setChartSeries(chart as [number, number][]);
            }
          )
        )
      );
    }
  }, [selectedInterval, hasFetchedTokens, getTokenById, picaPrice]);

  return {
    isLoading,
    chartSeries,
    selectedInterval,
    setSelectedInterval,
    durationLabels,
  };
}
