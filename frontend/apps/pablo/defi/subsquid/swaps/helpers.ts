import {
  ChartRange,
  MAX_CHART_LABELS,
  processSubsquidChartData,
  toMomentChartLabel,
} from "@/defi/utils";
import BigNumber from "bignumber.js";
import moment from "moment";
import { fetchPabloTransactions } from "../auctions/helpers";
import { querySpotPriceBeforeTimestamp } from "./queries";

export function getChartLabels(
  chartSeries: [number, number][],
  chartRange: ChartRange
): string[] {
  if (chartSeries.length < MAX_CHART_LABELS) {
    return chartSeries.map((i) =>
      moment(i[0]).format(toMomentChartLabel(chartRange))
    );
  }

  let steps = Math.floor(chartSeries.length / MAX_CHART_LABELS);

  let labels = [];
  for (let step = 0; step < chartSeries.length; step += steps) {
    labels.push(
      moment(chartSeries[step][0]).format(toMomentChartLabel(chartRange))
    );
  }

  return labels;
}

export async function fetchSwapsChart(
  poolId: number,
  selectedQuoteAsset: string,
  chartRange: ChartRange
): Promise<[number, number][]> {
  let chartSeries: [number, number][] = [];

  try {
    const { pabloTransactions } = await fetchPabloTransactions(
      poolId,
      "SWAP",
      "DESC",
      250
    );

    let swapTransactions = pabloTransactions.map(
      ({ quoteAssetId, spotPrice, pool: { calculatedTimestamp } }) => {
        let _spotPrice = new BigNumber(spotPrice);
        if (quoteAssetId !== selectedQuoteAsset) {
          _spotPrice = new BigNumber(1).div(_spotPrice);
        }

        return [+calculatedTimestamp, _spotPrice.toNumber()] as [
          number,
          number
        ];
      }
    );

    chartSeries = processSubsquidChartData(swapTransactions, chartRange);
  } catch (err) {
    console.error(err);
  }

  return chartSeries;
}

export async function fetch24HourOldPrice(
  poolId: number,
  selectedQuoteAsset: string
): Promise<BigNumber> {
  let _24HourOldPrice = new BigNumber(0);

  try {
    const { data, error } = await querySpotPriceBeforeTimestamp(
      poolId,
      Number(selectedQuoteAsset)
    );
    if (error) throw new Error(error.message);
    if (!data) throw new Error("[fetch24HourOldPrice] unable to fetch subsquid data.");
    let { pabloTransactions } = data;

    if (pabloTransactions.length > 0) {
      _24HourOldPrice = new BigNumber(pabloTransactions[0].spotPrice);
    }
  } catch (err) {
    console.error(err);
  }

  return _24HourOldPrice;
}
