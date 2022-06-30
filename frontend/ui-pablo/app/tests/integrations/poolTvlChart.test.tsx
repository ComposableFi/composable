import { ChartRange, processSubsquidChartData, generateRandomSubsquidTvlData } from "@/defi/utils/charts";
import _ from "lodash";
import moment from "moment";

const SECONDS = 1 * 1000;
const MINUTES = 60 * SECONDS;
const HOURS = 60 * MINUTES;
const DAYS = 24 * HOURS;

describe("TVL Chart Series", () => {

  test("Test with Single Point", () => {
    const subsquidData: [number, number][] = [
      [1654278809477, 3],
    ];

    let range: ChartRange = "24h";
    const series24h = processSubsquidChartData(subsquidData, range);
    range = "1w";
    const series1w = processSubsquidChartData(subsquidData, range);
    range = "1m";
    const series1m = processSubsquidChartData(subsquidData, range);

    expect(series24h.length).toBe(1);
    expect(series1w.length).toBe(1);
    expect(series1m.length).toBe(1);
  });
  test("Test with Single day", () => {
    const subsquidData: [number, number][] = [
      [1654278809477, 3],
      [1654271612229, 5],
      [1654268015774, 8],
    ];

    let range: ChartRange = "24h";
    const series24h = processSubsquidChartData(subsquidData, range);
    range = "1w";
    const series1w = processSubsquidChartData(subsquidData, range);
    range = "1m";
    const series1m = processSubsquidChartData(subsquidData, range);

    expect(series24h.length).toBe(4);
    expect(series1w.length).toBe(1);
    expect(series1m.length).toBe(1);
  });
  test("Test with no data", () => {
    const subsquidData: [number, number][] = [];

    let range: ChartRange = "24h";
    const series24h = processSubsquidChartData(subsquidData, range);
    range = "1w";
    const series1w = processSubsquidChartData(subsquidData, range);
    range = "1m";
    const series1m = processSubsquidChartData(subsquidData, range);

    expect(series24h.length).toBe(0);
    expect(series1w.length).toBe(0);
    expect(series1m.length).toBe(0);
  });
  test("Test with data generated (24h)", () => {
    const dummyDateRange = 2 * DAYS;
    let rightNow = Date.now();

    const expectedMinimum = moment(rightNow - dummyDateRange)
      .startOf("hour")
      .valueOf();
    const subsquidData: [number, number][] = generateRandomSubsquidTvlData(
      dummyDateRange,
      5000
    );

    let range: ChartRange = "24h";
    const series24h = processSubsquidChartData(subsquidData, range);

    let firstSeriesTimeStamp = series24h[0][0],
      lastSeriesTimeStamp = series24h[series24h.length - 1][0];
    expect(firstSeriesTimeStamp).toBeGreaterThanOrEqual(expectedMinimum);

    let hours = 0,
      hourStep = firstSeriesTimeStamp;
    while (hourStep <= lastSeriesTimeStamp) {
      hours = hours + 1;
      hourStep += 1 * HOURS;
    }

    expect(hours).toEqual(series24h.length);
  });
  test("Test with data generated (1w)", () => {
    const dummyDateRange = 15 * DAYS;
    let rightNow = Date.now();

    const expectedMinimum = moment(rightNow - dummyDateRange)
      .startOf("week")
      .valueOf();
    const subsquidData: [number, number][] = generateRandomSubsquidTvlData(
      dummyDateRange,
      5000
    );

    let range: ChartRange = "1w";
    const series24h = processSubsquidChartData(subsquidData, range);

    let firstSeriesTimeStamp = series24h[0][0],
      lastSeriesTimeStamp = series24h[series24h.length - 1][0];
    expect(firstSeriesTimeStamp).toBeGreaterThanOrEqual(expectedMinimum);

    let weeks = 0,
      weekStep = firstSeriesTimeStamp;
    while (weekStep <= lastSeriesTimeStamp) {
      weeks = weeks + 1;
      weekStep += 7 * DAYS;
    }

    expect(weeks).toEqual(series24h.length);
  });
});
