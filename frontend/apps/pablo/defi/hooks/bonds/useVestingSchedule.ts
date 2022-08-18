import { BondOffer, BondPrincipalAsset } from "@/defi/types";
import { useCallback, useEffect, useMemo, useState } from "react";
import {
  calculateBondROI,
  decodeBondOffer,
  DEFAULT_NETWORK_ID,
} from "@/defi/utils";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { useBlockInterval } from "../useBlockInterval";
import BigNumber from "bignumber.js";

export default function useVestingSchedule(bondOffer: BondOffer | undefined): any {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const [vestingSchedule, setVestingSchedule] = useState(undefined);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  // useEffect(() => {
  //   if (parachainApi && bondOffer && selectedAccount) {
  //     parachainApi.query.vesting.vestingSchedules(selectedAccount.address, bondOffer.reward.asset).then((schedules) => {

  //     })
  //   }
  // }, [parachainApi, bondOffer, selectedAccount]);

  return vestingSchedule;
}