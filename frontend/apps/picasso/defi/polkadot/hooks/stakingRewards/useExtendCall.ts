import { useCallback } from "react";
import { SnackbarKey, useSnackbar } from "notistack";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import { extend } from "@/defi/polkadot/pallets/StakingRewards";
import { subscanExtrinsicLink } from "shared";
import { useExecutor, usePicassoProvider, useSigner } from "substrate-react";
import { usePicassoAccount } from "@/defi/polkadot/hooks";

export const useExtendCall = (onClose: () => void) => {
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const signer = useSigner();
  const executor = useExecutor();
  const account = usePicassoAccount();
  const { parachainApi } = usePicassoProvider();

  return useCallback(() => {
    let snackbarKey: SnackbarKey | undefined;
    return pipe(
      O.Do,
      O.bind("api", () => O.fromNullable(parachainApi)),
      O.bind("exec", () => O.fromNullable(executor)),
      O.bind("address", () => O.fromNullable(account?.address)),
      O.bind("signer", () => O.fromNullable(signer)),
      O.map(({ api, exec, address, signer }) =>
        extend(
          api,
          signer,
          address,
          exec,
          (txHash: string) => {
            snackbarKey = enqueueSnackbar("Processing transaction", {
              variant: "info",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
          },
          (txHash: string) => {
            closeSnackbar(snackbarKey);
            enqueueSnackbar(`Successfully staked`, {
              variant: "success",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
            onClose();
          },
          (errorMessage: string) => {
            closeSnackbar(snackbarKey);
            enqueueSnackbar(errorMessage, {
              variant: "error",
              isClosable: true,
              persist: true,
              description:
                "An error occurred while processing the transaction.",
            });
            onClose();
          }
        )
      )
    );
  }, [
    parachainApi,
    executor,
    account?.address,
    signer,
    enqueueSnackbar,
    closeSnackbar,
    onClose,
  ]);
};
