import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";
import { expect } from "chai";
import { PalletPabloPoolConfiguration } from "@composable/types/interfaces";
import { Option, u128 } from "@polkadot/types-codec";

export class Phase2 {
  public static async verifyLastPoolCreation(
    api: ApiPromise,
    poolConfig: PalletPabloPoolConfiguration
  ): Promise<{ poolId: BN, lpTokenId: BN }> {
    const poolAmount = await api.query.pablo.poolCount();
    const poolId = poolAmount.sub(new BN(1));
    const pools = await api.query.pablo.pools(poolId);
    console.log(pools.toString());
    if (poolConfig.isConstantProduct == true) {
      return this.verifyConstantProductPool(poolConfig, pools, poolId, poolAmount);
    } else if (poolConfig.isLiquidityBootstrapping == true) {
      return this.verifyLiquidityBootstrappingPool(poolConfig, pools, poolId, poolAmount);
    } else if (poolConfig.isStableSwap == true) {
      // ToDo!
    }
  }

  static verifyConstantProductPool(
    poolConfig: PalletPabloPoolConfiguration,
    pools: Option<PalletPabloPoolConfiguration>,
    poolId: BN,
    poolAmount: u128
  ) {
    const lpTokenId = poolConfig.asConstantProduct.lpToken.add(poolAmount);
    expect(pools.unwrap().asConstantProduct.pair.base).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.pair.base));
    expect(pools.unwrap().asConstantProduct.pair.quote).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.pair.quote));
    expect(pools.unwrap().asConstantProduct.feeConfig.feeRate).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.feeConfig.feeRate));
    expect(pools.unwrap().asConstantProduct.feeConfig.ownerFeeRate).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.feeConfig.ownerFeeRate));
    expect(pools.unwrap().asConstantProduct.feeConfig.protocolFeeRate).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.feeConfig.protocolFeeRate));
    expect(pools.unwrap().asConstantProduct.baseWeight).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.baseWeight));
    expect(pools.unwrap().asConstantProduct.quoteWeight).to.be.bignumber
      .equal(new BN(poolConfig.asConstantProduct.quoteWeight));
    expect(pools.unwrap().asConstantProduct.lpToken).to.be.bignumber
      .equal(lpTokenId);
    return { poolId, lpTokenId };
  }

  static verifyLiquidityBootstrappingPool(
    poolConfig: PalletPabloPoolConfiguration,
    pools: Option<PalletPabloPoolConfiguration>,
    poolId: BN,
    poolAmount: u128
  ) {
    expect(pools.unwrap().asLiquidityBootstrapping.pair.base).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.pair.base));
    expect(pools.unwrap().asLiquidityBootstrapping.pair.quote).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.pair.quote));
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.ownerFeeRate).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.feeConfig.ownerFeeRate));
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.protocolFeeRate).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.feeConfig.protocolFeeRate));
    expect(pools.unwrap().asLiquidityBootstrapping.sale.start).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.sale.start));
    expect(pools.unwrap().asLiquidityBootstrapping.sale.end).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.sale.end));
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.feeRate).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.feeConfig.feeRate));
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.ownerFeeRate).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.feeConfig.ownerFeeRate));
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.protocolFeeRate).to.be.bignumber
      .equal(new BN(poolConfig.asLiquidityBootstrapping.feeConfig.protocolFeeRate));
    const lpTokenId = new BN(0);
    return { poolId, lpTokenId };
  }

  public static async verifyPoolLiquidityAdded() {

  }

  public static async verifyPoolLiquidityRemoved() {

  }
}
