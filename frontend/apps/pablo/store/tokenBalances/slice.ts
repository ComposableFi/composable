import { StoreSlice } from "../types";
import { TokenId, TOKENS } from "tokens";
import { SubstrateNetworkId, SubstrateNetworks } from "shared";
import { TokenBalancesSlice } from "./types";
import BigNumber from "bignumber.js";
import { option } from "fp-ts";
import { pipe } from "fp-ts/lib/function";

const createTokenBalancesSlice: StoreSlice<TokenBalancesSlice> = (
  set,
  get
) => ({
  substrateBalances: {
    tokenBalances: SubstrateNetworks.reduce(
      (networkBalances, networkId) => {
        const networkTokens = Object.keys(TOKENS).reduce(
          (tokenBalances, currToken) => {
            tokenBalances[currToken as TokenId] = {
              locked: new BigNumber(0),
              free: new BigNumber(0),
            };
            return tokenBalances;
          },
          {} as Record<
            TokenId,
            {
              locked: BigNumber;
              free: BigNumber;
            }
          >
        );
        networkBalances[networkId] = networkTokens;
        return networkBalances;
      },
      {} as Record<
        SubstrateNetworkId,
        Record<
          TokenId,
          {
            locked: BigNumber;
            free: BigNumber;
          }
        >
      >
    ),
    setTokenBalance: (tokenId, network, free, locked) => {
      set((state) => {
        state.substrateBalances.tokenBalances[network][tokenId].free = free;
        state.substrateBalances.tokenBalances[network][tokenId].locked = locked;
        return state;
      });
    },
    getTokenBalance: (tokenId, network) => {
      return pipe(
        Object.values(get().substrateTokens.tokens).find(
          (token) => token.getIdOnChain(network) === tokenId
        ),
        option.fromNullable,
        option.map(
          (asset) =>
            get().substrateBalances.tokenBalances[network][asset.getTokenId()]
        ),
        option.fold(
          () => ({
            free: new BigNumber(0),
            locked: new BigNumber(0),
          }),
          (a) => a
        )
      );
    },
  },
});

export default createTokenBalancesSlice;
