import { ApiPromise } from "@polkadot/api";
import { APP_NAME } from "../../constants";
import { SUBSTRATE_NETWORKS } from "../../Networks";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import { Executor, useSigner } from "substrate-react";


export type ClaimProps = {
  api: ApiPromise | undefined;
  executor: Executor | undefined;
  selectedPicassoAddress: string | undefined;
  selectedEthereumAddress: string | undefined;
};

export function useCrowdloanRewardsClaim({
  api,
  executor,
  selectedPicassoAddress,
}: ClaimProps) {
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();

  const onClaimReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim Processing", {
        variant: "info",
        isClosable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + transactionHash,
      });
    },
    [enqueueSnackbar]
  );

  const onClaimFinalized = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim Finalized", {
        variant: "success",
        isClosable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + transactionHash,
      });

      // if (selectedPicassoAddress) {
      //   setClaimedKsm(selectedPicassoAddress)
      // }
    },
    [enqueueSnackbar, selectedPicassoAddress]
  );

  const onClaimFail = useCallback(
    (associationError: string) => {
      enqueueSnackbar(associationError, {
        variant: "error",
        isClosable: true,
      });
    },
    [
      enqueueSnackbar,
    ]
  );

  return useCallback(
    async () => {
      const { web3Enable } = require("@polkadot/extension-dapp");
      await web3Enable(APP_NAME);

      if (
        !api ||
        !executor ||
        !selectedPicassoAddress ||
        !signer
      )
        return;
      try {
        await executor.execute(
          api.tx.crowdloanRewards.claim(),
          selectedPicassoAddress,
          api,
          signer,
          onClaimReady,
          onClaimFinalized,
          onClaimFail
        );
      } catch (err: any) {
        onClaimFail(err.message);
      }
    },
    [
      api,
      executor,
      selectedPicassoAddress,
      onClaimReady,
      onClaimFinalized,
      onClaimFail,
      signer,
    ]
  );
}
