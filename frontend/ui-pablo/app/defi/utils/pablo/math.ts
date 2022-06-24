import BigNumber from "bignumber.js";

export function uniswapCalculator(
    sideUpdated: "base" | "quote",
    isInverse: boolean,
    tokenAmount: BigNumber,
    oneBaseInQuote: BigNumber,
    slippage: number, // in %
    feeRate: number // in %
): {
    tokenOutAmount: BigNumber,
    feeChargedAmount: BigNumber,
    slippageAmount: BigNumber,
    minReceive: BigNumber,
} {
    let tokenOutAmount = new BigNumber(0);

    let oneQuoteInBase = new BigNumber(1).div(oneBaseInQuote);
    if (isInverse) {
        let quoteInBase = new BigNumber(oneQuoteInBase)
        oneQuoteInBase = new BigNumber(oneBaseInQuote)
        oneBaseInQuote = new BigNumber(quoteInBase)
    }

    tokenOutAmount = sideUpdated === "base" ? tokenAmount.times(oneBaseInQuote) : tokenAmount.times(oneQuoteInBase)

    let slippageAmount = new BigNumber(0);
    slippageAmount = sideUpdated === "base" ?
        tokenOutAmount.times(new BigNumber(slippage).div(100)) :
        tokenAmount.times(new BigNumber(slippage).div(100))

    let feeChargedAmount = new BigNumber(0);
    feeChargedAmount = sideUpdated === "base" ?
        tokenOutAmount.times(new BigNumber(feeRate).div(100)) :
        tokenAmount.times(new BigNumber(feeRate).div(100))

    let minReceive = new BigNumber(0);
    if (sideUpdated === "base") {
        minReceive = tokenOutAmount.minus(feeChargedAmount.plus(slippageAmount)).times(oneQuoteInBase);
    } else {
        minReceive = tokenAmount.minus(feeChargedAmount.plus(slippageAmount)).times(oneQuoteInBase);
    }

    return {
        feeChargedAmount,
        slippageAmount,
        tokenOutAmount,
        minReceive
    }
}