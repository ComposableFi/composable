import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { ApiPromise } from "@polkadot/api";
import { Signer } from "@polkadot/api/types";
import { Executor } from "substrate-react";
import { extractTokenByNetworkIdentifier } from "../Assets";
import { SubstrateNetworkId } from "shared";

export type SetPaymentAssetArgs = {
  api: ApiPromise;
  signer: Signer;
  walletAddress: string;
  assetId: string | number;
  executor: Executor;
  onReady: (txHash: string) => void;
  onError: (error: string) => void;
  onSuccess: (txHash: string) => void;
};

export async function setPaymentAsset({
  api,
  signer,
  walletAddress,
  assetId,
  executor,
  onSuccess,
  onError,
  onReady,
}: SetPaymentAssetArgs) {
  return executor.execute(
    api.tx.assetTxPayment.setPaymentAsset(
      api.createType("AccountId32", walletAddress),
      Number(assetId) === 1 ? null : api.createType("u128", assetId)
    ),
    walletAddress,
    api,
    signer,
    onReady,
    onSuccess,
    onError
  );
}

export type GetPaymentAssetArgs = {
  api: ApiPromise;
  walletAddress: string;
  network: SubstrateNetworkId;
  tokens: Record<TokenId, TokenMetadata>;
};

export async function getPaymentAsset({
  api,
  walletAddress,
  network,
  tokens,
}: GetPaymentAssetArgs) {
  if (network === "picasso") {
    try {
      const result: any = await api.query.assetTxPayment.paymentAssets(
        api.createType("AccountId32", walletAddress)
      );

      if (result.isSome) {
        const [assetId] = result.toJSON();
        if (assetId) {
          const asset = extractTokenByNetworkIdentifier(
            tokens,
            network,
            assetId
          );
          if (asset) {
            return asset;
          }
        }
      }
      return tokens.pica;
    } catch (e) {
      console.error("Error while trying to access assetTxPayment pallet", e);
      return tokens.pica;
    }
  }
  if (network === "karura") {
    return tokens.kar;
  }

  return tokens.ksm;
}
