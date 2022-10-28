import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { unwrapNumberOrHex } from "shared";
import { fromChainIdUnit } from "./unit";

export const getExistentialDeposit = (api: ApiPromise): BigNumber =>
  fromChainIdUnit(
    unwrapNumberOrHex(api.consts.balances.existentialDeposit.toHex())
  );
