import BigNumber from "bignumber.js";
import useStore from "../useStore";

import { useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { fetchBalanceByAssetId } from "../../updaters/balances/utils";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";
import { useBlockInterval } from "../../utils/defi/hooks/polkadot/useBlockInterval";
import { fromPica } from "../../utils/defi/fromPica";
import { fetchVesitngPeriod } from "../bonds/fetchVestingPeriod";
import { IDepositSummary } from "../bonds/bonds.types";

type Props = {
  offerId: number;
};

export function useDepositSummary({
  offerId,
}: Props): "no-summary" | IDepositSummary {
  const { allBonds } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const selectedBond = useMemo(
    () => allBonds.find((bond) => bond.offerId === offerId),
    [allBonds, offerId]
  );
  const interval = useBlockInterval();

  if (!selectedBond || !selectedAccount || !parachainApi) {
    return "no-summary";
  }

  const lpPerBond = fromPica(selectedBond.bondOffer.bondPrice);
  const vestingPeriod = fetchVesitngPeriod({
    interval: interval ? interval.toNumber() : undefined,
    bondMaturity: selectedBond.bondOffer.maturity,
  });

  return {
    principalAsset: selectedBond.bondOffer.asset,
    userBalance: async () => {
      return await fetchBalanceByAssetId(
        parachainApi,
        DEFAULT_NETWORK_ID,
        selectedAccount.address,
        selectedBond.bondOffer.currencyId.toString()
      );
    },
    purchasableTokens: async () => {
      const userLPBalance = await fetchBalanceByAssetId(
        parachainApi,
        DEFAULT_NETWORK_ID,
        selectedAccount.address,
        selectedBond.bondOffer.currencyId.toString()
      );
      return new BigNumber(userLPBalance).div(lpPerBond).toString();
    },
    rewardableTokens: (amount: number) => {
      const principalTokens = fromPica(selectedBond.bondOffer.bondPrice);
      const nbOfBonds = new BigNumber(amount)
        .div(principalTokens)
        .times(selectedBond.bondOffer.nbOfBonds);
      const rewardTokens = fromPica(selectedBond.bondOffer.reward.amount);
      return rewardTokens.times(nbOfBonds).toString();
    },
    roi: selectedBond.roi,
    vestingPeriod,
  };
}
