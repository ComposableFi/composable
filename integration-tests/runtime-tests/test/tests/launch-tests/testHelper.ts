import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";
import { u128 } from "@polkadot/types-codec";
import { Permill } from "@polkadot/types/interfaces/runtime";
import { expect } from "chai";

export class Phase2 {

  public static async verifyLastPoolCreation(
    api: ApiPromise,
    pair: { base: number | u128, quote: number | u128 },
    feeConfig: {
      feeRate: number | u128,
      ownerFeeRate: number | u128,
      protocolFeeRate: number | u128
    },
    baseWeight: number | Permill,
    quoteWeight: number | Permill
  ) {
    const poolAmount = await api.query.pablo.poolCount();
    const pools = await api.query.pablo.pools(poolAmount.sub(new BN(1)));
    console.log(pools.toString());
    const unwrapped = pools.unwrap();
    expect(pools.unwrap().asConstantProduct.pair.base).to.be.bignumber.equal(new BN(pair.base));
    expect(pools.unwrap().asConstantProduct.pair.quote).to.be.bignumber.equal(new BN(pair.quote));
    expect(pools.unwrap().asConstantProduct.feeConfig.feeRate).to.be.bignumber.equal(new BN(feeConfig.feeRate));
    expect(pools.unwrap().asConstantProduct.feeConfig.ownerFeeRate).to.be.bignumber.equal(new BN(feeConfig.ownerFeeRate));
    expect(pools.unwrap().asConstantProduct.feeConfig.protocolFeeRate).to.be.bignumber.equal(new BN(feeConfig.protocolFeeRate));
    expect(pools.unwrap().asConstantProduct.baseWeight).to.be.bignumber.equal(new BN(baseWeight));
    expect(pools.unwrap().asConstantProduct.quoteWeight).to.be.bignumber.equal(new BN(quoteWeight));
  }

  public static async verifyPoolLiquidityAdded() {

  }

  public static async verifyPoolLiquidityRemoved() {

  }
}
