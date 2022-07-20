import { Box, useTheme } from "@mui/material";
import { Chart, FeaturedBox } from "@/components/Molecules";
import { useStore } from "@/stores/root";
import {
  formatNumber,
  formatNumberWithSymbol,
  formatNumberCompact,
  formatNumberCompactWithToken,
  formatNumberCompactWithSymbol
} from "shared";
import { OverviewDataProps } from "@/stores/defi/stats/overview";

function formatOverviewTitleValue(index: number, info: OverviewDataProps) {
  switch (index) {
    case 0:
      return formatNumberWithSymbol(info.value, "$");
    case 1:
      return formatNumber(info.value);
    case 2:
      return formatNumberCompact(info.value);
    case 3:
      return formatNumberCompactWithToken(info.value, "pica");
    case 4:
      return formatNumberCompactWithSymbol(info.value, "$");
    case 5:
      return formatNumberWithSymbol(info.value, "$");
    default:
      return undefined;
  }
}

export const StatsOverviewTab: React.FC<{}> = ({}) => {
  const theme = useTheme();
  const {
    overviewData,
    overviewChartData,
    setTvlInterval,
    setDailyActiveUsersInterval
  } = useStore(({ statsOverview }) => statsOverview);

  function dispatchTVLInterval(selectedInterval: string) {
    setTvlInterval(
      overviewChartData.data[0].data.interval.indexOf(selectedInterval)
    );
  }
  function dispatchDailyUsersInterval(selectedInterval: string) {
    setDailyActiveUsersInterval(
      overviewChartData.data[1].data.interval.indexOf(selectedInterval)
    );
  }

  return (
    <>
      <Box
        display="grid"
        sx={{
          gridTemplateColumns: {
            xs: "1fr 1fr",
            lg: "1fr 1fr 1fr"
          }
        }}
        mb={5}
        gap={4}
      >
        {overviewData.data.map((info, index) => (
          <FeaturedBox
            key={index}
            textAbove={info.name}
            title={formatOverviewTitleValue(index, info)}
          />
        ))}
      </Box>
      <Box display="flex" flexDirection="column" gap={4}>
        {overviewChartData.data.map((info, index) => (
          <Chart
            key={index}
            title={info.data.name}
            totalText={
              index === 0
                ? formatNumberWithSymbol(info.data.value, "$")
                : formatNumber(info.data.value)
            }
            changeText={formatNumberWithSymbol(info.data.change, "", "%")}
            changeTextColor={
              info.data.change >= 0 ? "featured.lemon" : "error.main"
            }
            AreaChartProps={{
              data: info.data.data[info.data.pickedInterval],
              height: 90.7,
              shorthandLabel: "Change",
              labelFormat: (n: number) => n.toFixed(),
              color: theme.palette.primary.main
            }}
            currentInterval={info.data.interval[info.data.pickedInterval]}
            onIntervalChange={
              index === 0 ? dispatchTVLInterval : dispatchDailyUsersInterval
            }
            intervals={info.data.interval}
          />
        ))}
      </Box>
    </>
  );
};
