import { Box, useTheme } from "@mui/material";
import { Chart } from "../Molecules";
import { FeaturedBox } from "../Molecules/FeaturedBox";

export const StatsTelemetryTab: React.FC<{}> = ({}) => {
  const theme = useTheme();
  return (
    <>
      <Box
        display="grid"
        sx={{
          gridTemplateColumns: {
            xs: "1fr",
            lg: "1fr 1fr 1fr",
          },
          gap: theme.spacing(4),
        }}
        mb={5}
      >
        <FeaturedBox textAbove="Finalized block" title="34,521" />
        <FeaturedBox textAbove="Average time" title="300.55K" />
        <FeaturedBox textAbove="Last block" title="300.55K" />
      </Box>
      <Chart
        title="Mempool &amp; fee growth"
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
    </>
  );
};
