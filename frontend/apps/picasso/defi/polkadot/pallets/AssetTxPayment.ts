import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { ApiPromise } from "@polkadot/api";
import { Signer } from "@polkadot/api/types";
import BigNumber from "bignumber.js";
import {
  Executor,
  ParachainId,
  RelayChainId,
} from "substrate-react";
import { extractTokenByNetworkIdentifier } from "./Assets";

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
  tokens: Record<TokenId, TokenMetadata>;
};

export async function getPaymentAsset({
  api,
  walletAddress,
  network,
  tokens,
}: GetPaymentAssetArgs) {
  if ("assetTxPayment" in api.query) {
    /**
     * Identify behavior on
     * all chains the 
     * we would support
     */
    const result: any = await api.query.assetTxPayment.paymentAssets(
      api.createType("AccountId32", walletAddress)
    );

    if (result.isSome) {
      /**
       * TODO
       * This can cause weird behavior
       * we need to address the types returned
       * by paymentAssets
       * for picasso assetId type is BigNumber
       * for karura its symbol in string
       * for kusama its js number
       */
      const [assetId, _] = result.toJSON();
      if (assetId) {
        /**
         * Needs to change
         * Not a good approach
         */
        const asset = extractTokenByNetworkIdentifier(tokens, network, assetId);
        if (asset) {
          return asset;
        }
      }
    }
    return tokens.pica;
  }

  return tokens.pica;
}
