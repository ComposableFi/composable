import { ApiPromise } from "@polkadot/api";
import { Option, StorageKey } from "@polkadot/types";
import { pipe } from "fp-ts/function";
import { option, readonlyArray } from "fp-ts";
import BigNumber from "bignumber.js";
import { PoolConfig, PoolKind } from "@/store/createPool/types";
import { fromPermill, unwrapNumberOrHex } from "shared";
import type { PalletPabloPoolConfiguration } from "defi-interfaces";

export function subscribePoolEntries(
  parachainApi: ApiPromise,
  storeFn: (a: any) => void
) {
  return parachainApi.query.pablo.pools.entries(
    (entries: Array<[StorageKey, Option<PalletPabloPoolConfiguration>]>) =>
      pipe(
        readonlyArray.fromArray(entries),
        readonlyArray.map(([k, o]) => {
          const [poolId] = k.toHuman() as Array<string>;

          return pipe(
            option.Do,
            option.bind("poolId", () => option.some(new BigNumber(poolId))),
            option.bind("config", () =>
              o.isSome ? option.some(o.value) : option.none
            ),
            option.bind("kind", ({ config }) =>
              (config as any).isDualAssetConstantProduct
                ? option.some("dualAssetConstantProduct" as PoolKind)
                : option.none
            ),
            option.chain(({ poolId, config, kind }) =>
              option.some({
                poolId,
                kind,
                config: {
                  owner: (config.toJSON() as any)[kind].toString(),
                  assetsWeights: Object.fromEntries(
                    Object.entries(
                      (config.toJSON() as any)[kind].assetsWeights
                    ).map((a) => [
                      a[0],
                      fromPermill(
                        unwrapNumberOrHex(a[1] as string).toString()
                      ).toNumber(),
                    ])
                  ),
                  lpToken: (config.toJSON() as any)[kind].lpToken,
                  feeConfig: {
                    feeRate: fromPermill(
                      unwrapNumberOrHex(
                        (config.toJSON() as any)[kind].feeConfig
                          .feeRate as string
                      ).toNumber()
                    ).toNumber(),
                    ownerFeeRate: fromPermill(
                      unwrapNumberOrHex(
                        (config.toJSON() as any)[kind].feeConfig
                          .ownerFeeRate as string
                      ).toNumber()
                    ).toNumber(),
                    protocolFeeRate: fromPermill(
                      unwrapNumberOrHex(
                        (config.toJSON() as any)[kind].feeConfig
                          .protocolFeeRate as string
                      ).toNumber()
                    ).toNumber(),
                  },
                },
              } as PoolConfig)
            ),
            option.fold(
              () => null,
              (s) => s
            )
          );
        }),
        readonlyArray.filter((a) => a !== null),
        readonlyArray.toArray,
        (items) => storeFn(items)
      )
  ) as unknown as Promise<() => void>;
}
