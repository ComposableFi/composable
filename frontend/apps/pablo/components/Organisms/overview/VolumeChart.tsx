import { Box, useTheme } from "@mui/material";
import { Chart } from "@/components";
import { DEFI_CONFIG } from "@/defi/config";

export const VolumeChart = ({}) => {
  const theme = useTheme();
  const intervals = DEFI_CONFIG.swapChartIntervals;

  const onIntervalChange = (interval: string) => {};

  const getCurrentInterval = () => {};

  return (
    <Box>
      <Chart
        height="100%"
        title="TVL"
        changeTextColor={theme.palette.error.main}
        changeIntroText={`Past 24 hours`}
        changeText="+2% KSM"
        AreaChartProps={{
          data: [],
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.featured.main,
        }}
        onIntervalChange={onIntervalChange}
        intervals={["1w", "1m", "1y", "All"]}
        currentInterval={"hr"}
        timeSlots={["hr", "minute"]}
      />
    </Box>
  );
};
