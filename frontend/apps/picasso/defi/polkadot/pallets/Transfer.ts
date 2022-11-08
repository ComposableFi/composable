import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";
import { fromChainIdUnit, toChainIdUnit } from "shared";
import { ParachainId, RelayChainId } from "substrate-react";
import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import BigNumber from "bignumber.js";

export function getAmountToTransfer({
  balance,
  amount,
  existentialDeposit,
  keepAlive,
  api,
  sourceChain,
  targetChain,
}: {
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
  const calculatedAmount =
    keepAlive && isExistentialDepositImportant && !isZeroAmount
      ? amount.minus(existentialDeposit)
      : amount;

  return api.createType("u128", toChainIdUnit(calculatedAmount, 12).toString());
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
        token: tokens.ksm,
      };
    case "karura=>picasso":
      return {
        fee: fromChainIdUnit(new BigNumber("927020")),
        token: tokens.kusd,
      };
    case "picasso=>karura":
      return {
        fee: fromChainIdUnit(new BigNumber("74592000000")),
        token: tokens.kusd,
      };
    case "picasso=>kusama":
      return {
        fee: fromChainIdUnit(new BigNumber("51105801784")),
        token: tokens.ksm,
      };
    default:
      return {
        fee: new BigNumber(0),
        token: tokens.pica,
      };
  }
}
