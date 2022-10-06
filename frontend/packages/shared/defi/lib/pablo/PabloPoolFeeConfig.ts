import { fromPerbill } from "shared";
import BigNumber from "bignumber.js";

export class PabloPoolFeeConfig {
  feeRate: BigNumber;
  ownerFeeRate: BigNumber;
  protocolFeeRate: BigNumber;

  static fromJSON(feeConfig: {
    feeRate: string;
    ownerFeeRate: string;
    protocolFeeRate: string;
  }): PabloPoolFeeConfig {
    return new PabloPoolFeeConfig(
      fromPerbill(feeConfig.feeRate),
      fromPerbill(feeConfig.ownerFeeRate),
      fromPerbill(feeConfig.protocolFeeRate)
    );
  }

  constructor(
    feeRate: BigNumber,
    ownerFeeRate: BigNumber,
    protocolFeeRate: BigNumber
  ) {
    this.feeRate = feeRate;
    this.ownerFeeRate = ownerFeeRate;
    this.protocolFeeRate = protocolFeeRate;
  }
}
