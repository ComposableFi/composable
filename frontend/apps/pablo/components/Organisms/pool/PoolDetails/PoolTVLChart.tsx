import { useTheme } from "@mui/material";
import { Chart } from "@/components/Molecules";
import { DEFI_CONFIG } from "@/defi/config";
import { usePoolTvlChart } from "@/defi/hooks/usePoolTvlChart";

export const PoolTVLChart: React.FC<{
  poolId: number
}> = ({ poolId }) => {
  const theme = useTheme();

  const {
    selectedInterval,
    setSelectedInterval,
    chartSeries,
    seriesIntervals
  } = usePoolTvlChart(poolId);

  const onIntervalChange = (intervalSymbol: string) => {
    const interval = DEFI_CONFIG.swapChartIntervals.find(i => i.symbol===intervalSymbol)
    if (interval) {
      setSelectedInterval(interval)
    }
  };

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
      intervals={DEFI_CONFIG.swapChartIntervals.map((interval) => interval.symbol)}
      currentInterval={selectedInterval.symbol}
      timeSlots={seriesIntervals}
      sx={{background: theme.palette.gradient.secondary}}
    />
  );
};

