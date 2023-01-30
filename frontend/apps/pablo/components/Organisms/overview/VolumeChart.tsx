import { Skeleton, useTheme } from "@mui/material";
import { Chart } from "@/components";
import { DEFI_CONFIG } from "@/defi/config";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { useVolumeChart } from "@/defi/hooks/overview/useVolumeChart";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import { Range } from "@/defi/subsquid/overview";
import { useMemo } from "react";

export const VolumeChart = ({}) => {
  const theme = useTheme();
  const {
    isLoading,
    chartSeries,
    selectedInterval,
    setSelectedInterval,
    durationLabels,
  } = useVolumeChart();
  const intervals = DEFI_CONFIG.swapChartIntervals;

  const onIntervalChange = (symbol: string) => {
    pipe(
      intervals.find((interval) => interval.symbol === symbol),
      O.fromNullable,
      O.map((i) => setSelectedInterval(i.range as Range))
    );
  };

  const changeIntroText = useMemo(() => {
    switch (selectedInterval) {
      case "day":
        return `Past 24 hours`;
      case "week":
        return "Past week";
      case "month":
        return "Past month";
      case "year":
        return "Past year";
      case "all":
        return "All time";
    }
  }, [selectedInterval]);

  const changeTextProps = useMemo(
    () =>
      pipe(
        chartSeries,
        O.fromPredicate((c) => c.length >= 2),
        O.bindTo("series"),
        O.bind("first", ({ series }) =>
          O.fromNullable(series.slice(0, 1).pop())
        ),
        O.bind("last", ({ series }) => O.fromNullable(series.slice(-1).pop())),
        O.bind("diff", ({ first, last }) =>
          first[1] + last[1] === 0
            ? O.none
            : O.some((100 * (last[1] - first[1])) / ((last[1] + first[1]) / 2))
        ),
        O.fold(
          () => ({
            changeText: "0.00%",
            changeTextColor: theme.palette.text.primary,
          }),
          ({ diff }) => ({
            changeText: `${diff.toFixed(2)}%`,
            changeTextColor:
              diff > 0 ? theme.palette.success.main : theme.palette.error.main,
          })
        )
      ),
    [
      chartSeries,
      theme.palette.error.main,
      theme.palette.success.main,
      theme.palette.text.primary,
    ]
  );

  if (isLoading) {
    return <Skeleton variant="rounded" width="100%" height="512px" />;
  }

  return (
    <HighlightBox>
      <Chart
        height="100%"
        title="Volume"
        changeIntroText={changeIntroText}
        {...changeTextProps}
        AreaChartProps={{
          data: chartSeries,
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => {
            return `$${n.toFixed(2)}`;
          },
          color: theme.palette.featured.main,
        }}
        onIntervalChange={onIntervalChange}
        intervals={intervals.map((interval) => interval.symbol)}
        currentInterval={
          intervals.find((i) => i.range == selectedInterval)?.symbol
        }
        timeSlots={[]}
      />
    </HighlightBox>
  );
};
