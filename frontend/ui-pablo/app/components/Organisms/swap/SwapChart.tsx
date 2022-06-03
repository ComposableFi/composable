import { Chart, PairAsset } from "@/components";
import {
  Box,
  useTheme,
} from "@mui/material";
import { useMemo } from "react";
import { BoxProps } from "@mui/system";
import { DEFI_CONFIG } from "@/defi/config";
import useStore from "@/store/useStore";
import moment from "moment";
import { getAsset } from "@/defi/polkadot/Assets";
import { SwapsChartRange } from "@/store/swaps/swaps.types";
import BigNumber from "bignumber.js";

const SwapChart: React.FC<BoxProps> = ({ ...boxProps }) => {
  const theme = useTheme();

  const {swaps, swapsChart, putSwapsChartSelectedRange} = useStore();

  const baseAsset = useMemo(() => {
    if (swaps.ui.baseAssetSelected !== "none") {
      return getAsset(swaps.ui.baseAssetSelected)
    }
    return null;
  }, [swaps.ui])

  const quoteAsset = useMemo(() => {
    if (swaps.ui.quoteAssetSelected !== "none") {
      return getAsset(swaps.ui.quoteAssetSelected)
    }
    return null;
  }, [swaps.ui])

  const changePercent = useMemo(() => {
    if (swaps.poolVariables.spotPrice === "0") return 0 
    if (swapsChart._24hourOldPrice === "0") return 100
    return new BigNumber(swapsChart._24hourOldPrice).div(swaps.poolVariables.spotPrice).toNumber()
  }, [swaps.poolVariables.spotPrice, swapsChart._24hourOldPrice]);

  const intervals = DEFI_CONFIG.swapChartIntervals;

  const timeSlots = useMemo(() => {
    return swapsChart.series.map((series) => {
      return moment.utc(series[0]).format("HH:mm")
    })
  }, [swapsChart.series]);

  const onIntervalChange = (interval: string) => {
    putSwapsChartSelectedRange(interval as SwapsChartRange)
  };

  const getCurrentInterval = () => {
    return intervals.find(
      (interval) => interval.symbol === swapsChart.selectedRange
    );
  };

  // const onRefreshChart = () => {
    //TODO: refresh Chart Data
  // };

  return (
    <Box {...boxProps}>
      <Chart
        height="100%"
        titleComponent={
          <Box>
            <Box pt={2} display="flex" gap={1}>
              {
                baseAsset && quoteAsset ? <PairAsset
                assets={[
                  {
                    icon: quoteAsset.icon,
                    label: quoteAsset.symbol,
                  },
                  {
                    icon: baseAsset.icon,
                    label: baseAsset.symbol
                  },
                ]}
                separator="-"
              /> : null
              }
              {/* <RefreshOutlined
                sx={{
                  cursor: "pointer",
                  "&:hover": {
                    color: theme.palette.primary.main,
                  },
                }}
                onClick={onRefreshChart}
              /> */}
            </Box>
          </Box>
        }
        totalText={`${swaps.poolVariables.spotPrice} ${baseAsset ? baseAsset.symbol : ""}`}
        changeTextColor={
          changePercent > 0
            ? theme.palette.featured.main
            : theme.palette.error.main
        }
        changeIntroText={`Past ${getCurrentInterval()?.name}`}
        changeText={
          changePercent > 0
            ? `+${changePercent}% ${baseAsset ? baseAsset.symbol : ""}`
            : `${changePercent}% ${baseAsset ? baseAsset.symbol : ""}`
        }
        AreaChartProps={{
          data: swapsChart.series,
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.featured.main,
        }}
        onIntervalChange={onIntervalChange}
        intervals={intervals.map((interval) => interval.symbol)}
        currentInterval={swapsChart.selectedRange}
        timeSlots={timeSlots}
      />
    </Box>
  );
};

export default SwapChart;
