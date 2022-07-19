import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";
import { expect } from "chai";
import { PalletPabloPoolConfiguration } from "@composable/types/interfaces";
import { Option, u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { waitForBlocks } from "@composable/utils/polkadotjs";

export class Phase2 {
  public static async verifyLastPoolCreation(
    api: ApiPromise,
    poolConfig: PalletPabloPoolConfiguration
  ): Promise<{ poolId: BN; lpTokenId: BN }> {
    const poolAmount = await api.query.pablo.poolCount();
    const poolId = poolAmount.sub(new BN(1));
    const pools = await api.query.pablo.pools(poolId);
    console.log(pools.toString());
    if (poolConfig.isConstantProduct == true) {
      return this.verifyConstantProductPool(poolConfig, pools, poolId, poolAmount);
    } else if (poolConfig.isLiquidityBootstrapping == true) {
      return this.verifyLiquidityBootstrappingPool(poolConfig, pools, poolId, poolAmount);
    } else if (poolConfig.isStableSwap == true) {
      return this.verifyStableSwapPool(poolConfig, pools, poolId, poolAmount);
    }
  }

  static verifyConstantProductPool(
    poolConfig: PalletPabloPoolConfiguration,
    pools: Option<PalletPabloPoolConfiguration>,
    poolId: BN,
    poolAmount: u128
  ) {
    const lpTokenId = poolConfig.asConstantProduct.lpToken.add(poolAmount.sub(new BN(1)));
    expect(pools.unwrap().asConstantProduct.owner.toString()).to.be.equal(
      poolConfig.asConstantProduct.owner.toString()
    );
    expect(pools.unwrap().asConstantProduct.pair.base).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.pair.base)
    );
    expect(pools.unwrap().asConstantProduct.pair.quote).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.pair.quote)
    );
    expect(pools.unwrap().asConstantProduct.feeConfig.feeRate).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.feeConfig.feeRate)
    );
    expect(pools.unwrap().asConstantProduct.feeConfig.ownerFeeRate).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.feeConfig.ownerFeeRate)
    );
    expect(pools.unwrap().asConstantProduct.feeConfig.protocolFeeRate).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.feeConfig.protocolFeeRate)
    );
    expect(pools.unwrap().asConstantProduct.baseWeight).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.baseWeight)
    );
    expect(pools.unwrap().asConstantProduct.quoteWeight).to.be.bignumber.equal(
      new BN(poolConfig.asConstantProduct.quoteWeight)
    );
    expect(pools.unwrap().asConstantProduct.lpToken).to.be.bignumber.equal(lpTokenId);
    return { poolId, lpTokenId };
  }

  static verifyLBPPool(
    poolConfig: PalletPabloPoolConfiguration,
    pools: Option<PalletPabloPoolConfiguration>,
    poolId: BN,
    poolAmount: u128
  ) {
    console.debug(poolConfig.toString());
    expect(pools.unwrap().asLiquidityBootstrapping.owner.toString()).to.be.equal(
      poolConfig.asLiquidityBootstrapping.owner.toString()
    );
    expect(pools.unwrap().asLiquidityBootstrapping.pair.base).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.pair.base)
    );
    expect(pools.unwrap().asLiquidityBootstrapping.pair.quote).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.pair.quote)
    );

    expect(new BN(pools.unwrap().asLiquidityBootstrapping.sale.start)).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.sale.start)
    );
    expect(new BN(pools.unwrap().asLiquidityBootstrapping.sale.end)).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.sale.end)
    );
    expect(new BN(pools.unwrap().asLiquidityBootstrapping.sale.initial_weight)).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.sale.initial_weight)
    );
    expect(new BN(pools.unwrap().asLiquidityBootstrapping.sale.final_weight)).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.sale.final_weight)
    );

    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.feeRate).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.feeConfig.feeRate)
    );
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.ownerFeeRate).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.feeConfig.ownerFeeRate)
    );
    expect(pools.unwrap().asLiquidityBootstrapping.feeConfig.protocolFeeRate).to.be.bignumber.equal(
      new BN(poolConfig.asLiquidityBootstrapping.feeConfig.protocolFeeRate)
    );

    return { poolId, undefined };
  }

  static verifyStableSwapPool(
    poolConfig: PalletPabloPoolConfiguration,
    pools: Option<PalletPabloPoolConfiguration>,
    poolId: BN,
    poolAmount: u128
  ) {
    const lpTokenId = poolConfig.asStableSwap.lpToken.add(poolAmount.sub(new BN(1)));
    expect(pools.unwrap().asStableSwap.owner.toString()).to.be.equal(poolConfig.asStableSwap.owner.toString());
    expect(pools.unwrap().asStableSwap.pair.base).to.be.bignumber.equal(new BN(poolConfig.asStableSwap.pair.base));
    expect(pools.unwrap().asStableSwap.pair.quote).to.be.bignumber.equal(new BN(poolConfig.asStableSwap.pair.quote));
    expect(pools.unwrap().asStableSwap.lpToken).to.be.bignumber.equal(new BN(lpTokenId));
    expect(pools.unwrap().asStableSwap.amplification_coefficient).to.be.bignumber.equal(
      new BN(poolConfig.asStableSwap.amplification_coefficient)
    );
    expect(pools.unwrap().asStableSwap.feeConfig.feeRate).to.be.bignumber.equal(
      new BN(poolConfig.asStableSwap.feeConfig.feeRate)
    );
    expect(pools.unwrap().asStableSwap.feeConfig.ownerFeeRate).to.be.bignumber.equal(
      new BN(poolConfig.asStableSwap.feeConfig.ownerFeeRate)
    );
    expect(pools.unwrap().asStableSwap.feeConfig.protocolFeeRate).to.be.bignumber.equal(
      new BN(poolConfig.asStableSwap.feeConfig.protocolFeeRate)
    );

    return { poolId, lpTokenId };
  }

  static verifyLiquidityBootstrappingPool(
    poolConfig: PalletPabloPoolConfiguration,
    pools: Option<PalletPabloPoolConfiguration>,
    poolId: BN,
    poolAmount: u128
  ) {
    // ToDo!
    const lpTokenId = new BN(0);
    return { poolId, lpTokenId };
  }

  public static async verifyPoolLiquidityAdded(
    api: ApiPromise,
    baseAssetId: number,
    quoteAssetId: number,
    lpTokenId: number,
    wallet: AccountId32 | Uint8Array,
    amount: number,
    baseAssetFundsBefore: BN,
    quoteAssetFundsBefore: BN,
    currentLpTokenFundsBefore: BN
  ) {
    const currentBaseAssetFunds = await api.rpc.assets.balanceOf(baseAssetId.toString(), wallet);
    const currentQuoteAssetFunds = await api.rpc.assets.balanceOf(baseAssetId.toString(), wallet);
    const currentLPTokenAssetFunds = await api.rpc.assets.balanceOf(baseAssetId.toString(), wallet);

    expect(new BN(currentBaseAssetFunds.toString())).to.be.bignumber.lessThan(baseAssetFundsBefore);
    expect(new BN(currentQuoteAssetFunds.toString())).to.be.bignumber.lessThan(quoteAssetFundsBefore);
    expect(new BN(currentLPTokenAssetFunds.toString())).to.be.bignumber.greaterThan(currentLpTokenFundsBefore);
  }

  public static async verifyPoolLiquidityRemoved() {}

  public static async waitForLBPPoolStarted(api: ApiPromise, poolId: u128 | number | BN) {
    let currentBlock = await api.query.system.number();
    const poolInfo = await api.query.pablo.pools(poolId);
    while (currentBlock.lt(poolInfo.unwrap().asLiquidityBootstrapping.sale.start)) {
      await waitForBlocks(api);
      currentBlock = await api.query.system.number();
    }
  }
}
