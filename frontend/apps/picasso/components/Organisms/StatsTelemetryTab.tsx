import { Box, useTheme } from "@mui/material";
import { Chart, FeaturedBox } from "@/components/Molecules";
import { useStore } from "@/stores/root";
import { formatNumber, formatNumberCompact } from "shared";
import { TelemetryDataProps } from "@/stores/defi/stats/telemetry";

function formatTelemetryTitleValue(index: number, info: TelemetryDataProps) {
  switch (index) {
    case 0:
      return formatNumber(info.value);
    default:
      return formatNumberCompact(info.value);
  }
}

export const StatsTelemetryTab: React.FC<{}> = ({}) => {
  const theme = useTheme();
  const { telemetryData, telemetryChartData, setMemPoolInterval } = useStore(
    ({ statsTelemetry }) => statsTelemetry
  );

  function dispatchMemPoolInterval(selectedInterval: string) {
    setMemPoolInterval(
      telemetryChartData.data[0].data.interval.indexOf(selectedInterval)
    );
  }

  return (
    <>
      <Box
        display="grid"
        sx={{
          gridTemplateColumns: {
            xs: "1fr",
            lg: "1fr 1fr 1fr"
          }
        }}
        mb={5}
        gap={4}
      >
        {telemetryData.data.map((info, index) => (
          <FeaturedBox
            key={index}
            textAbove={info.name}
            title={formatTelemetryTitleValue(index, info)}
          />
        ))}
      </Box>
      {telemetryChartData.data.map((info, index) => (
        <Chart
          key={index}
          title={info.data.name}
          AreaChartProps={{
            data: info.data.data[info.data.pickedInterval],
            height: 90.7,
            shorthandLabel: "Change",
            labelFormat: (n: number) => n.toFixed(),
            color: theme.palette.primary.main
          }}
          currentInterval={info.data.interval[info.data.pickedInterval]}
          onIntervalChange={dispatchMemPoolInterval}
          intervals={info.data.interval}
        />
      ))}
    </>
  );
};
