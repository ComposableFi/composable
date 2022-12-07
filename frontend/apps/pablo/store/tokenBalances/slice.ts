import { StoreSlice } from "../types";
import { TOKENS, TokenId } from "tokens";
import { SubstrateNetworkId, SubstrateNetworks } from "shared";
import { TokenBalancesSlice } from "./types";
import BigNumber from "bignumber.js";

const createTokenBalancesSlice: StoreSlice<TokenBalancesSlice> = (set) => ({
  substrateBalances: {
    tokenBalances: SubstrateNetworks.reduce((networkBalances, networkId) => {
        const networkTokens = Object.keys(TOKENS).reduce((tokenBalances, currToken) => {
            tokenBalances[currToken as TokenId] = {
                locked: new BigNumber(0),
                free: new BigNumber(0)
            }
            return tokenBalances;
        }, {} as Record<TokenId, {
            locked: BigNumber;
            free: BigNumber;            
        }>);
        networkBalances[networkId] = networkTokens
        return networkBalances;
    }, {} as Record<SubstrateNetworkId, Record<TokenId, {
        locked: BigNumber;
        free: BigNumber;
    }>>),
    setTokenBalance: (tokenId, network, free, locked) => {
      set((state) => {
        state.substrateBalances.tokenBalances[network][tokenId].free = free;
        state.substrateBalances.tokenBalances[network][tokenId].locked = locked;
        return state;
      });
    },
  },
});

export default createTokenBalancesSlice;
