import BigNumber from "bignumber.js";
import { SubstrateNetworkId } from "shared";
import { TokenId } from "tokens";

export interface TokenBalancesSlice {
    substrateBalances: {
        tokenBalances: Record<SubstrateNetworkId, Record<TokenId, {
            locked: BigNumber;
            free: BigNumber;
        }>>;
        setTokenBalance: (
            tokenId: TokenId,
            network: SubstrateNetworkId,
            free: BigNumber,
            locked: BigNumber
        ) => void;
    }
}
