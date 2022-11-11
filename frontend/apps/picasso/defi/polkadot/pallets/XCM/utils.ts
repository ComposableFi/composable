import { EventRecord } from "@polkadot/types/interfaces/system";
import { ApiPromise } from "@polkadot/api";
import { AnyComponentMap, EnqueueSnackbar, SnackbarKey } from "notistack";
import { pipe } from "fp-ts/function";
import { findFirst } from "fp-ts/Array";
import { boolean, option } from "fp-ts";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

export function xcmPalletEventParser(
  records: EventRecord[],
  api: ApiPromise,
  closeSnackbar: (key?: SnackbarKey) => void,
  snackbarKey: SnackbarKey | undefined,
  enqueueSnackbar: EnqueueSnackbar<AnyComponentMap>,
  txHash: string
) {
  pipe(
    records,
    findFirst(
      (e) =>
        api.events?.xcmPallet?.Attempted?.is?.(e.event) ||
        api.events?.polkadotXcm?.Attempted?.is?.(e.event)
    ),
    option.map((e) => e.event),
    option.map((event) => {
      // @ts-ignore
      if (event.data.find((x: XcmV2TraitsOutcome) => x.isComplete)) return true;
      return false;
    }),
    option.fold(
      () => {
        closeSnackbar(snackbarKey);
        enqueueSnackbar("Transfer failed", {
          persist: true,
          description: "",
          variant: "error",
          isCloseable: true,
          url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
        });
      },
      boolean.fold(
        () => {
          closeSnackbar(snackbarKey);
          enqueueSnackbar(
            "Transfer failed, could not confirm transaction on-chain.",
            {
              persist: true,
              description: "XcmV2TraitsOutcome: Incomplete",
              variant: "error",
              isCloseable: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
            }
          );
        },
        () => {
          closeSnackbar(snackbarKey);
          enqueueSnackbar("Transfer successful!", {
            persist: true,
            variant: "success",
            isCloseable: true,
            url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
          });
        }
      )
    )
  );
}
