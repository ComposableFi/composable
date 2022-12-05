import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { pabloHistoricalValues } from "@/defi/subsquid/overview";
import { fromChainIdUnit } from "shared";
import { DEFI_CONFIG } from "@/defi/config";
import { ChartRange, processSubsquidChartData } from "@/defi/utils";
import { getChartLabels } from "@/defi/subsquid/swaps/helpers";

export function usePabloHistoricalTotalValueLocked(): {
    chartSeries: [number, number][],
    selectedInterval: string,
    setSelectedInterval: Dispatch<SetStateAction<string>>,
    durationLabels: string[]
} {
    const [selectedInterval, setSelectedInterval] = useState(DEFI_CONFIG.swapChartIntervals[0].symbol);
    const [durationLabels, setDurationLabels] = useState<string[]>([]);
    const [chartSeries, setChartSeries] = useState<[number, number][]>([]);

    useEffect(() => {
        pabloHistoricalValues().then((lockedValues) => {
            let chartSeriesRaw = lockedValues.map((lockedValue) => {
                const amountInUSD = BigInt(lockedValue.amount);
                const timestamp = +lockedValue.timestamp;
                return [timestamp, fromChainIdUnit(amountInUSD, 12).toNumber()]
            }) as [number, number][];

            // testing
            // let diff = selectedInterval === "24h" ? 1 * DAYS : selectedInterval === "1w" ? 7 * DAYS : 30 * DAYS;
            // let chartSeriesRaw = generateRandomSubsquidTvlData(diff, 5, 10, 50);

            let chartSeries = processSubsquidChartData(
                chartSeriesRaw,
                selectedInterval as ChartRange
            );

            setDurationLabels(getChartLabels(chartSeries, selectedInterval as ChartRange))
            setChartSeries(chartSeries)
        })
    }, [selectedInterval]);

    return {
        chartSeries,
        selectedInterval,
        setSelectedInterval,
        durationLabels
    };
}