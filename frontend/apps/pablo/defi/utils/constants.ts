import BigNumber from "bignumber.js";
import moment from "moment";

export const DEFAULT_DECIMALS = new BigNumber(10).pow(12);
export const AVERAGE_BLOCK_TIME = 20 * 1000;
export const DEFAULT_NETWORK_ID = "picasso";
export const PALLET_TYPE_ID = "modl";
export const SECONDS = 1 * 1000;
export const MINUTES = 60 * SECONDS;
export const HOURS = 60 * MINUTES;
export const DAYS = 24 * HOURS;

export const FORMAT_1D = "hh:mm";
export const FORMAT_1w = "DD/MM";
export const FORMAT_1M = "MM/YYYY";

export const MAX_CHART_LABELS = 5;
export const APOLLO_UPDATE_BLOCKS = 6;
export const DEFAULT_UI_FORMAT_DECIMALS = 4;

export const DUMMY_LAUNCH_DESCRIPTION = (
  name: string = "Picasso",
  symbol: string = "PICA",
  startTime: number = Date.now(),
  duration: string = "2 days",
  vestingPeriod: string = "1 year"
): string[] => [
  `${symbol} Protocol aims to enable developers in the Polkadot ecosystem. \
      The ${name} protocol introduces DeFi 2.0, moving the industry a massive step closer towards the latter. \
      The LBP token event will be held for ${duration}, starting from ${moment(
    startTime
  ).utc()}.",
      "${symbol} tokens purchased in the LBP are the only ones without a lockup. \
      All other parties are subject to a minimum block by block vesting of ${vestingPeriod}, \
      making the LBP investors the only ones able to participate in ${symbol} or LP staking.`,
];


export const DEFAULT_SWAP_BASE = "1";
export const DEFAULT_SWAP_QUOTE = "4";