import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";
import { AssetId, SubstrateNetworkId } from "@/defi/polkadot/types";
import {
  getTransferCallKaruraPicasso,
  getTransferCallKusamaPicasso,
  getTransferCallPicassoKarura,
  getTransferCallPicassoKusama
} from "@/defi/polkadot/pallets/xcmp";
import { fromChainIdUnit, toChainIdUnit } from "shared";
import BigNumber from "bignumber.js";
import { ParachainId, RelayChainId } from "substrate-react";
import { Assets } from "@/defi/polkadot/Assets";

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
}

export function getAmountToTransfer({
  balance,
  amount,
  existentialDeposit,
  keepAlive,
  api,
  sourceChain,
  targetChain,
  tokenId,
}: {
  tokenId: AssetId;
  balance: BigNumber;
  amount: BigNumber;
  existentialDeposit: BigNumber;
  keepAlive: boolean;
  api: ApiPromise;
  sourceChain: ParachainId | RelayChainId;
  targetChain: ParachainId | RelayChainId;
}): u128 {
  const isExistentialDepositImportant = balance
    .minus(amount)
    .lte(existentialDeposit);
  const isZeroAmount =
    keepAlive &&
    isExistentialDepositImportant &&
    amount.minus(existentialDeposit).lte(0);
  const destinationFee = getDestChainFee(sourceChain, targetChain, tokenId);
  const calculatedAmount =
    keepAlive && isExistentialDepositImportant && !isZeroAmount
      ? amount.minus(existentialDeposit)
      : amount;
  const sendAmount = destinationFee.fee.gt(0)
    ? calculatedAmount.plus(destinationFee.fee)
    : calculatedAmount;
  return api.createType("u128", toChainIdUnit(sendAmount).toString());
}

export function getDestChainFee(
  sourceChain: ParachainId | RelayChainId,
  targetChain: ParachainId | RelayChainId,
  tokenId: AssetId
) {
  switch (`${sourceChain}=>${targetChain}`) {
    case "kusama=>picasso":
      return {
        fee: fromChainIdUnit(new BigNumber("7536750")),
        symbol: Assets.ksm
      };
    case "karura=>picasso":
      if (tokenId === "kusd") {
        return {
          fee: fromChainIdUnit(new BigNumber("927020325")),
          symbol: Assets.kusd
        };
      }
      if (tokenId === "kar") {
        return {
          fee: fromChainIdUnit(new BigNumber("927020325")),
          symbol: Assets.kar
        }
      }

      if (tokenId === "ksm") {
        return {
          fee: fromChainIdUnit(new BigNumber("927020325")),
          symbol: Assets.ksm
        }
      }
    case "picasso=>karura":
      return {
        fee: fromChainIdUnit(new BigNumber("74592000000")),
        symbol: Assets.kusd
      };
    case "picasso=>kusama":
      return {
        fee: fromChainIdUnit(new BigNumber("51105801784")),
        symbol: Assets.ksm
      };
    default:
      return {
        fee: new BigNumber(0),
        symbol: Assets.pica
      };
  }
}
