import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { u128, u32 } from "@polkadot/types-codec";
import { Permill } from "@polkadot/types/interfaces/runtime";
import { IEvent } from "@polkadot/types/types";
import { AccountId32 } from "@polkadot/types/interfaces";
import { ComposableTraitsDefiCurrencyPairCurrencyId } from "@composable/types/interfaces";

/**
 * Creates a constant product (Uniswap) dex pool.
 * @param api Connected API client.
 * @param senderWallet The wallet to send the transaction from.
 * @param managerWallet The wallet managing the pool.
 * @param baseAssetId
 * @param quoteAssetId
 * @param saleStart
 * @param saleEnd
 * @param initialWeight
 * @param finalWeight
 * @param feeRate
 * @param ownerFeeRate
 * @param protocolFeeRate
 */
export default async function(
  api: ApiPromise,
  senderWallet: KeyringPair,
  managerWallet: Uint8Array | AccountId32 | string,
  baseAssetId: number | u128,
  quoteAssetId: number | u128,
  saleStart: number | u32,
  saleEnd: number | u32,
  initialWeight: number | Permill,
  finalWeight: number | Permill,
  feeRate: number | Permill,
  ownerFeeRate: number | Permill,
  protocolFeeRate: number | Permill
): Promise<IEvent<[u128, AccountId32, ComposableTraitsDefiCurrencyPairCurrencyId]>> {
  const pool = api.createType("PalletPabloPoolInitConfiguration", {
    LiquidityBootstrapping: {
      owner: api.createType("AccountId32", managerWallet),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", baseAssetId),
        quote: api.createType("u128", quoteAssetId)
      }),
      sale: api.createType("ComposableTraitsDexSale", {
        start: api.createType("u32", saleStart),
        end: api.createType("u32", saleEnd),
        initialWeight: api.createType("Permill", initialWeight),
        finalWeight: api.createType("Permill", finalWeight)
      }),
      feeConfig: api.createType("ComposableTraitsDexFeeConfig", {
        feeRate: api.createType("Permill", feeRate),
        ownerFeeRate: api.createType("Permill", ownerFeeRate),
        protocolFeeRate: api.createType("Permill", protocolFeeRate)
      })
    }
  });
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.pablo.PoolCreated.is,
    api.tx.pablo.create(pool)
  );
}
