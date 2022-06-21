import BigNumber from "bignumber.js";
import useStore from "../../useStore";

import { useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { useBlockInterval } from "../../../utils/defi/hooks/polkadot/useBlockInterval";
import { fromChainUnits } from "../../../utils/defi/fromChainUnits";
import { fetchVesitngPeriod } from "../../bonds/fetchVestingPeriod";
import { IDepositSummary } from "../../bonds/bonds.types";
import { DEFAULT_NETWORK_ID, fetchBalanceByAssetId } from "@/defi/utils";

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

  const lpPerBond = fromChainUnits(selectedBond.bondOffer.bondPrice);
  const vestingPeriod = fetchVesitngPeriod({
    interval: interval ? interval.toNumber() : undefined,
    bondMaturity: selectedBond.bondOffer.maturity,
  });

  const getNbOfBonds = (amount: number) => {
    const principalTokens = fromChainUnits(selectedBond.bondOffer.bondPrice);
    return Math.round(
      new BigNumber(amount)
        .div(principalTokens)
        .times(selectedBond.bondOffer.nbOfBonds)
        .toNumber()
    );
  };

  return {
    principalAsset: selectedBond.bondOffer.asset,
    userBalance: async () => {
      return await fetchBalanceByAssetId(
        parachainApi,
        selectedAccount.address,
        selectedBond.bondOffer.currencyId.toString()
      );
    },
    purchasableTokens: async () => {
      const userLPBalance = await fetchBalanceByAssetId(
        parachainApi,
        selectedAccount.address,
        selectedBond.bondOffer.currencyId.toString()
      );
      return new BigNumber(userLPBalance).div(lpPerBond).toString();
    },
    nbOfBonds: (amount: number) => {
      return getNbOfBonds(amount);
    },
    rewardableTokens: (amount: number) => {
      const rewardTokens = fromChainUnits(selectedBond.bondOffer.reward.amount);
      return rewardTokens.times(getNbOfBonds(amount)).toString();
    },
    roi: selectedBond.roi,
    vestingPeriod,
  };
}
