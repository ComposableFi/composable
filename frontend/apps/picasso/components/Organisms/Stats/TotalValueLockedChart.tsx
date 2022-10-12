import { FC, useMemo, useState } from "react";
import {
  formatNumber,
  getRange,
  head,
  PRESET_RANGE,
  PresetRange,
  tail,
} from "shared";
import { useQuery } from "@apollo/client";
import { Box, Typography, useTheme } from "@mui/material";
import { Chart } from "@/components";
import {
  GET_TOTAL_VALUE_LOCKED,
  TotalValueLocked,
} from "@/apollo/queries/totalValueLocked";
import { ChartLoadingSkeleton } from "@/components/Organisms/Stats/ChartLoadingSkeleton";

export const TotalValueLockedChart: FC = () => {
  const theme = useTheme();
  const [interval, setInterval] = useState<PresetRange>("24h");
  const [dateFrom, dateTo, intervalQuery] = useMemo(
    () => getRange(interval),
    [interval]
  );
  const { data, loading, error } = useQuery<TotalValueLocked>(
    GET_TOTAL_VALUE_LOCKED,
    {
      variables: {
        interval: intervalQuery,
        dateTo,
        dateFrom,
      },
    }
  );
  const chartSeries: [number, number][] = useMemo(() => {
    if (!data) return [];

    const tuples: [number, number][] = data.totalValueLocked.map((tvl) => {
      const date = new Date(tvl.date);
      return [date.getTime(), tvl.totalValueLocked];
    });

    return tuples.sort((a, b) => (a[0] > b[0] ? 1 : -1));
  }, [data]);
  const change = useMemo(() => {
    const first = head(chartSeries);
    const last = tail(chartSeries);

    if (first && last) {
      const firstValue = first[1];
      const lastValue = last[1];
      console.log(firstValue, lastValue);

      const percentageDifference =
        ((firstValue - lastValue) / firstValue) * 100;
      return {
        value: percentageDifference.toFixed(2) + "%",
        color:
          firstValue > lastValue
            ? theme.palette.error.main
            : theme.palette.success.main,
      };
    }

    return {
      value: "",
      color: theme.palette.text.primary,
    };
  }, [
    chartSeries,
    theme.palette.error.main,
    theme.palette.success.main,
    theme.palette.text.primary,
  ]);
  const changeTextPrimary = useMemo(() => {
    const first = tail(chartSeries);
    return formatNumber(first?.[1] ?? 0);
  }, [chartSeries]);

  if (loading) {
    return <ChartLoadingSkeleton />;
  }

  if (error) {
    return <>{"error:" + error}</>;
  }

  return (
    <Box sx={{ height: 337 }}>
      <Chart
        height="100%"
        title="Total value locked"
        changeTextColor={theme.palette.text.primary}
        ChangeTextTypographyProps={{
          variant: "h5",
        }}
        changeText={
          <>
            <Typography variant="h5">{changeTextPrimary}</Typography>
            <Typography color={change.color} variant="body1">
              {change.value}
            </Typography>
          </>
        }
        AreaChartProps={{
          data: chartSeries,
          height: 118,
          shorthandLabel: "Locked value",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.primary.main,
        }}
        onIntervalChange={setInterval}
        intervals={PRESET_RANGE as unknown as string[]}
        currentInterval={interval}
      />
    </Box>
  );
};
