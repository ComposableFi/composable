import { useEffect } from "react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import {
  calculateBondROI,
  DEFAULT_NETWORK_ID,
  fetchBondOffers,
  fetchVestingSchedulesByBondOffers,
} from "@/defi/utils";
import {
  putBondedOfferBondedVestingScheduleIds,
  putBondedOfferVestingSchedules,
  putBondOffers,
  putBondOffersReturnOnInvestmentRecord,
  putBondOffersTotalPurchasedCount,
  useBondOffersSlice,
} from "@/store/bond/bond.slice";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import {
  fetchTotalPurchasedBondsByOfferIds,
  extractUserBondedFinanceVestingScheduleAddedEvents,
} from "@/defi/subsquid/bonds/helpers";

const Updater = () => {
  const { apollo } = useStore();
  const { bondOffers, bondedOfferVestingScheduleIds } = useBondOffersSlice();

  useEffect(() => {
    const roiRecord = bondOffers.reduce((acc, bondOffer) => {
      const principalAssetPrinceInUSD =
        new BigNumber(apollo[bondOffer.asset]) || new BigNumber(0);
      const rewardAssetPriceInUSD =
        new BigNumber(apollo[bondOffer.reward.asset]) || new BigNumber(0);
      const rewardAssetAmountPerBond = bondOffer.reward.amount.div(
        bondOffer.nbOfBonds
      );
      const principalAssetAmountPerBond = bondOffer.bondPrice;
      return {
        ...acc,
        [bondOffer.offerId.toString()]: calculateBondROI(
          principalAssetPrinceInUSD,
          rewardAssetPriceInUSD,
          principalAssetAmountPerBond,
          rewardAssetAmountPerBond
        ),
      };
    }, {} as Record<string, BigNumber>);

    putBondOffersReturnOnInvestmentRecord(roiRecord);
  }, [apollo, bondOffers]);

  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  /**
   * Query chain
   * for bond offers
   */
  useEffect(() => {
    if (parachainApi) {
      fetchBondOffers(parachainApi).then(putBondOffers);
    }
  }, [parachainApi]);
  /**
   * Query subsquid for showing
   * total bonds purchased w.r.t offers
   */
  useEffect(() => {
    fetchTotalPurchasedBondsByOfferIds().then(putBondOffersTotalPurchasedCount);
  }, []);
  /**
   * 
   */
  useEffect(() => {
    if (selectedAccount && parachainApi) {
      extractUserBondedFinanceVestingScheduleAddedEvents(
        parachainApi,
        selectedAccount.address
      ).then(putBondedOfferBondedVestingScheduleIds);
    }
  }, [selectedAccount, parachainApi]);

  useEffect(() => {
    if (selectedAccount && parachainApi) {
      fetchVestingSchedulesByBondOffers(
        parachainApi,
        selectedAccount.address,
        bondOffers,
        bondedOfferVestingScheduleIds
      ).then(putBondedOfferVestingSchedules);
    }
  }, [
    selectedAccount,
    parachainApi,
    bondOffers,
    bondedOfferVestingScheduleIds,
  ]);

  return null;
};

export default Updater;
