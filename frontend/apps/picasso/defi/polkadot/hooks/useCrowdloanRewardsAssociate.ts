import { ApiPromise } from "@polkadot/api";
import { APP_NAME } from "../constants";
import { SUBSTRATE_NETWORKS } from "../Networks";
import { useSnackbar } from "notistack";
import { useCallback, useMemo } from "react";
import { Executor } from "substrate-react";
import {
  setAssociatedEthereum,
  setAssociatedKsm,
} from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";

export type AssociateProps = {
  api: ApiPromise | undefined;
  executor: Executor | undefined;
  selectedPicassoAddress: string | undefined;
  selectedEthereumAddress: string | undefined;
  associateMode?: "ethereum" | "kusama";
};

function createSignatureParam(
  signature: string,
  api: ApiPromise,
  associateMode: "ethereum" | "kusama",
  selectedPicassoAddress: string
) {
  const accountId32 = api.createType("AccountId32", selectedPicassoAddress);

  return associateMode === "ethereum"
    ? { Ethereum: signature }
    : { RelayChain: [accountId32, { Sr25519: signature }] };
}

export function useCrowdloanRewardsAssociate({
  api,
  executor,
  selectedPicassoAddress,
  selectedEthereumAddress,
  associateMode,
}: AssociateProps) {
  const { enqueueSnackbar } = useSnackbar();

  const onAssociationReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim Processing", {
        variant: "info",
        isClosable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + transactionHash,
      });
    },
    [enqueueSnackbar]
  );

  const onAssociateFinalized = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim Finalized", {
        variant: "success",
        isClosable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + transactionHash,
      });

      if (selectedPicassoAddress) {
        if (
          associateMode &&
          associateMode === "ethereum" &&
          selectedEthereumAddress
        ) {
          setAssociatedEthereum(
            selectedEthereumAddress,
            selectedPicassoAddress
          );
        } else {
          setAssociatedKsm(selectedPicassoAddress);
        }
      }
    },
    [
      // initialPayment,
      // totalRewards,
      enqueueSnackbar,
      associateMode,
      selectedEthereumAddress,
      selectedPicassoAddress,
    ]
  );

  const onAssociationFail = useCallback(
    (associationError: string) => {
      enqueueSnackbar(associationError, {
        variant: "error",
        isClosable: true,
      });
    },
    [
      // initialPayment,
      // totalRewards,
      enqueueSnackbar,
    ]
  );

  return useCallback(async (signature: string) => {
    const { web3Enable } = require("@polkadot/extension-dapp");
    await web3Enable(APP_NAME);

    if (!api || !executor || !associateMode || !selectedPicassoAddress) return;
    try {
      const signatureParam = createSignatureParam(signature, api, associateMode, selectedPicassoAddress)
      const accountId32 = api.createType("AccountId32", selectedPicassoAddress);
      await executor.executeUnsigned(
        api.tx.crowdloanRewards.associate(accountId32, signatureParam),
        api,
        onAssociationReady,
        onAssociateFinalized
      );
    } catch (err: any) {
      onAssociationFail(err.message);
    }
  }, [api, executor, associateMode, selectedPicassoAddress, onAssociationReady, onAssociateFinalized, onAssociationFail]);
}
