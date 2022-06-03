import BigNumber from "bignumber.js";
import useStore from "../../useStore";
import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { decodeBondOffer } from "./decodeBondOffer";

/**
 * Updates zustand store with all bonds from bondedFinance pallet
 * @returns null
 */
const Updater = () => {
  const { setAllBonds } = useStore();
  const { parachainApi } = useParachainApi("picasso");

  useEffect(() => {
    if (parachainApi) {
      parachainApi.query.bondedFinance
        ?.bondOfferCount()
        .then(async (offerCount) => {
          const _offerCount = new BigNumber(offerCount.toString());

          let offerPromises = [];
          for (let i = 1; i <= _offerCount.toNumber(); i++) {
            offerPromises.push(parachainApi.query.bondedFinance.bondOffers(i));
          }

          const bonds = await Promise.all(offerPromises);
          bonds.map((bond) => {
            const [beneficiary, bondOffer] = bond.toHuman() as any;
            const decodedBondOffer = decodeBondOffer(beneficiary, bondOffer);
            /*TODO : We need to match beneficiary with the connected account's id to set up active bonds */
            /* setActiveBonds(); */
            setAllBonds(decodedBondOffer);
          });
        });
    }
  }, [parachainApi]);

  return null;
};

export default Updater;
