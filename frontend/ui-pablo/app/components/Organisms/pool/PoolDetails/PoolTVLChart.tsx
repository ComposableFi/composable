import { useTheme } from "@mui/material";
import { Chart } from "@/components/Molecules";
import { DEFI_CONFIG } from "@/defi/config";
import { useState } from "react";
import { useAppSelector } from "@/hooks/store";

export const PoolTVLChart: React.FC<{}> = ({}) => {
  const theme = useTheme();
  const {series, timeSlots} = useAppSelector(
    (state) => state.pool.selectedPool.tvlChartData
  );
  const intervals = DEFI_CONFIG.poolChartIntervals;
  const [currentInterval, setCurrentInterval] = useState(
    intervals[0]
  );

  const onIntervalChange = (intervalSymbol: string) => {
    const interval = intervals.find(item => item.symbol === intervalSymbol)
    interval && setCurrentInterval(interval);
  };

  return (
    <Chart
      title="TVL"
      changeTextColor={theme.palette.common.white}
      changeText={`Past ${currentInterval.name}`}
      AreaChartProps={{
        data: series,
        height: 238,
        shorthandLabel: "Change",
        labelFormat: (n: number) => n.toFixed(),
        color: theme.palette.common.white,
      }}
      onIntervalChange={onIntervalChange}
      intervals={intervals.map((interval) => interval.symbol)}
      currentInterval={currentInterval.symbol}
      timeSlots={timeSlots}
      sx={{background: theme.palette.gradient.secondary}}
    />
  );
};

