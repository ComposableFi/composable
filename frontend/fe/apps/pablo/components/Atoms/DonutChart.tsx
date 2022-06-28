import React from "react";
import dynamic from "next/dynamic";
import { useTheme } from "@mui/material";

const Chart = dynamic(() => import("react-apexcharts"), { ssr: false });

export type DonutChartProps = {
  data: number[];
  labels: string[];
  width?: string;
  height?: string;
  colors: string[];
};

export const DonutChart: React.FC<DonutChartProps> = ({
  data,
  labels,
  width,
  height,
  colors,
}) => {
  const theme = useTheme();
  return (
    <Chart
      options={{
        colors: colors,
        labels: labels,
        dataLabels: { enabled: false },
        legend: {
          show: true,
          position: "bottom",
          fontSize: "16px",
          labels: { colors: [theme.palette.common.white] },
        },
        plotOptions: {
          pie: {
            donut: {
              size: "50%",
            },
          },
        },
        stroke: {
          show: false,
          width: 0,
        },
      }}
      series={data}
      type="donut"
      width={width}
      height={height}
    />
  );
};
