import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import {
  getTransferCallKaruraPicasso,
  getTransferCallKusamaPicasso,
  getTransferCallPicassoKarura,
  getTransferCallPicassoKusama
} from "@/components/Organisms/Transfer/xcmp";

export async function getApiCallAndSigner(
  api: ApiPromise,
  targetAccountAddress: string,
  amountToTransfer: u128,
  feeItemId: number | null,
  signerAddress: string,
  targetParachainId: number,
  from: SubstrateNetworkId,
  to: SubstrateNetworkId,
  hasFeeItem: boolean
) {
  switch (`${from}-${to}`) {
    case "picasso-kusama":
      return getTransferCallPicassoKusama(
        api,
        targetAccountAddress,
        amountToTransfer,
        feeItemId,
        signerAddress,
        hasFeeItem
      );
    case "picasso-karura":
      return getTransferCallPicassoKarura(
        api,
        targetParachainId,
        targetAccountAddress,
        hasFeeItem,
        signerAddress,
        amountToTransfer,
        feeItemId
      );
    case "kusama-picasso":
      return getTransferCallKusamaPicasso(
        api,
        targetParachainId,
        targetAccountAddress,
        amountToTransfer,
        signerAddress
      );
    case "karura-picasso":
      return getTransferCallKaruraPicasso(
        api,
        targetParachainId,
        targetAccountAddress,
        signerAddress,
        amountToTransfer
      );
    default:
      throw new Error("Invalid network");
  }
};
