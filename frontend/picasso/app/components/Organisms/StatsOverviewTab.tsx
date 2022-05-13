import { Box, useTheme } from "@mui/material";
import { Chart } from "../Molecules";
import { FeaturedBox } from "../Molecules/FeaturedBox";

export const StatsOverviewTab: React.FC<{}> = ({}) => {
  const theme = useTheme();

  return (
    <>
      <Box
        display="grid"
        sx={{
          gridTemplateColumns: {
            xs: "1fr 1fr",
            lg: "1fr 1fr 1fr",
          },
          gap: theme.spacing(4),
        }}
        mb={5}
      >
        <FeaturedBox textAbove="Total value locked" title="$34,458,567" />
        <FeaturedBox textAbove="Account holders" title="34,521" />
        <FeaturedBox textAbove="Total transactions" title="300.55K" />
        <FeaturedBox textAbove="Reward distribution" title="500K PICA" />
        <FeaturedBox textAbove="Total fees" title="$120K" />
        <FeaturedBox textAbove="Earned staking TVL" title="$25,324,533" />
      </Box>
      <Box display="flex" flexDirection="column" gap={theme.spacing(4)}>
        <Chart
          title="Total value locked"
          totalText="$54,653,784"
          changeText="+34%"
          changeTextColor={theme.palette.featured.lemon}
          AreaChartProps={{
            data: [
              [1644550600000, 20],
              [1644560620928, 40],
              [1644570600000, 35],
              [1644580600000, 60],
              [1644590600000, 80],
            ],
            height: 200,
            shorthandLabel: "Change",
            labelFormat: (n: number) => n.toFixed(),
            color: theme.palette.primary.main,
          }}
          intervals={["1h", "24h", "1w", "1m"]}
        />
        <Chart
          title="Daily active users"
          totalText="12,567"
          changeText="+34%"
          changeTextColor={theme.palette.featured.lemon}
          AreaChartProps={{
            data: [
              [1644550600000, 20],
              [1644560620928, 40],
              [1644570600000, 35],
              [1644580600000, 60],
              [1644590600000, 80],
            ],
            height: 200,
            shorthandLabel: "Change",
            labelFormat: (n: number) => n.toFixed(),
            color: theme.palette.primary.main,
          }}
          intervals={["1h", "24h", "1w", "1m"]}
        />
      </Box>
    </>
  );
};
