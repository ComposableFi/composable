import { head, humanBalance, tail } from "shared";
import { Theme } from "@mui/material";

export function changeCalculator(chartSeries: [number, number][], theme: Theme) {
  const first = head(chartSeries);
  const last = tail(chartSeries);

  if (first && last) {
    const firstValue = first[1];
    const lastValue = last[1];
    const percentageDifference =
      ((firstValue - lastValue) / firstValue) * 100;
    return {
      value: humanBalance(Math.abs(Number.isNaN(percentageDifference) ? 0 : percentageDifference).toFixed(2)) + "%",
      color:
        firstValue > lastValue
          ? theme.palette.error.main
          : theme.palette.success.main
    };
  }

  return {
    value: "0",
    color: theme.palette.text.primary
  };
}
