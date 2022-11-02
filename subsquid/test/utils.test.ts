import { getAmountWithoutDecimals } from "../src/utils";
import { expect } from "chai";
import BigNumber from "bignumber.js";

describe("getAmountWithoutDecimals", () => {
  it("should correctly remove decimals", () => {
    let normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_000_000_000),
      12
    );
    expect(normalizedAmount).to.deep.equal(BigNumber(1_000));

    normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_000_000_000),
      8
    );
    expect(normalizedAmount).to.deep.equal(BigNumber(10_000_000));

    normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_000_000_000),
      15
    );
    expect(normalizedAmount).to.deep.equal(BigNumber(1));

    normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_123_000_123),
      15
    );
    expect(normalizedAmount).to.deep.equal(BigNumber(1.000000123000123));
  });
});
