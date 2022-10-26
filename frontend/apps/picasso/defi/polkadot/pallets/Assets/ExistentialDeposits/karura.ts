import { TokenId } from "tokens";
import BigNumber from "bignumber.js";

// @see https://wiki.acala.network/get-started/get-started/karura-account
const karuraExistentialDeposits: {
    [key in string] : BigNumber
} = {
    "kusd": new BigNumber(0.01),
    "ausd": new BigNumber(0.01),
    "kar": new BigNumber(0.1),
    "ksm": new BigNumber(0.0001),
}


export const getKaruraExistentialDeposit = (tokenId: TokenId): BigNumber => {
    return tokenId in karuraExistentialDeposits ? karuraExistentialDeposits[tokenId] : new BigNumber(1)
}
