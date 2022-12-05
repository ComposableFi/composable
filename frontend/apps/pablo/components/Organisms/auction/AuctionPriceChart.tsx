import React, { useMemo, useState } from "react";
import dynamic from "next/dynamic";
import { Box, Typography, useTheme } from "@mui/material";
import FiberManualRecordIcon from "@mui/icons-material/FiberManualRecord";
import { Asset } from "shared";
import { ApexOptions } from "apexcharts";

const NoSSRChart = dynamic(() => import("react-apexcharts"), { ssr: false });

const createChartOptions = (color: string, theme: any, dateFormat: any): ApexOptions => {
  return {
    grid: {
      show: false,
      padding: {
        left: 0,
        right: 0,
      },
    },
    legend: {
      show: false,
    },
    chart: {
      redrawOnParentResize: false,
      type: "area",
      toolbar: {
        show: false,
      },
      fontFamily: "'Konnect', serif",
      zoom: {
        enabled: false,
      },
    },
    fill: {
      type: "gradient",
      gradient: {
        opacityFrom: 0,
        opacityTo: 0,
      },
    },
    stroke: {
      width: [2, 2],
      colors: [color, theme.palette.common.white],
      curve: "smooth",
    },
    colors: [color],
    markers: {
      colors: [color],
      strokeColors: [color],
      strokeWidth: 1,
    },
    tooltip: {
      theme: "dark",
      shared: false,
      custom: (options: any) => {
        return (
          "<div class='y-label'>$" +
          options.series[options.seriesIndex][options.dataPointIndex] +
          "</div>" +
          "<div class='x-label'>" +
          dateFormat(options.w.globals.labels[options.dataPointIndex]) +
          "</div>"
        );
      },
      x: {
        show: false,
      },
      y: {
        formatter: (v: number) => v?.toFixed(),
      },
    },
    dataLabels: {
      enabled: false,
    },
    xaxis: {
      labels: {
        show: false,
        offsetY: 0,
      },
      axisBorder: {
        show: false,
      },
      axisTicks: {
        show: false,
      },
      tooltip: {
        enabled: false,
      },
    },
    yaxis: {
      show: true,
      axisBorder: {
        show: false,
      },
      labels: {
        offsetX: -15,
        style: {
          fontSize: "16px",
          fontFamily: theme.custom.fontFamily.primary,
          fontWeight: 300,
          colors: [theme.palette.common.white],
        },
        formatter: (val: number, opts?: any) => {
          return "$" + val?.toFixed();
        },
      },
      tooltip: {
        enabled: false,
      },
    },
  };
};

type PriceChartLabelProps = {
  priceSeries: [number, number][];
  predictedPriceSeries: [number, number][];
  assetSymbol: string;
};

const PriceChartLabels = ({
  predictedPriceSeries,
  priceSeries,
  assetSymbol,
}: PriceChartLabelProps) => {
  if (priceSeries.length > 0 && predictedPriceSeries.length > 0) {
    return (
      <>
        <FiberManualRecordIcon color="primary" />
        <Typography variant="body2" pl={1} pr={2}>
          {assetSymbol}
        </Typography>
        <FiberManualRecordIcon color="inherit" />
        <Typography variant="body2" pl={1} whiteSpace="nowrap">
          {assetSymbol} predicted price (without new buyers)
        </Typography>
      </>
    );
  }

  if (priceSeries.length === 0 || predictedPriceSeries.length === 0) {
    let label = `${assetSymbol}`;
    if (priceSeries.length === 0) {
      label += " predicted price (without new buyers)";
    }

    return (
      <>
        <FiberManualRecordIcon color="primary" />
        <Typography variant="body2" pl={1} pr={2}>
          {label}
        </Typography>
      </>
    );
  }
  return null;
};

export type AuctionPriceChartProps = {
  baseAsset: Asset;
  chartSeries: {
    currentPriceSeries: [number, number][];
    predictedPriceSeries: [number, number][];
  };
  height: number | string;
  dateFormat: (timestamp: number | string) => string;
  color?: string;
};

export const AuctionPriceChart: React.FC<AuctionPriceChartProps> = ({
  baseAsset,
  chartSeries,
  height,
  dateFormat,
  color,
}) => {
  const theme = useTheme();

  const [options, setChartOptions] = useState(createChartOptions(color || theme.palette.primary.main, theme, dateFormat));
  const series = useMemo(() => {
    return [
      { data: chartSeries.currentPriceSeries, name: "Price" },
      { data: chartSeries.predictedPriceSeries, name: "Predicted Price" }
    ];
  }, [chartSeries])

  return (
    <Box height={height}>
      <Box height="calc(100% - 155px)" width="calc(100% - 24px)">
        <NoSSRChart
          options={options}
          series={series}
          type="area"
          height="100%"
        />
      </Box>
      {/* <Box
        display="flex"
        alignItems="center"
        justifyContent="space-around"
        pl={4}
        mt={3}
      >
        {dates.map((date) => (
          <Typography variant="body2" key={date}>
            {date}
          </Typography>
        ))}
      </Box> */}
      <Box display="flex" alignItems="center" mt={6.5}>
        <PriceChartLabels
          predictedPriceSeries={chartSeries.predictedPriceSeries}
          priceSeries={chartSeries.currentPriceSeries}
          assetSymbol={baseAsset.getSymbol()}
        />
      </Box>
    </Box>
  );
};
