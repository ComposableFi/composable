import { StoreSlice } from "../types";
import { addActiveBond, addBond } from "./bonds.reducers";
import { BondOffer, BondSlice, VestingSchedule } from "./bonds.types";

const createBondsSlice: StoreSlice<BondSlice> = (set) => ({
  allBonds: [],
  activeBonds: [],
  addActiveBond: (
    bondOffer: BondOffer,
    vestingSchedule: VestingSchedule,
    currentBlock: number,
    currentTime: number
  ) =>
    set((prev: BondSlice) => ({
      activeBonds: addActiveBond(
        prev.activeBonds,
        bondOffer,
        vestingSchedule,
        currentBlock,
        currentTime
      ),
    })),
  addBond: (
    bondOffer: BondOffer,
    principalAppoloPriceInUSD: number,
    rewardAppoloPriceInUSD: number
  ) =>
    set((prev: BondSlice) => ({
      allBonds: addBond(
        prev.allBonds,
        bondOffer,
        principalAppoloPriceInUSD,
        rewardAppoloPriceInUSD
      ),
    })),
  reset: () =>
    set(() => ({
      allBonds: [],
      activeBonds: [],
    })),
});

export default createBondsSlice;
