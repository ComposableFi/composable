import { useEffect } from "react";
import useStore from "@/store/useStore";
import { fetchTotalPurchasedBondsByOfferIds } from "@/defi/subsquid/bonds/helpers";
import { calculateBondROI } from "@/defi/utils/bonds/math";
import BigNumber from "bignumber.js";

const Updater = () => {
  const { setBondOfferTotalPurchased, apollo, bondOffers, putBondOfferROI } =
    useStore();
  const { list } = bondOffers;

  useEffect(() => {
    fetchTotalPurchasedBondsByOfferIds().then(setBondOfferTotalPurchased);
  }, [setBondOfferTotalPurchased]);

  useEffect(() => {
    const bondOfferROIMap = list.reduce((acc, bondOffer) => {
      return {
        ...acc,
        [bondOffer.offerId.toString()]: calculateBondROI(
          new BigNumber(apollo[bondOffer.asset] || 0),
          new BigNumber(apollo[bondOffer.reward.asset.toString()] || 0),
          bondOffer.bondPrice,
          bondOffer.reward.amount.div(bondOffer.nbOfBonds)
        ),
      };
    }, {} as Record<string, BigNumber>);

    putBondOfferROI(bondOfferROIMap);
  }, [apollo, list, putBondOfferROI]);

  return null;
};

export default Updater;
