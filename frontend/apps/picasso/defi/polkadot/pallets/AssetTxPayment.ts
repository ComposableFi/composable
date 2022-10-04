import { ApiPromise } from "@polkadot/api";
import { Executor, getSigner } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { Assets } from "@/defi/polkadot/Assets";

export type SetPaymentAssetArgs = {
  api: ApiPromise;
  walletAddress: string;
  assetId: string | number;
  executor: Executor;
  onReady: (txHash: string) => void;
  onError: (error: string) => void;
  onSuccess: (txHash: string) => void;
};

export async function setPaymentAsset({
  api,
  walletAddress,
  assetId,
  executor,
  onSuccess,
  onError,
  onReady
}: SetPaymentAssetArgs) {
  const signer = await getSigner(APP_NAME, walletAddress);
  return executor.execute(
    api.tx.assetTxPayment.setPaymentAsset(
      api.createType("AccountId32", walletAddress),
      api.createType("u128", assetId)
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
};

export async function getPaymentAsset({
  api,
  walletAddress
}: GetPaymentAssetArgs) {
  const result: any = await api.query.assetTxPayment.paymentAssets(
    api.createType("AccountId32", walletAddress)
  );

  if (result.isSome) {
    console.log("Got the value", result.toJSON());
    return Assets.pica;
  }

  return Assets.pica;
}
