import { FC, useMemo, useState } from "react";
import { getDiffInMinutes, getRange, PRESET_RANGE, PresetRange } from "shared";
import { useQuery } from "@apollo/client";
import { ActiveUsers, GET_ACTIVE_USERS } from "@/apollo/queries/activeUsers";
import { Box, Skeleton, Stack, useTheme } from "@mui/material";
import { Chart } from "@/components";

export const DailyActiveUsersChart: FC = () => {
  const theme = useTheme();
  const [interval, setInterval] = useState<PresetRange>("24h");
  const [dateFrom, dateTo, intervalQuery] = useMemo(() => getRange(interval), [interval]);
  const { data, loading, error } = useQuery<ActiveUsers>(GET_ACTIVE_USERS, {
    variables: {
      interval: intervalQuery,
      dateTo,
      dateFrom
    }
  });

  const chartSeries = useMemo(() => {
    if (!data) return [];

    return data.activeUsers.map(activeUser => {
      const date = new Date(activeUser.date);
      return [date.getTime(), activeUser.count];
    });
  }, [data]);

  const filledChartSeries = useMemo(() => {
    if (!dateFrom) return chartSeries;
    const startIndex = new Date(dateFrom);
    const intervalToSeriesCount = {
      "24h": 24,
      "1w": 7,
      "1m": 30,
      "1y": 365,
      "ALL": 365
    }[interval];
    let currentTimeSeriesIndex = 0;
    if (getDiffInMinutes(startIndex, new Date(chartSeries[currentTimeSeriesIndex][0])) < 0) {
      
    }
    for (let i = 1; i <= intervalToSeriesCount; i++) {

    }


  }, [chartSeries, dateFrom, interval]);

  if (loading) {
    return <Box
      borderRadius={1}
      padding={6}
      sx={{
        background: theme.palette.background.paper
      }}
    >
      <Stack direction="row" gap={3}>
        <Skeleton variant="text" height={48} width="50%" />
        <Skeleton variant="text" width={48} />
        <Skeleton variant="text" width={48} />
        <Skeleton variant="text" width={48} />
        <Skeleton variant="text" width={48} />
      </Stack>
      <Box sx={{
        height: 330,
        mt: 4
      }}>
        <Skeleton variant="rounded" height={330} width={"100%"} />
      </Box>
    </Box>;
  }

  if (error) {
    return <>
      {"error:" + error}
    </>;
  }
  return (
    <Box>
      <Chart
        height="100%"
        title="Daily active users"
        changeTextColor={theme.palette.error.main}
        changeText="+2% KSM"
        AreaChartProps={{
          data: [],
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.primary.main
        }}
        onIntervalChange={setInterval}
        intervals={PRESET_RANGE as unknown as string[]}
        currentInterval={interval}
      />
    </Box>
  );
};
