import { ApiPromise } from "@polkadot/api";
import { subscanAccountLink } from "../../Networks";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import { Executor, useSigner } from "substrate-react";
import { setCrowdloanRewardsState } from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";
import { Signer } from "@polkadot/api/types";
import BigNumber from "bignumber.js";

export type ClaimProps = {
  api: ApiPromise | undefined;
  executor: Executor | undefined;
  selectedPicassoAddress: string | undefined;
  selectedEthereumAddress: string | undefined;
};

export function useCrowdloanRewardsClaim({
  api,
  executor,
  selectedPicassoAddress
}: ClaimProps) {
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();

  const onClaimReady = useCallback(
    (transactionHash: string) => {
      if (selectedPicassoAddress)
      enqueueSnackbar("Claim processing...", {
        variant: "info",
        isClosable: true,
        url: subscanAccountLink("picasso", selectedPicassoAddress),
      });
    },
    [enqueueSnackbar, selectedPicassoAddress]
  );

  const onClaimFinalized = useCallback(
    (transactionHash: string) => {
      if (selectedPicassoAddress)
      enqueueSnackbar("Your claim was successful!", {
        variant: "success",
        isClosable: true,
        url: subscanAccountLink("picasso", selectedPicassoAddress),
      });

      setCrowdloanRewardsState({ claimableAmount: new BigNumber(0) });
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
    [enqueueSnackbar]
  );

  return useCallback(async () => {
    if (!api || !executor || !selectedPicassoAddress || !signer) return;
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
  }, [
    api,
    executor,
    selectedPicassoAddress,
    onClaimReady,
    onClaimFinalized,
    onClaimFail,
    signer,
  ]);
}
