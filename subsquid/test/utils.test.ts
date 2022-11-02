import { getAmountWithoutDecimals } from "../src/utils";
import { expect } from "chai";

describe("getAmountWithoutDecimals", () => {
  it("should correctly remove decimals", () => {
    let normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_000_000_000),
      12
    );
    expect(normalizedAmount).to.equal(BigInt(1_000));

    normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_000_000_000),
      8
    );
    expect(normalizedAmount).to.equal(BigInt(10_000_000));

    normalizedAmount = getAmountWithoutDecimals(
      BigInt(1_000_000_000_000_000),
      15
    );
    expect(normalizedAmount).to.equal(BigInt(1));
  });
});
