import { Chart, PairAsset } from "@/components";
import { Box, useTheme } from "@mui/material";
import { useMemo } from "react";
import { BoxProps } from "@mui/system";
import { DEFI_CONFIG } from "@/defi/config";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useSwapsChart } from "@/defi/hooks/swaps/useSwapsChart";
import { useAsset } from "@/defi/hooks/assets/useAsset";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

const SwapChart: React.FC<BoxProps> = ({ ...boxProps }) => {
  const theme = useTheme();

  const { swaps } = useStore();
  const {
    selectedInterval,
    chartSeries,
    seriesIntervals,
    _24hourOldPrice,
    setSelectedInterval,
  } = useSwapsChart();

  const baseAsset = useAsset(swaps.selectedAssets.base);
  const quoteAsset = useAsset(swaps.selectedAssets.quote);

  const changePercent = useMemo(() => {
    if (swaps.spotPrice.eq(0)) return 0;
    if (_24hourOldPrice.eq(0)) return 100;
    return new BigNumber(_24hourOldPrice)
      .div(swaps.spotPrice)
      .dp(DEFAULT_UI_FORMAT_DECIMALS)
      .toNumber();
  }, [swaps.spotPrice, _24hourOldPrice]);

  const intervals = DEFI_CONFIG.swapChartIntervals;

  const onIntervalChange = (interval: string) => {
    let i = intervals.find((i) => i.symbol === interval);
    if (i) setSelectedInterval(i);
  };

  const getCurrentInterval = () => {
    return intervals.find(
      (interval) => interval.symbol === selectedInterval.symbol
    );
  };

  // const onRefreshChart = () => {
  //TODO: refresh Chart Data
  // };
  return (
    <HighlightBox {...boxProps}>
      <Chart
        height="100%"
        titleComponent={
          <Box>
            <Box pt={2} display="flex" gap={1}>
              {baseAsset && quoteAsset ? (
                <PairAsset
                  assets={[
                    {
                      icon: baseAsset.getIconUrl(),
                      label: baseAsset.getSymbol(),
                    },
                    {
                      icon: quoteAsset.getIconUrl(),
                      label: quoteAsset.getSymbol(),
                    },
                  ]}
                  separator="-"
                />
              ) : null}
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
        totalText={`${swaps.spotPrice.dp(DEFAULT_UI_FORMAT_DECIMALS)} ${
          quoteAsset ? quoteAsset.getSymbol() : ""
        }`}
        changeTextColor={
          changePercent > 0
            ? theme.palette.featured.main
            : theme.palette.error.main
        }
        changeIntroText={`Past ${getCurrentInterval()?.name}`}
        changeText={
          changePercent > 0
            ? `+${changePercent}% ${quoteAsset ? quoteAsset.getSymbol() : ""}`
            : `${changePercent}% ${quoteAsset ? quoteAsset.getSymbol() : ""}`
        }
        AreaChartProps={{
          data: chartSeries,
          height: 330,
          shorthandLabel: "Change",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.featured.main,
        }}
        onIntervalChange={onIntervalChange}
        intervals={intervals.map((interval) => interval.symbol)}
        currentInterval={selectedInterval.symbol}
        timeSlots={seriesIntervals}
      />
    </HighlightBox>
  );
};

export default SwapChart;
