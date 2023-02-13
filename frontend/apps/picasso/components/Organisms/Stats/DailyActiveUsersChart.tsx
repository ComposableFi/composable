import {
  ActiveUsers,
  GET_ACTIVE_USERS,
} from "@/apollo/queries/stats/activeUsers";
import { Chart } from "@/components";
import { ChartLoadingSkeleton } from "@/components/Organisms/Stats/ChartLoadingSkeleton";
import { useQuery } from "@apollo/client";
import { Box, Typography, useTheme } from "@mui/material";
import { FC, useMemo, useState } from "react";
import {
  formatNumber,
  getRange,
  PRESET_RANGE,
  PresetRange,
  tail,
} from "shared";
import { changeCalculator } from "@/components/Organisms/Stats/utils";

export const DailyActiveUsersChart: FC = () => {
  const theme = useTheme();
  const [range, setRange] = useState<PresetRange>("24h");
  const { data, loading, error } = useQuery<ActiveUsers>(GET_ACTIVE_USERS, {
    variables: {
      range: getRange(range),
    },
    pollInterval: 60_000, // Every 60 seconds
  });
  const chartSeries: [number, number][] = useMemo(() => {
    if (!data) return [];

    const tuples: [number, number][] = data.activeUsers.map((activeUser) => {
      const date = new Date(Number(activeUser.date));
      return [date.getTime(), activeUser.count];
    });

    return tuples.sort((a, b) => (a[0] > b[0] ? 1 : -1));
  }, [data]);
  const change = useMemo(() => {
    return changeCalculator(chartSeries, theme);
  }, [chartSeries, theme]);
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
        onIntervalChange={setRange}
        intervals={PRESET_RANGE as unknown as string[]}
        currentInterval={range}
      />
    </Box>
  );
};
