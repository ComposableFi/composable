import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import {
  getTransferCallKaruraPicasso,
  getTransferCallKusamaPicasso,
  getTransferCallPicassoKarura,
  getTransferCallPicassoKusama,
} from "@/defi/polkadot/pallets/xcmp";
import { fromChainIdUnit, toChainIdUnit } from "shared";
import { ParachainId, RelayChainId } from "substrate-react";
import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import BigNumber from "bignumber.js";

export type XCMTransferCallArgs = {
  api: ApiPromise,
  targetAccountAddress: string,
  amountToTransfer: u128,
  feeToken: TokenMetadata,
  transferToken: TokenMetadata,
  targetParachainId: number,
  from: SubstrateNetworkId,
  to: SubstrateNetworkId,
  hasFeeItem: boolean
}

export async function getXCMTransferCall({
  api,
  targetAccountAddress,
  amountToTransfer,
  feeToken,
  transferToken,
  targetParachainId,
  from,
  to,
  hasFeeItem
}: XCMTransferCallArgs) {
  switch (`${from}-${to}`) {
    case "picasso-kusama":
      return getTransferCallPicassoKusama(
        api,
        targetAccountAddress,
        transferToken,
        amountToTransfer,
        feeToken.picassoId
      );
    case "picasso-karura":
      return getTransferCallPicassoKarura(
        api,
        targetParachainId,
        targetAccountAddress,
        transferToken,
        amountToTransfer,
        feeToken.picassoId
      );
    case "kusama-picasso":
      return getTransferCallKusamaPicasso(
        api,
        targetParachainId,
        targetAccountAddress,
        amountToTransfer
      );
    case "karura-picasso":
      return getTransferCallKaruraPicasso(
        api,
        targetParachainId,
        targetAccountAddress,
        transferToken.karuraId,
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
  tokens
}: {
  balance: BigNumber;
  amount: BigNumber;
  existentialDeposit: BigNumber;
  keepAlive: boolean;
  api: ApiPromise;
  sourceChain: ParachainId | RelayChainId;
  targetChain: ParachainId | RelayChainId;
  tokens: Record<TokenId, TokenMetadata>
}): u128 {
  const isExistentialDepositImportant = balance
    .minus(amount)
    .lte(existentialDeposit);
  const isZeroAmount =
    keepAlive &&
    isExistentialDepositImportant &&
    amount.minus(existentialDeposit).lte(0);
  const destinationFee = getDestChainFee(sourceChain, targetChain, tokens);
  const calculatedAmount =
    keepAlive && isExistentialDepositImportant && !isZeroAmount
      ? amount.minus(existentialDeposit)
      : amount;
  const sendAmount = destinationFee.fee.gt(0)
    ? calculatedAmount.plus(destinationFee.fee)
    : calculatedAmount;

  return api.createType("u128", toChainIdUnit(sendAmount, 12).toString());
}

export function getDestChainFee(
  sourceChain: ParachainId | RelayChainId,
  targetChain: ParachainId | RelayChainId,
  tokens: Record<TokenId, TokenMetadata>
) {
  switch (`${sourceChain}=>${targetChain}`) {
    case "kusama=>picasso":
      return {
        fee: fromChainIdUnit(new BigNumber("7536750")),
        token: tokens.ksm
      };
    case "karura=>picasso":
      return {
        fee: fromChainIdUnit(new BigNumber("927020325")),
        token: tokens.kusd
      };
    case "picasso=>karura":
      return {
        fee: fromChainIdUnit(new BigNumber("74592000000")),
        token: tokens.kusd
      };
    case "picasso=>kusama":
      return {
        fee: fromChainIdUnit(new BigNumber("51105801784")),
        token: tokens.ksm
      };
    default:
      return {
        fee: new BigNumber(0),
        token: tokens.pica
      };
  }
}
