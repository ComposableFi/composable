import { useTheme } from "@mui/material";
import { Chart } from "@/components";
import { DEFI_CONFIG } from "@/defi/config";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

export const TVLChart = ({}) => {
  const theme = useTheme();
  const intervals = DEFI_CONFIG.swapChartIntervals;

  const onIntervalChange = (interval: string) => {};

  const getCurrentInterval = () => {};

  return (
    <HighlightBox>
      <Chart
        height="100%"
        title="TVL"
        changeTextColor={theme.palette.error.main}
        changeIntroText={`Past 24 hours`}
        changeText=""
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
        timeSlots={["7:00 am", "10:00 am", "1:00 pm", "3:00 pm", "5:00 pm"]}
      />
    </HighlightBox>
  );
};
