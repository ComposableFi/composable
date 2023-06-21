import { ApiPromise } from "@polkadot/api";
import { pipe } from "fp-ts/function";
import { option, readonlyArray } from "fp-ts";

export function isPalletSupported(api: ApiPromise | undefined) {
  return function (pallet: string) {
    return pipe(
      api,
      option.fromNullable,
      option.map((api) => api.runtimeMetadata.asLatest.pallets),
      option.chain((pallets) =>
        pipe(
          pallets.toArray(),
          readonlyArray.fromArray,
          readonlyArray.findFirst((p) => p.name.eq(pallet))
        )
      ),
      option.fold(
        () => false,
        () => true
      )
    );
  };
}
