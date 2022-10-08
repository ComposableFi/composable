import { fromPerbill } from "shared";
import BigNumber from "bignumber.js";

export class PabloPoolFeeConfig {
  protected __feeRate: BigNumber;
  protected __ownerFeeRate: BigNumber;
  protected __protocolFeeRate: BigNumber;

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
    this.__feeRate = feeRate;
    this.__ownerFeeRate = ownerFeeRate;
    this.__protocolFeeRate = protocolFeeRate;
  }

  toJSON(): {
    feeRate: string;
    ownerFeeRate: string;
    protocolFeeRate: string
  } {
    return {
      feeRate: this.__feeRate.toString(),
      ownerFeeRate: this.__ownerFeeRate.toString(),
      protocolFeeRate: this.__protocolFeeRate.toString(),
    }
  }

  getFeeRate(): BigNumber {
    return this.__feeRate;
  }
}
