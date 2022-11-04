import BigNumber from "bignumber.js";
import { stringToU8a } from "@polkadot/util";

export const PERMILL_UNIT = new BigNumber(1_000_000);
export const PERBILL_UNIT = new BigNumber(1_000_000_000);
export const PALLET_ID = "modl";
export const PALLET_TYPE_ID = stringToU8a(PALLET_ID);