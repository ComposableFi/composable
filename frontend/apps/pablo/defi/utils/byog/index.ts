import { ApiPromise } from "@polkadot/api";
import { Signer } from "@polkadot/api/types";
import { Executor, ParachainId, RelayChainId } from "substrate-react";
import { TokenId } from "tokens";
import { Asset } from "shared";

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
  network: ParachainId | Extract<"kusama", RelayChainId>;
  tokens: Record<TokenId, Asset>;
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
          const [tokenId, asset] =
            Object.entries<Asset>(tokens).find(([_, token]) => {
              return (
                token.getIdOnChain("picasso")?.toString() === assetId.toString()
              );
            }) ?? [];

          if (asset) {
            return tokenId as TokenId;
          }
        }
      }
      return "pica" as TokenId;
    } catch (e) {
      console.error("Error while trying to access assetTxPayment pallet", e);
      return "pica" as TokenId;
    }
  }
  if (network === "karura") {
    return "kar" as TokenId;
  }

  return "ksm";
}
