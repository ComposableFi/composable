import { useEffect } from "react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { calculateBondROI } from "@/defi/utils";

const Updater = () => {
  const { bondOffers: { list }, putBondOfferROI, apollo } = useStore();

  useEffect(() => {
    const roiRecord = list.reduce((acc, bondOffer) => {
      const principalAssetPrinceInUSD = new BigNumber(apollo[bondOffer.asset]) || new BigNumber(0);
      const rewardAssetPriceInUSD = new BigNumber(apollo[bondOffer.reward.asset]) || new BigNumber(0);
      const rewardAssetAmountPerBond = bondOffer.reward.amount.div(bondOffer.nbOfBonds);
      const principalAssetAmountPerBond = bondOffer.bondPrice;
      return {
        ...acc,
        [bondOffer.offerId.toString()]: calculateBondROI(
          principalAssetPrinceInUSD,
          rewardAssetPriceInUSD,
          principalAssetAmountPerBond,
          rewardAssetAmountPerBond
        )
      }
    }, {} as Record<string, BigNumber>);

    putBondOfferROI(roiRecord);
  }, [apollo, list, putBondOfferROI])

  return null;
};

export default Updater;
