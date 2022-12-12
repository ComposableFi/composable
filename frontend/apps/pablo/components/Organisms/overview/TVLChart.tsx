import { useTheme } from "@mui/material";
import { Chart } from "@/components";
import { DEFI_CONFIG } from "@/defi/config";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { usePabloHistoricalTotalValueLocked } from "@/defi/hooks/overview/usePabloHistoricalTotalValueLocked";
import { useMemo } from "react";

export const TVLChart = ({}) => {
  const theme = useTheme();
  const { chartSeries, setSelectedInterval, selectedInterval, durationLabels } = usePabloHistoricalTotalValueLocked();

  const change = useMemo(() => {
    if (chartSeries.length === 0) return 0; 

    const head = chartSeries[0][1];
    const tail = chartSeries[chartSeries.length - 1][1];
    
    return ((tail - head) / tail)
  }, [chartSeries]);

  return (
    <HighlightBox>
      <Chart
        height="100%"
        title="TVL"
        changeTextColor={change < 0 ? theme.palette.error.main : theme.palette.success.main}
        changeIntroText={`Past 24 hours`}
        changeText={change.toFixed(2)}
        AreaChartProps={{
          data: chartSeries,
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.featured.main,
        }}
        currentInterval={selectedInterval}
        onIntervalChange={setSelectedInterval}
        intervals={DEFI_CONFIG.swapChartIntervals.map(x => x.symbol)}
        timeSlots={durationLabels}
      />
    </HighlightBox>
  );
};
