import { uniswapCalculator } from "@/defi/utils";
import BigNumber from "bignumber.js";

let oneBaseInQuote = new BigNumber(2);
let slippage = new BigNumber(1);
let fee = new BigNumber(1);

describe("SwapMath", () => {
  /**
   * USD = Quote
   * KSM = Base
   * 1 KSM = 2 USD
   * 1 USD = 0.5 KSM
   * Slippage = 2%
   * Fee Rate = 1%
   * Changed KSM = 5
   * Fee will be charged on USD
   * Fee: 10 * 0.01 = 0.1
   * Slippage: 10 * 0.02 = 0.2
   * Min Receive =  10 - (0.02+0.01) = 9.7 * 0.5 = 4.85
   * Token Out = 5 * 2 = 10 USD
   */
   test("Change Base KSM to 5, USD should change to 10", () => {
    let ksmAmount = new BigNumber(5);

    const {
      minReceive,
      slippageAmount,
      feeChargedAmount,
      tokenOutAmount
    } = uniswapCalculator("base", false, ksmAmount, oneBaseInQuote, new BigNumber(2).toNumber(), fee.toNumber());

    expect(minReceive.toString()).toEqual("4.85");
    expect(tokenOutAmount.toString()).toEqual("10");
    expect(feeChargedAmount.toString()).toEqual("0.1");
    expect(slippageAmount.toString()).toEqual("0.2");
  });
  /**
   * USD = Quote
   * KSM = Base
   * 1 KSM = 2 USD
   * 1 USD = 0.5 KSM
   * Slippage = 1%
   * Fee Rate = 1%
   * Changed KSM = 1
   * Fee will be charged on USD
   * Fee: 1 * 2 * 0.01 = 0.02
   * Slippage: 1 * 2 * 0.01 = 0.02
   * Min Receive =  2 - (0.02+0.02) = 1.96 * 0.5 = 0.98 KSM
   * Token Out = 1 * 2 = 2 USD
   */
  test("Change Base KSM to 1, USD should change to 2", () => {
    let ksmAmount = new BigNumber(1);

    const {
      minReceive,
      slippageAmount,
      feeChargedAmount,
      tokenOutAmount
    } = uniswapCalculator("base", false, ksmAmount, oneBaseInQuote, slippage.toNumber(), fee.toNumber());

    expect(minReceive.toString()).toEqual("0.98");
    expect(tokenOutAmount.toString()).toEqual("2");
    expect(feeChargedAmount.toString()).toEqual("0.02");
    expect(slippageAmount.toString()).toEqual("0.02");
  });
  /**
 * USD = Quote
 * KSM = Base
 * 1 KSM = 2 USD
 * 1 USD = 0.5 KSM
 * Slippage = 1%
 * Fee Rate = 1%
 * Changed USD = 2
 * Fee will be charged on USD
 * Fee: 2 * 0.01 = 0.02
 * Slippage: 2 * 0.01 = 0.02
 * Min Receive =  2 - (0.02+0.02) = 1.96 * 0.5 = 0.96 KSM
 * Token Out = 2 * 0.5 = 1 KSM
 */
  test("Change Base USD to 2, KSM should change to 1", () => {
    let usdAmount = new BigNumber(2);

    const {
      minReceive,
      slippageAmount,
      feeChargedAmount,
      tokenOutAmount
    } = uniswapCalculator("quote", false, usdAmount, oneBaseInQuote, slippage.toNumber(), fee.toNumber());

    expect(minReceive.toString()).toEqual("0.98");
    expect(tokenOutAmount.toString()).toEqual("1");
    expect(feeChargedAmount.toString()).toEqual("0.02");
    expect(slippageAmount.toString()).toEqual("0.02");
  });
  /**
   * USD = Quote
   * KSM = Base
   * 1 KSM = 2 USD
   * 1 USD = 0.5 KSM
   * Slippage = 1%
   * Fee Rate = 1%
   * Changed USD = 2
   * Fee will be charged on KSM
   * Fee: 2 * 0.5 * 0.01 = 0.01
   * Slippage: 2 * 0.5 * 0.01 = 0.01
   * Min Receive =  1 - (0.01+0.01) = 0.98 * 2 
   * Token Out = 2 * 1 = 1
   */
  test("Change Inverted USD to 2, Expect 1 KSM", () => {
    let usdAmount = new BigNumber(2);

    const {
      minReceive,
      slippageAmount,
      feeChargedAmount,
      tokenOutAmount
    } = uniswapCalculator("base", true, usdAmount, oneBaseInQuote, slippage.toNumber(), fee.toNumber());

    expect(minReceive.toString()).toEqual("1.96");
    expect(tokenOutAmount.toString()).toEqual("1");
    expect(feeChargedAmount.toString()).toEqual("0.01");
    expect(slippageAmount.toString()).toEqual("0.01");
  });
  /**
 * KSM = Quote
 * USD = Base
 * 1 KSM = 2 USD, 1 USD = 0.5 KSM
 * Slippage = 1%
 * Fee Rate = 1%
 * Changed KSM = 1
 * Fee will be charged on KSM
 * Fee: 1 * 0.01 = 0.01
 * Slippage: 1 * 0.01 = 0.01
 * Min Receive =  1 - (0.01+0.01) = 0.98 * 2 = 1.96 USD
 * Token Out = 1 * 2 = 2
 */
  test("Change Inverted KSM to 1, Expect 2 USD", () => {
    let ksmAmount = new BigNumber(1);

    const {
      minReceive,
      slippageAmount,
      feeChargedAmount,
      tokenOutAmount
    } = uniswapCalculator("quote", true, ksmAmount, oneBaseInQuote, slippage.toNumber(), fee.toNumber());

    expect(minReceive.toString()).toEqual("1.96");
    expect(tokenOutAmount.toString()).toEqual("2");
    expect(feeChargedAmount.toString()).toEqual("0.01");
    expect(slippageAmount.toString()).toEqual("0.01");
  });
});
