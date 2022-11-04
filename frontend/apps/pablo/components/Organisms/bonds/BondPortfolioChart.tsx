import {
  Box,
  useTheme,
  Typography,
} from "@mui/material";
import { Chart } from "@/components/Molecules";
import { DEFI_CONFIG } from "@/defi/config";
import { useState } from "react";
import { RefreshOutlined } from "@mui/icons-material";
import BigNumber from "bignumber.js";

export const BondPortfolioChart: React.FC<{}> = ({}) => {
  const theme = useTheme();
  const {total, change, series} = {
    total: new BigNumber(24546395.04),
    change: 2,
    series: [],
  };
  const intervals = DEFI_CONFIG.bondChartIntervals;
  const [currentInterval, setCurrentInterval] = useState(
    intervals[0].symbol
  );
  const changeText = change >= 0 ? `+${change}` : change;
  const changeTextColor = change >= 0
                            ? theme.palette.featured.main
                            : theme.palette.error.main;

  const onIntervalChange = (interval: string) => {
    setCurrentInterval(interval);
  };

  const onRefreshChart = () => {
    console.log("Refresh Chart");
  };

  return (
    <Chart
      titleComponent={
        <Box display="flex" alignItems="center" gap={1}>
          <Typography variant="body1" color="text.secondary">
            My portfolio
          </Typography>
          <RefreshOutlined
            sx={{
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.primary.main,
              },
            }}
            onClick={onRefreshChart}
          />
        </Box>
      }
      totalText={`$${total.toFormat()}`}
      changeTextColor={changeTextColor}
      changeText={`${changeText}% KSM`}
      AreaChartProps={{
        data: series,
        height: 85,
        shorthandLabel: "Change",
        labelFormat: (n: number) => n.toFixed(),
        color: theme.palette.featured.main,
      }}
      onIntervalChange={onIntervalChange}
      intervals={intervals.map((interval) => interval.symbol)}
      currentInterval={currentInterval}
      sx={{background: theme.palette.gradient.secondary}}
    />
  );
};

