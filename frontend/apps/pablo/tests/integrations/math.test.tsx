import { calculator } from "@/defi/utils";
import BigNumber from "bignumber.js";

let oneBaseInQuote = new BigNumber(2);
let slippage = new BigNumber(1);
let fee = new BigNumber(1);

describe("SwapMath", () => {
  /**
    USD
    KSM
    1 KSM = 2 USD
   */
  test("Change USD to 2", () => {
    let usdAmount = new BigNumber(2);
    const { minReceive, slippageAmount, feeChargedAmount, tokenOutAmount } =
      calculator(
        "quote",
        usdAmount,
        oneBaseInQuote,
        slippage.toNumber(),
        fee.toNumber()
      );

    expect(minReceive.toString()).toEqual("0.98");
    expect(tokenOutAmount.toString()).toEqual("1");
    expect(feeChargedAmount.toString()).toEqual("0.02");
    expect(slippageAmount.toString()).toEqual("0.02");
  });
  /**
    USD
    KSM
    1 KSM = 2 USD
   */
  test("Change USD to 4", () => {
    let usdAmount = new BigNumber(4);
    const { minReceive, slippageAmount, feeChargedAmount, tokenOutAmount } =
      calculator(
        "quote",
        usdAmount,
        oneBaseInQuote,
        slippage.toNumber(),
        fee.toNumber()
      );

    expect(minReceive.toString()).toEqual("1.96");
    expect(tokenOutAmount.toString()).toEqual("2");
    expect(feeChargedAmount.toString()).toEqual("0.04");
    expect(slippageAmount.toString()).toEqual("0.04");
  });
  /**
    USD
    KSM
    1 KSM = 2 USD
   */
  test("Change KSM to 1", () => {
    let ksmAmount = new BigNumber(1);

    const { minReceive, slippageAmount, feeChargedAmount, tokenOutAmount } =
      calculator(
        "base",
        ksmAmount,
        oneBaseInQuote,
        slippage.toNumber(),
        fee.toNumber()
      );

    expect(tokenOutAmount.toString()).toEqual("2");
    expect(feeChargedAmount.toString()).toEqual("0.02");
    expect(slippageAmount.toString()).toEqual("0.02");
    expect(minReceive.toString()).toEqual("0.98");
  });
  /**
    USD
    KSM
    1 KSM = 2 USD
   */
  test("Change KSM to 2", () => {
    let ksmAmount = new BigNumber(2);

    const { minReceive, slippageAmount, feeChargedAmount, tokenOutAmount } =
      calculator(
        "base",
        ksmAmount,
        oneBaseInQuote,
        slippage.toNumber(),
        fee.toNumber()
      );

    expect(tokenOutAmount.toString()).toEqual("4");
    expect(feeChargedAmount.toString()).toEqual("0.04");
    expect(slippageAmount.toString()).toEqual("0.04");
    expect(minReceive.toString()).toEqual("1.96");
  });

  /**
    KSM
    USD
    1 USD = 0.5 KSM
   */
  test("Change KSM to 1", () => {
    let ksmAmount = new BigNumber(1);

    const { minReceive, slippageAmount, feeChargedAmount, tokenOutAmount } =
      calculator(
        "quote",
        ksmAmount,
        new BigNumber(0.5),
        slippage.toNumber(),
        fee.toNumber()
      );

    expect(tokenOutAmount.toString()).toEqual("2");
    expect(feeChargedAmount.toString()).toEqual("0.01");
    expect(slippageAmount.toString()).toEqual("0.01");
    expect(minReceive.toString()).toEqual("1.96");
  });

  /**
    KSM
    USD
    1 USD = 0.5 KSM
   */
  test("Change USD to 2", () => {
    let ksmAmount = new BigNumber(2);

    const { minReceive, slippageAmount, feeChargedAmount, tokenOutAmount } =
      calculator(
        "base",
        ksmAmount,
        new BigNumber(0.5),
        slippage.toNumber(),
        fee.toNumber()
      );

    expect(tokenOutAmount.toString()).toEqual("1");
    expect(feeChargedAmount.toString()).toEqual("0.01");
    expect(slippageAmount.toString()).toEqual("0.01");
    expect(minReceive.toString()).toEqual("1.96");
  });
});
