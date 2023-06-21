import { Skeleton, useTheme } from "@mui/material";
import { Chart } from "@/components/Molecules";
import { DEFI_CONFIG } from "@/defi/config";
import { usePoolTvlChart } from "@/defi/hooks/usePoolTvlChart";
import { FC } from "react";

export const PoolTVLChart: FC<{
  poolId: string;
}> = ({ poolId }) => {
  const theme = useTheme();

  const {
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    isLoading,
    seriesIntervals,
  } = usePoolTvlChart(poolId);

  const onIntervalChange = (intervalSymbol: string) => {
    const interval = DEFI_CONFIG.swapChartIntervals.find(
      (i) => i.symbol === intervalSymbol
    );
    if (interval) {
      // @ts-ignore
      setSelectedInterval(interval);
    }
  };

  if (isLoading) {
    return <Skeleton variant="rounded" width="100%" height="420px" />;
  }
  return (
    <Chart
      title="TVL"
      changeTextColor={theme.palette.common.white}
      changeText={`Past ${selectedInterval.name}`}
      AreaChartProps={{
        data: chartSeries,
        height: 238,
        shorthandLabel: "Change",
        labelFormat: (n: number) => n.toFixed(),
        color: theme.palette.common.white,
      }}
      onIntervalChange={onIntervalChange}
      intervals={DEFI_CONFIG.swapChartIntervals.map(
        (interval) => interval.symbol
      )}
      currentInterval={selectedInterval.symbol}
      timeSlots={seriesIntervals}
      sx={{ background: theme.palette.gradient.secondary }}
    />
  );
};
