import BigNumber from "bignumber.js";
import useStore from "../../store/useStore";
import { useEffect } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { decodeBondOffer } from "./decodeBondOffer";
import { DEFAULT_DECIMALS, DEFAULT_NETWORK_ID } from "../constants";
import { decodeVestingSchedule } from "./decodeVestingSchedule";
import { stringToBigNumber } from "../../utils/stringToBigNumber";
import { fetchApolloPriceByAssetId } from "../../utils/defi/apollo";

/**
 * Updates zustand store with all bonds from bondedFinance pallet
 * @returns null
 */
const Updater = () => {
  const { addBond, addActiveBond, reset } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (parachainApi && selectedAccount) {
      parachainApi.query.bondedFinance
        ?.bondOfferCount()
        .then(async (offerCount) => {
          const _offerCount = new BigNumber(offerCount.toString());

          let offerPromises = [];
          for (let i = 1; i <= _offerCount.toNumber(); i++) {
            offerPromises.push(parachainApi.query.bondedFinance.bondOffers(i));
          }

          const bonds = await Promise.all(offerPromises);

          bonds.map(async (bond, index) => {
            const [beneficiary, bondOffer] = bond.toHuman() as any;
            const principalCurrencyId = stringToBigNumber(
              bondOffer.asset
            ).toNumber();
            const rewardCurrencyId = stringToBigNumber(
              bondOffer.reward.asset
            ).toNumber();
            const [vestingSchedule] = (await (
              await parachainApi.query.vesting.vestingSchedules(
                selectedAccount.address,
                principalCurrencyId
              )
            ).toHuman()) as any;
            const principalApolloPrice = await fetchApolloPriceByAssetId(
              parachainApi,
              principalCurrencyId
            );
            const rewardApolloPrice = await fetchApolloPriceByAssetId(
              parachainApi,
              rewardCurrencyId
            );
            const principalAppoloPriceInUSD =
              stringToBigNumber(principalApolloPrice).div(DEFAULT_DECIMALS);
            const rewardAppoloPriceInUSD =
              stringToBigNumber(rewardApolloPrice).div(DEFAULT_DECIMALS);
            const decodedBondOffer = decodeBondOffer(
              index + 1,
              beneficiary,
              bondOffer
            );
            const decodedVestingSchedule = vestingSchedule
              ? decodeVestingSchedule(vestingSchedule)
              : null;
            const currentBlock = Number(
              (await parachainApi.query.system.number()).toString()
            );
            const currentTime = Number(
              (await parachainApi.query.timestamp.now()).toString()
            );
            if (decodedVestingSchedule) {
              addActiveBond(
                decodedBondOffer,
                decodedVestingSchedule,
                currentBlock,
                currentTime
              );
            }
            addBond(
              decodedBondOffer,
              principalAppoloPriceInUSD.toNumber(),
              rewardAppoloPriceInUSD.toNumber()
            );
          });
        });
    }
  }, [parachainApi, selectedAccount]);

  useEffect(() => {
    reset();
  }, [selectedAccount]);

  return null;
};

export default Updater;
