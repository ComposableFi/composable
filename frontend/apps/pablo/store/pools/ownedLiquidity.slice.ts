import { StoreSlice } from "../types";
import { LPTokenId, LPTokenState, OwnedLiquiditySlice } from "./types";
import BigNumber from "bignumber.js";
import { Asset } from "shared";

const NullLiquidity: LPTokenState = {
  pair: [
    new Asset("", "", "", "pica", undefined),
    new Asset("", "", "", "pica", undefined),
  ],
  poolId: new BigNumber(0),
  balance: {
    free: new BigNumber(0),
    locked: new BigNumber(0),
  },
};
const createOwnedLiquiditySLice: StoreSlice<OwnedLiquiditySlice> = (set) => ({
  ownedLiquidity: {
    tokens: {},
    isLoaded: false,
    setOwnedLiquidity: (lpTokenId: LPTokenId, balance, pair, poolId) => {
      set((state) => {
        state.ownedLiquidity.tokens[lpTokenId] = { ...NullLiquidity };
        state.ownedLiquidity.tokens[lpTokenId].balance = balance;
        state.ownedLiquidity.tokens[lpTokenId].pair = pair;
        state.ownedLiquidity.tokens[lpTokenId].poolId = poolId;
        state.ownedLiquidity.isLoaded = true;
      });
    },
  },
});

export default createOwnedLiquiditySLice;
