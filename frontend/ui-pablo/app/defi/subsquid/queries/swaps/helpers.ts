import { ChartRange, processSubsquidChartData } from "@/defi/utils";
import BigNumber from "bignumber.js";
import { queryPoolTransactionsByType } from "../pools";
import { query24hOldTransactionByPoolQuoteAsset } from "./queries";

export async function fetchSwapsChart(
  poolId: number,
  selectedQuoteAsset: string,
  chartRange: ChartRange
): Promise<[number, number][]> {
  let chartSeries: [number, number][] = [];

  try {
    const { data, error } = await queryPoolTransactionsByType(
      poolId,
      "SWAP",
      250
    );
    if (error) throw new Error(error.message);
    let { pabloTransactions } = data;

    let swapTransactions = pabloTransactions.map(
      (tx: {
        baseAssetId: string;
        quoteAssetId: string;
        receivedTimestamp: string;
        spotPrice: string;
      }) => {
        const { quoteAssetId, spotPrice, receivedTimestamp } = tx;
        let _spotPrice = new BigNumber(spotPrice);
        if (quoteAssetId !== selectedQuoteAsset) {
          _spotPrice = new BigNumber(1).div(_spotPrice);
        }

        return [+receivedTimestamp, _spotPrice.toNumber()];
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
    const { data, error } = await query24hOldTransactionByPoolQuoteAsset(
      poolId,
      Number(selectedQuoteAsset),
      "SWAP",
      1
    );
    if (error) throw new Error(error.message);
    let { pabloTransactions } = data;

    if (pabloTransactions.length > 0) {
      _24HourOldPrice = new BigNumber(pabloTransactions[0].spotPrice);
    }
  } catch (err) {
    console.error(err);
  }

  return _24HourOldPrice;
}
