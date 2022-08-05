import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "picasso/defi/polkadot/pallets/BondedFinance";
import { unwrapNumberOrHex } from "shared";

export const getExistentialDeposit = (api: ApiPromise): BigNumber =>
  fromChainIdUnit(
    unwrapNumberOrHex(api.consts.balances.existentialDeposit.toHex())
  );
