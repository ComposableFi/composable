import { ApiPromise } from "@polkadot/api";
import { subscanAccountLink } from "../../Networks";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import { Executor, useSigner } from "substrate-react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { fetchAssociations } from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards";
import { setCrowdloanRewardsState } from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";

export type AssociateProps = {
  api: ApiPromise | undefined;
  executor: Executor | undefined;
  selectedPicassoAddress: string | undefined;
  associateMode?: "ethereum" | "kusama";
  connectedAccounts: InjectedAccountWithMeta[];
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
  associateMode,
  connectedAccounts,
}: AssociateProps) {
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();

  const onAssociationReady = useCallback(
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

  const onAssociateFinalized = useCallback(
    (transactionHash: string) => {
      if (selectedPicassoAddress)
      enqueueSnackbar("Your claim was successful!", {
        variant: "success",
        isClosable: true,
        url: subscanAccountLink("picasso", selectedPicassoAddress),
      });

      if (api) {
        fetchAssociations(
          api,
          connectedAccounts.map((x) => x.address)
        ).then((onChainAssociations) => {
          setCrowdloanRewardsState({ onChainAssociations });
        });
      }
    },
    [enqueueSnackbar, api, connectedAccounts, selectedPicassoAddress]
  );

  const onAssociationFail = useCallback(
    (associationError: string) => {
      enqueueSnackbar(associationError, {
        variant: "error",
        isClosable: true,
      });
    },
    [enqueueSnackbar]
  );

  return useCallback(
    async (signature: string) => {
      if (!api || !signer || !executor || !associateMode || !selectedPicassoAddress)
        return;
      try {
        const signatureParam = createSignatureParam(
          signature,
          api,
          associateMode,
          selectedPicassoAddress
        );
        const accountId32 = api.createType(
          "AccountId32",
          selectedPicassoAddress
        );
        await executor.executeUnsigned(
          api.tx.crowdloanRewards.associate(accountId32, signatureParam),
          api,
          onAssociationReady,
          onAssociateFinalized
        );
      } catch (err: any) {
        onAssociationFail(err.message);
      }
    },
    [
      api,
      executor,
      associateMode,
      selectedPicassoAddress,
      onAssociationReady,
      onAssociateFinalized,
      onAssociationFail,
      signer
    ]
  );
}
