import BigNumber from "bignumber.js";
import { TokenId } from "../defi/types";

export const CURRENCY_ID_TO_TOKEN_ID_MAP: Record<string, TokenId> = {
  "1": "pica",
  "4": "ksm",
  "129": "usdc",
};

export const DEFAULT_DECIMALS = new BigNumber(10).pow(12);
