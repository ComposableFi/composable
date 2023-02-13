import { AllSlices } from "@/stores/types";
import BigNumber from "bignumber.js";

export const getClaimableAmount = (store: AllSlices) =>
  Object.values(store.claimableRewards).reduce((acc, currentInstance) => {
    return acc.plus(
      currentInstance.reduce((balances, currentAsset) => {
        if (currentAsset.assetId === "1") {
          return balances.plus(currentAsset.balance);
        }
        return balances;
      }, new BigNumber(0))
    );
  }, new BigNumber(0));
