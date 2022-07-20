import {
  formatToken,
  formatNumber,
  formatNumberWithSymbol,
  formatNumberCompact,
  formatNumberCompactWithToken,
  formatNumberCompactWithSymbol
} from "./formatters";
import BigNumber from "bignumber.js";

describe("formatters", () => {
  describe("formatToken", () => {
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
  describe("formatNumber", () => {
    it("should format number 5000000 with commas", () => {
      const result = formatNumber(5000000);
      expect(result).toBe("5,000,000");
    });
    it("should format number 10000 commas", () => {
      const result = formatNumber(10000);
      expect(result).toBe("10,000");
    });
  });
  describe("formatNumberWithSymbol", () => {
    it("should format amount with $ in front", () => {
      const result = formatNumberWithSymbol(3500, "$");
      expect(result).toBe("$3,500");
    });
    it("should format amount with dash and $ in front", () => {
      const result = formatNumberWithSymbol(-2352, "$");
      expect(result).toBe("-$2,352");
    });
    it("should format amount with % on the end", () => {
      const result = formatNumberWithSymbol(4200, "", "%");
      expect(result).toBe("4,200%");
    });
    it("should format amount with dash and % on the end", () => {
      const result = formatNumberWithSymbol(-1400, "", "%");
      expect(result).toBe("-1,400%");
    });
    it("should format amount with dash and $ in front and % on the end", () => {
      const result = formatNumberWithSymbol(-2800, "$", "%");
      expect(result).toBe("-$2,800%");
    });
    it("should format amount with $ in front and % on the end", () => {
      const result = formatNumberWithSymbol(6150, "$", "%");
      expect(result).toBe("$6,150%");
    });
  });
  describe("formatNumberCompact", () => {
    it("should format amount into compact amount; 100000 => 100K", () => {
      const result = formatNumberCompact(100000);
      expect(result).toBe("100K");
    });
    it("should format amount into compact amount with decimals; 100250 => 100.25K", () => {
      const result = formatNumberCompact(100250);
      expect(result).toBe("100.25K");
    });
    it("should format amount into compact amount; 5000000 => 5M", () => {
      const result = formatNumberCompact(5000000);
      expect(result).toBe("5M");
    });
    it("should format amount into compact amount with decimals; 5007000 => 5.01M", () => {
      const result = formatNumberCompact(5007000);
      expect(result).toBe("5.01M");
    });
  });
  describe("formatNumberCompactWithToken", () => {
    it("should format amount into compact amount with pica token", () => {
      const result = formatNumberCompactWithToken(350500, "pica");
      expect(result).toBe("350.5K PICA");
    });
    it("should format amount into compact amount with eth token", () => {
      const result = formatNumberCompactWithToken(3505000, "eth");
      expect(result).toBe("3.51M ETH");
    });
    it("should format negative amount into negative compact amount with ksm token", () => {
      const result = formatNumberCompactWithToken(-230004, "ksm");
      expect(result).toBe("-230K KSM");
    });
  });
  describe("formatNumberCompactWithSymbol", () => {
    it("should format amount into compact amount with $ in front", () => {
      const result = formatNumberCompactWithSymbol(500100, "$");
      expect(result).toBe("$500.1K");
    });
  });
});
