import useStore from "../../store/useStore";
import { useEffect, useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { decodeBondOffer } from "./decodeBondOffer";

import { decodeVestingSchedule } from "./decodeVestingSchedule";
import { getAppoloPriceInUSD } from "../../utils/defi/apollo";
import { fetchBonds } from "./fetchBonds";
import { fetchVestingSchedule } from "./fetchVestingSchedule";
import { getToken, getTokenId } from "../../defi/Tokens";
import { getCurrentBlock } from "../../utils/defi/getCurrentBlock";
import { getCurrentTime } from "../../utils/defi/getCurrentTime";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

/**
 * Updates zustand store with all bonds from bondedFinance pallet
 * @returns null
 */
const Updater = () => {
  const { pools, addBond, addActiveBond, reset } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const constantProductPool = useMemo(
    () => [
      ...pools.constantProductPools.verified,
      ...pools.constantProductPools.unVerified,
    ],
    [pools]
  );

  useEffect(() => {
    if (parachainApi && selectedAccount) {
      fetchBonds(parachainApi).then((bonds) => {
        bonds.map(async (bond, index) => {
          try {
            const [beneficiary, bondOffer] = bond.unwrap();
            const principalCurrencyId = bondOffer.asset.toString();
            const principalAppoloPriceInUSD = await getAppoloPriceInUSD(
              parachainApi,
              principalCurrencyId
            );
            const rewardAppoloPriceInUSD = await getAppoloPriceInUSD(
              parachainApi,
              bondOffer.reward.asset.toString()
            );

            const vestingSchedule = await fetchVestingSchedule(
              parachainApi,
              selectedAccount.address,
              principalCurrencyId
            );
            // const lpTokenPair = getLPTokenPair(
            //   constantProductPool,
            //   principalCurrencyId
            // );
            // const principalAsset = lpTokenPair
            //   ? {
            //       base: getToken(getTokenId(lpTokenPair.base)),
            //       quote: getToken(getTokenId(lpTokenPair.quote)),
            //     }
            //   : getToken(getTokenId(bondOffer.asset.toNumber()));
            // const decodedBondOffer = decodeBondOffer(
            //   index + 1,
            //   beneficiary,
            //   bondOffer,
            //   principalAsset
            // );
            // const decodedVestingSchedule = vestingSchedule
            //   ? decodeVestingSchedule(vestingSchedule)
            //   : null;
            // const currentBlock = await getCurrentBlock(parachainApi);
            // const currentTime = await getCurrentTime(parachainApi);
            // if (decodedVestingSchedule) {
            //   addActiveBond(
            //     decodedBondOffer,
            //     decodedVestingSchedule,
            //     currentBlock,
            //     currentTime
            //   );
            // }
            // addBond(
            //   decodedBondOffer,
            //   principalAppoloPriceInUSD.toNumber(),
            //   rewardAppoloPriceInUSD.toNumber()
            // );
          } catch (ex) {
            return null;
          }
        });
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [parachainApi, selectedAccount]);

  useEffect(() => {
    reset();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedAccount]);

  return null;
};

export default Updater;
