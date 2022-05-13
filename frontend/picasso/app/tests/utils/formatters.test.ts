import { formatToken } from "@/utils/formatters";
import BigNumber from "bignumber.js";

describe("formatters", () => {
  describe("formatAmount", () => {
    it("should format the amount with eth token", () => {
      const result = formatToken(5.2, "eth");
      expect(result).toBe("5.2 ETH");
    });

    it("should format the amount with usdc token", () => {
      const result = formatToken(5.2, "usdc");
      expect(result).toBe("5.2 USDC");
    });

    it("should format a bignumber amount correctly", () => {
      const result = formatToken(new BigNumber(5.2), "usdc");
      expect(result).toBe("5.2 USDC");
    });
  });
});
