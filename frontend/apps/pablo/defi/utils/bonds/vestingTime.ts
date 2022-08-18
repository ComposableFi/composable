import BigNumber from "bignumber.js";
import moment from "moment";

export function calculateVestingTime(
    maturity: BigNumber,
    blockInterval: BigNumber
) {
    const duration = maturity.times(blockInterval);
    return moment.utc(duration.toNumber()).format("HH:mm:ss")
}