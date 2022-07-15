export {}
// async function calculatePriceImpactLBP(
//   api: ApiPromise,
//   poolConstants: SwapsSlice["swaps"]["poolConstants"],
//   exchange: SwapMetadata,
//   tokenOutAmount: BigNumber,
//   baseToQuotePrice: BigNumber,
//   quoteToBasePrice: BigNumber
// ): Promise<BigNumber> {
//   try {
//     if (!poolConstants.lbpConstants) throw new Error("Might not be an LBP");

//     const { poolAccountId, pair } = poolConstants;
//     const { end, start, initialWeight, finalWeight } =
//       poolConstants.lbpConstants;
//     const baseAssetReserve = await fetchBalanceByAssetId(
//       api,
//       poolAccountId,
//       pair.base.toString()
//     );
//     const quoteAssetReserve = await fetchBalanceByAssetId(
//       api,
//       poolAccountId,
//       pair.quote.toString()
//     );
//     let baseAssetReserveBn = new BigNumber(baseAssetReserve);
//     let quoteAssetReserveBn = new BigNumber(quoteAssetReserve);

//     const { quoteAmount } = exchange;
//     const cb = await api.query.system.number();
//     const current_block = await cb.toNumber();
//     let one = new BigNumber(1);
//     let pointInSale = new BigNumber(current_block).div(end - start);
//     let weightRange = new BigNumber(initialWeight)
//       .div(100)
//       .minus(new BigNumber(finalWeight).div(100));
//     let baseWeight = new BigNumber(initialWeight)
//       .div(100)
//       .minus(pointInSale.times(weightRange));
//     let quoteWeight = one.minus(baseWeight);

//     if (exchange.side === "quote") {
//       let num = quoteAssetReserveBn.plus(quoteAmount).div(quoteWeight);
//       let denom = baseAssetReserveBn.minus(tokenOutAmount).div(baseWeight);
//       let price = num.div(denom);
//       return new BigNumber(1).minus(
//         new BigNumber(
//           /** Need to confirm which price to send here */
//           quoteToBasePrice
//         ).div(price)
//       );
//     } else {
//       let num = quoteAssetReserveBn.plus(tokenOutAmount).div(quoteWeight);
//       let denom = baseAssetReserveBn.minus(quoteAmount).div(baseWeight);
//       let price = num.div(denom);
//       return new BigNumber(1).minus(
//         new BigNumber(
//           /** Need to confirm which price to send here */
//           baseToQuotePrice
//         ).div(price)
//       );
//     }
//   } catch (err: any) {
//     console.error(err);
//     return new BigNumber(0);
//   }
// }