import { useEffect } from "react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import {
  calculateBondROI,
  calculateVestingState,
  DEFAULT_NETWORK_ID,
  fetchBondOffers,
  fetchVestingSchedulesByBondOffers,
} from "@/defi/utils";
import {
  putBondedOfferBondedVestingScheduleIds,
  putBondedOfferVestingSchedules,
  putBondedOfferVestingState,
  putBondOffers,
  putBondOffersReturnOnInvestmentRecord,
  putBondOffersTotalPurchasedCount,
  resetBondedOfferVestingState,
  useBondOffersSlice,
} from "@/store/bond/bond.slice";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import {
  fetchTotalPurchasedBondsByOfferIds,
  extractUserBondedFinanceVestingScheduleAddedEvents,
} from "@/defi/subsquid/bonds/helpers";
import { useBlockInterval } from "@/defi/hooks";
import useBlockNumber from "@/defi/hooks/useBlockNumber";
import { AVERAGE_BLOCK_TIME } from "@/defi/utils/constants";

const Updater = () => {
  const { apollo } = useStore();
  const {
    bondOffers,
    bondedOfferVestingScheduleIds,
    bondedOfferVestingSchedules,
  } = useBondOffersSlice();

  useEffect(() => {
    const roiRecord = bondOffers.reduce((acc, bondOffer) => {
      const principalAssetPrinceInUSD: BigNumber =
        new BigNumber(apollo[bondOffer.asset]) || new BigNumber(0);
      const rewardAssetPriceInUSD =
        new BigNumber(apollo[bondOffer.reward.asset]) || new BigNumber(0);
      const rewardAssetAmountPerBond = bondOffer.reward.amount.div(
        bondOffer.nbOfBonds
      );
      const principalAssetAmountPerBond: BigNumber = bondOffer.bondPrice;
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
   * Get bond offers ids and
   * vesting schedule
   * id map from subsquid e.g { "1": ["1","2"] }
   */
  useEffect(() => {
    if (selectedAccount && parachainApi) {
      extractUserBondedFinanceVestingScheduleAddedEvents(
        parachainApi,
        selectedAccount.address
      ).then(putBondedOfferBondedVestingScheduleIds);
    }
  }, [selectedAccount, parachainApi]);
  /**
   * fetch vesting schedules using
   * the map if any matching ids found
   * from the chain data structure:
   * { "1": VestingSchedule[] }
   */
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

  const blockInterval = useBlockInterval();
  const blockNumber = useBlockNumber(DEFAULT_NETWORK_ID);
  /**
   * Calculate and store vesting related
   * values shown on UI
   */
  useEffect(() => {
    if (Object.keys(bondedOfferVestingSchedules).length > 0) {
      let vestingState = calculateVestingState(
        blockNumber,
        new BigNumber(blockInterval?.toString() ?? AVERAGE_BLOCK_TIME),
        bondedOfferVestingSchedules
      );
      putBondedOfferVestingState(vestingState);
    } else {
      resetBondedOfferVestingState();
    }
  }, [blockInterval, blockNumber, bondedOfferVestingSchedules]);

  return null;
};

export default Updater;
