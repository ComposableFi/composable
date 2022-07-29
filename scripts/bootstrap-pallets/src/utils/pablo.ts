
import { ApiPromise } from "@polkadot/api";
import { Permill } from "@polkadot/types/interfaces/runtime";

import { KeyringPair } from "@polkadot/keyring/types";
import { BigNumber } from "bignumber.js";
import { ComposableTraitsDefiCurrencyPairCurrencyId, PalletPabloPoolInitConfiguration } from "@composable/types";
import { PabloPoolFeeConfig } from "@bootstrap-pallets/types";

export function toPabloPoolPair(
  api: ApiPromise,
  base: string,
  quote: string
): ComposableTraitsDefiCurrencyPairCurrencyId {
  return api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
    base: api.createType("u128", base),
    quote: api.createType("u128", quote)
  });
}

export function toPermill(api: ApiPromise, value: string): Permill {
  return api.createType("Permill", value);
}

export function toFeeConfig(
  api: ApiPromise,
  feeRate: string,
  ownerFeeRate: string,
  protocolFeeRate: string
): PabloPoolFeeConfig {
  return {
    feeRate: api.createType("Permill", feeRate),
    ownerFeeRate: api.createType("Permill", ownerFeeRate),
    protocolFeeRate: api.createType("Permill", protocolFeeRate)
  };
}

export function toDexSale(
  api: ApiPromise,
  start: string,
  end: string,
  initialWeight: string,
  finalWeight: string,
  currentBlock: BigNumber,
  startDelay: number = 25
): any {
  const startBlock = currentBlock.plus(start).plus(startDelay).toString();
  const endBlock = new BigNumber(startBlock).plus(end).toString();

  return api.createType("ComposableTraitsDexSale", {
    start: api.createType("u32", startBlock),
    end: api.createType("u32", endBlock),
    initialWeight: api.createType("Permill", initialWeight),
    finalWeight: api.createType("Permill", finalWeight)
  });
}

export function toLiquidityBootstrappingPoolInitConfig(
  api: ApiPromise,
  owner: KeyringPair,
  poolConfig: any,
  currentBlock: BigNumber,
  startDelay: number = 25
): PalletPabloPoolInitConfiguration {
  return api.createType("PalletPabloPoolInitConfiguration", {
    LiquidityBootstrapping: {
      owner: owner.publicKey,
      pair: toPabloPoolPair(api, poolConfig.pair.base, poolConfig.pair.quote),
      sale: toDexSale(
        api,
        poolConfig.sale.start,
        poolConfig.sale.end,
        poolConfig.sale.initialWeight,
        poolConfig.sale.finalWeight,
        currentBlock,
        startDelay
      ),
      feeConfig: toFeeConfig(
        api,
        poolConfig.feeConfig.feeRate,
        poolConfig.feeConfig.ownerFeeRate,
        poolConfig.feeConfig.protocolFeeRate
      )
    }
  });
}

export function toConstantProductPoolInitConfig(
  api: ApiPromise,
  owner: KeyringPair,
  poolConfig: any
): PalletPabloPoolInitConfiguration {
  return api.createType("PalletPabloPoolInitConfiguration", {
    ConstantProduct: {
      owner: owner.publicKey,
      pair: toPabloPoolPair(api, poolConfig.pair.base, poolConfig.pair.quote),
      fee: toPermill(api, poolConfig.fee),
      baseWeight: toPermill(api, poolConfig.baseWeight)
    }
  });
}

export function toStableSwapPoolInitConfig(
  api: ApiPromise,
  owner: KeyringPair,
  poolConfig: any
): PalletPabloPoolInitConfiguration {
  return api.createType("PalletPabloPoolInitConfiguration", {
    StableSwap: {
      owner: owner.publicKey,
      pair: toPabloPoolPair(api, poolConfig.pair.base, poolConfig.pair.quote),
      amplificationCoefficient: api.createType("u16", poolConfig.amplificationCoefficient),
      fee: api.createType("Permill", poolConfig.fee)
    }
  });
}
