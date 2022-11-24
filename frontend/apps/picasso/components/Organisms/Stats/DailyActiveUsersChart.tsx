import { Chart } from "@/components";
import { Box, Typography, useTheme } from "@mui/material";
import { FC, useMemo, useState } from "react";
import {
  formatNumber,
  head,
  humanBalance,
  PRESET_RANGE,
  PresetRange,
  tail,
} from "shared";

export const DailyActiveUsersChart: FC = () => {
  const theme = useTheme();
  const [interval, setInterval] = useState<PresetRange>("24h");

  const chartSeries: [number, number][] = useMemo(() => [], []);

  const change = useMemo(() => {
    const first = head(chartSeries);
    const last = tail(chartSeries);

    if (first && last) {
      const firstValue = first[1];
      const lastValue = last[1];
      const percentageDifference =
        ((firstValue - lastValue) / firstValue) * 100;
      return {
        value: humanBalance(Math.abs(percentageDifference).toFixed(2)) + "%",
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

  return (
    <Box sx={{ height: 337 }}>
      <Chart
        height="100%"
        title="Daily active users"
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
          shorthandLabel: "Active users",
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
