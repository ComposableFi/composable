import { getNormalizedAmount } from "../src/utils";
import { expect } from "chai";

describe("getNormalizedAmount", () => {
  it("should correctly normalize amount", () => {
    let normalizedAmount = getNormalizedAmount(
      BigInt(1_000_000_000_000_000),
      12
    );
    expect(normalizedAmount).to.equal(BigInt(1_000_000_000_000_000));

    normalizedAmount = getNormalizedAmount(BigInt(1_000_000_000_000_000), 8);
    expect(normalizedAmount).to.equal(BigInt(10_000_000_000_000_000_000));

    normalizedAmount = getNormalizedAmount(BigInt(1_000_000_000_000_000), 15);
    expect(normalizedAmount).to.equal(BigInt(1_000_000_000_000));
  });
});
