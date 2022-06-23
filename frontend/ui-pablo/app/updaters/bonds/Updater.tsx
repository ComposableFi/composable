import useStore from "../../store/useStore";
import { useEffect, useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { fetchBondOffers } from "@/defi/utils/";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

/**
 * Updates zustand store with all bonds from bondedFinance pallet
 * @returns null
 */
const Updater = () => {
  // const { putBondOffers } = useStore();
  // const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  // const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  // useEffect(() => {
  //     if (parachainApi) {
  //       fetchBondOffers(parachainApi).then(decodedOffers => {
  //         putBondOffers(decodedOffers)
  //       })
  //     }
  //   }, [parachainApi, putBondOffers])

  // useEffect(() => {
  //   if (parachainApi && selectedAccount) {
  //     fetchBonds(parachainApi).then((bonds) => {
  //       bonds.map(async (bond, index) => {
  //         try {
  //           const [beneficiary, bondOffer] = bond.unwrap();
  //           const principalCurrencyId = bondOffer.asset.toString();
  //           const principalAppoloPriceInUSD = await fetchApolloPriceByAssetId(
  //             parachainApi,
  //             principalCurrencyId
  //           );
  //           const rewardAppoloPriceInUSD = await fetchApolloPriceByAssetId(
  //             parachainApi,
  //             bondOffer.reward.asset.toString()
  //           );

  //           const vestingSchedule = await fetchVestingSchedule(
  //             parachainApi,
  //             selectedAccount.address,
  //             principalCurrencyId
  //           );
  //           const lpTokenPair = getLPTokenPair(
  //             lpRewardingPools as ConstantProductPool[],
  //             principalCurrencyId
  //           );
  //           const principalAsset = lpTokenPair
  //             ? {
  //                 base: getToken(getTokenId(lpTokenPair.base)),
  //                 quote: getToken(getTokenId(lpTokenPair.quote)),
  //               }
  //             : getToken(getTokenId(bondOffer.asset.toNumber()));
  //           const decodedBondOffer = decodeBondOffer(
  //             index + 1,
  //             beneficiary,
  //             bondOffer.toHuman(),
  //             principalAsset
  //           );
  //           const decodedVestingSchedule = vestingSchedule
  //             ? decodeVestingSchedule(vestingSchedule.toJSON())
  //             : null;
  //           const currentBlock = await getCurrentBlock(parachainApi);
  //           const currentTime = await getCurrentTime(parachainApi);
  //           if (decodedVestingSchedule) {
  //             addActiveBond(
  //               decodedBondOffer,
  //               decodedVestingSchedule,
  //               currentBlock,
  //               currentTime
  //             );
  //           }
  //           console.log(decodeBondOffer)
  //           addBond(
  //             decodedBondOffer,
  //             new BigNumber(principalAppoloPriceInUSD).toNumber(),
  //             (new BigNumber(rewardAppoloPriceInUSD)).toNumber()
  //           );
  //         } catch (ex) {
  //           console.error(ex)
  //           return null;
  //         }
  //       });
  //     });
  //   }
  //   // eslint-disable-next-line react-hooks/exhaustive-deps
  // }, [parachainApi, selectedAccount]);

  // useEffect(() => {
    // reset();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  // }, [selectedAccount]);

  return null;
};

export default Updater;
