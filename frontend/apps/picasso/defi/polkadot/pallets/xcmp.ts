import { ApiPromise } from "@polkadot/api";
import { Executor } from "substrate-react";
import { u128 } from "@polkadot/types-codec";
import { AnyComponentMap, EnqueueSnackbar } from "notistack";
import { XcmVersionedMultiLocation } from "@polkadot/types/lookup";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import {
  buildParachainToParachainAccountDestination,
  buildParachainToRelaychainAccountDestination,
  buildRelaychainToParachainBeneficiary,
  buildRelaychainToParachainDestination,
  buildXCMAssetOriginKsm,
  getParachainDestinationCallOriginKarura,
  getXTokenTransferCallOriginPicasso,
} from "./XCM";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { Signer } from "@polkadot/api/types";
import BigNumber from "bignumber.js";

export type TransferHandlerArgs = {
  api: ApiPromise;
  targetChain: number | 0;
  transferToken: TokenMetadata;
  feeToken: TokenMetadata;
  targetAccount: string;
  amount: u128;
  executor: Executor;
  enqueueSnackbar: EnqueueSnackbar<AnyComponentMap>;
  signerAddress: string;
  hasFeeItem: boolean;
  signer: Signer;
  weight: BigNumber;
};

export function availableTargetNetwork(
  network: string,
  selectedNetwork: string
) {
  switch (selectedNetwork) {
    case "kusama":
      return network === "picasso";
    case "picasso":
      return network === "kusama" || network === "karura";
    case "karura":
      return network === "picasso";
  }
}

export function getTransferCallKusamaPicasso(
  api: ApiPromise,
  targetChain: number | 0,
  targetAccount: string,
  amount: u128
) {
  const destination = buildRelaychainToParachainDestination(api, targetChain);
  // Setting the wallet receiving the funds
  const beneficiary = buildRelaychainToParachainBeneficiary(api, targetAccount);
  // Setting up the asset & amount
  const assets = buildXCMAssetOriginKsm(api, amount);
  // Setting the asset which will be used for fees (0 refers to first in asset list)
  const feeAssetItem = api.createType("u32", 0);

  return api.tx.xcmPallet.reserveTransferAssets(
    destination,
    beneficiary,
    assets,
    feeAssetItem
  );
}

export function getTransferCallPicassoKarura(
  api: ApiPromise,
  targetChain: number | 0,
  targetAccount: string,
  hasFeeToken: boolean,
  transferToken: TokenMetadata,
  amount: u128,
  feeTokenId: BigNumber | null
) {
  if (!transferToken.picassoId) throw new Error("Unsupported transfer");

  const destination = buildParachainToParachainAccountDestination(
    api,
    targetChain,
    targetAccount
  );

  const call = getXTokenTransferCallOriginPicasso(
    api,
    destination as XcmVersionedMultiLocation,
    transferToken.picassoId,
    amount,
    hasFeeToken,
    feeTokenId
  );

  return call;
}

export function getTransferCallPicassoKusama(
  api: ApiPromise,
  targetAccount: string,
  hasFeeToken: boolean,
  transferToken: TokenMetadata,
  amount: u128,
  feeTokenId: BigNumber | null
) {
  if (!transferToken.picassoId) throw new Error("Unsupported Transfer");

  const destination = buildParachainToRelaychainAccountDestination(
    api,
    targetAccount
  );

  const call = getXTokenTransferCallOriginPicasso(
    api,
    destination as XcmVersionedMultiLocation,
    transferToken.picassoId,
    amount,
    hasFeeToken,
    feeTokenId
  );

  return call;
}

export function getTransferCallKaruraPicasso(
  api: ApiPromise,
  targetChain: number | 0,
  targetAccount: string,
  currency: string | null,
  amount: u128
) {
  if (!currency) throw new Error("Unsupported transfer");

  const destination = buildParachainToParachainAccountDestination(
    api,
    targetChain,
    targetAccount
  );

  const call = getParachainDestinationCallOriginKarura(
    api,
    destination as XcmVersionedMultiLocation,
    currency,
    amount
  );

  return call;
}

export async function transferPicassoKarura({
  api,
  targetChain,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
  hasFeeItem,
  feeToken,
  transferToken,
  signer,
}: TransferHandlerArgs) {
  // Set destination. Should have 2 Junctions, first to parent and then to wallet
  const call = getTransferCallPicassoKarura(
    api,
    targetChain,
    targetAccount,
    hasFeeItem,
    transferToken,
    amount,
    feeToken.picassoId
  );

  await executor.execute(
    call,
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}

export async function transferKaruraPicasso({
  api,
  targetChain,
  transferToken,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
  signer,
}: TransferHandlerArgs) {
  const call = getTransferCallKaruraPicasso(
    api,
    targetChain,
    targetAccount,
    transferToken.karuraId,
    amount
  );

  await executor.execute(
    call,
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}

export async function transferPicassoKusama({
  api,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
  hasFeeItem,
  feeToken,
  transferToken,
  signer,
}: TransferHandlerArgs) {
  const call = await getTransferCallPicassoKusama(
    api,
    targetAccount,
    hasFeeItem,
    transferToken,
    amount,
    feeToken.picassoId
  );

  await executor.execute(
    call,
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}

export async function transferKusamaPicasso({
  api,
  targetChain,
  signer,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
}: TransferHandlerArgs) {
  const call = await getTransferCallKusamaPicasso(
    api,
    targetChain,
    targetAccount,
    amount
  );

  await executor.execute(
    call,
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Executing transfer...", {
        persist: true,
        variant: "info",
        timeout: 0,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}
