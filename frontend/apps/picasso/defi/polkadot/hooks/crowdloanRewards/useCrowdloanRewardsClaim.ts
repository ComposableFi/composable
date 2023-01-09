import { ApiPromise } from "@polkadot/api";
import { subscanExtrinsicLink } from "shared";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import { Executor, useSigner } from "substrate-react";
import { setCrowdloanRewardsState } from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";
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
  selectedPicassoAddress,
}: ClaimProps) {
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();

  const onClaimReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim processing...", {
        variant: "info",
        isClosable: true,
        url: subscanExtrinsicLink("picasso", transactionHash),
      });
    },
    [enqueueSnackbar]
  );

  const onClaimFinalized = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Your claim was successful!", {
        variant: "success",
        isClosable: true,
        url: subscanExtrinsicLink("picasso", transactionHash),
      });

      setCrowdloanRewardsState({ claimableAmount: new BigNumber(0) });
    },
    [enqueueSnackbar]
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
