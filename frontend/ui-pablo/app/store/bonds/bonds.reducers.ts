import { BondOffer } from "@/defi/types";
import produce from "immer";
import { BondSlice } from "./bonds.types";

// export const addActiveBond = (
//   activeBonds: BondSlice["activeBonds"],
//   bondOffer: BondOffer,
//   vestingSchedule: VestingSchedule,
//   currentBlock: number,
//   currentTime: number
// ) => {
//   return produce(activeBonds, (draft) => {
//     const window = vestingSchedule.window;
//     const totalPeriod = window.period * vestingSchedule.periodCount;
//     const lastBlockOrMoment = window.start + totalPeriod;
//     const vestingTime =
//       vestingSchedule.type === "block"
//         ? fetchBlockBasedVestingTime({
//             start: window.start,
//             lastBlock: lastBlockOrMoment,
//             currentBlock,
//           })
//         : fetchMomentBasedVestingTime({
//             start: window.start,
//             lastMoment: lastBlockOrMoment,
//             currentTime,
//           });
//     const claimableAmount = fetchClaimable({
//       currentBlockOrMoment:
//         vestingSchedule.type === "block" ? currentBlock : currentTime,
//       start: window.start,
//       periodCount: vestingSchedule.periodCount,
//       perPeriod: vestingSchedule.perPeriod,
//       lastBlockOrMoment,
//     });
//     draft.push({
//       offerId: bondOffer.offerId,
//       asset: bondOffer.asset,
//       pendingAmount: vestingSchedule.perPeriod
//         .times(vestingSchedule.periodCount)
//         .minus(claimableAmount),
//       claimableAmount,
//       vestingTime,
//       bondOffer,
//     });
//   });
// };

// export const addBond = (
//   allBonds: BondSlice["allBonds"],
//   bondOffer: BondOffer,
//   principalAppoloPriceInUSD: number,
//   rewardAppoloPriceInUSD: number
// ) => {
//   const price = fromChainUnits(
//     new BigNumber(bondOffer.bondPrice)
//       .times(principalAppoloPriceInUSD)
//       .toString()
//   );
//   return produce(allBonds, (draft) => {
//     draft.push({
//       offerId: bondOffer.offerId,
//       asset: bondOffer.asset,
//       price,
//       roi: fromChainUnits(
//         new BigNumber(rewardAppoloPriceInUSD)
//           .times(bondOffer.reward.amount)
//           .times(100)
//           .toString()
//       ).div(price),
//       totalPurchased: new BigNumber(0), // TBD
//       bondOffer,
//     });
//   });
// };


export const putBondOffers = (
  bondOffersState: BondSlice["bondOffers"],
  bondOffers: BondOffer[],
) => {
  return produce(bondOffersState, (draft) => {
    draft.list = [...bondOffers]
  })
}