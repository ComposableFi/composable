import { ApiPromise } from "@polkadot/api";
import { APP_NAME } from "../../constants";
import { SUBSTRATE_NETWORKS } from "../../Networks";
import { useSnackbar } from "notistack";
import { useCallback } from "react";
import { Executor } from "substrate-react";
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

  const onAssociationReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim Processing", {
        variant: "info",
        isClosable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + "extrinsic/" + transactionHash,
      });
    },
    [enqueueSnackbar]
  );

  const onAssociateFinalized = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar("Claim Finalized", {
        variant: "success",
        isClosable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + "extrinsic/" + transactionHash,
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
    [enqueueSnackbar, api, connectedAccounts]
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
      const { web3Enable } = require("@polkadot/extension-dapp");
      await web3Enable(APP_NAME);

      if (!api || !executor || !associateMode || !selectedPicassoAddress)
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
    ]
  );
}
