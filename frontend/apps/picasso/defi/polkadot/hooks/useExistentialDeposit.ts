import { useStore } from "@/stores/root";
import { useEffect } from "react";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { useTransfer } from "@/defi/polkadot/hooks/useTransfer";
import { getKaruraExistentialDeposit } from "../pallets/Assets/ExistentialDeposits/karura";
import { fetchAccountExistentialDeposit } from "../pallets/Assets/ExistentialDeposits/picasso";
import { extractTokenByNetworkIdentifier } from "../pallets/Assets";

export const useExistentialDeposit = () => {
  const { from, balance, to, account, fromProvider } = useTransfer();
  const tokenId = useStore((state) => state.transfers.selectedToken);
  const updateFeeToken = useStore((state) => state.transfers.updateFeeToken);
  const getFeeToken = useStore((state) => state.transfers.getFeeToken);
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const { updateExistentialDeposit, existentialDeposit } = useStore(
    (state) => state.transfers
  );

  const { parachainApi } = fromProvider;

  /**
   * Fetch transfer token based on user config (only on karura and picasso)
   * for kusama transfer token is ksm
   *
   * If no transfer fee token is specified, fallback to native token
   */
  useEffect(() => {
    switch (from) {
      case "karura":
        const ed = getKaruraExistentialDeposit(tokenId);
        const assetOnChain = tokens[tokenId].karuraId;
        if (assetOnChain) {
          updateFeeToken(tokenId);
        }
        updateExistentialDeposit(ed);
        break;
      case "picasso":
        callbackGate(
          async function updateTransferFeeRequirements(api, address, _picaId) {
            const paymentAsset = await fetchAccountExistentialDeposit(
              api,
              address,
              _picaId
            );
            const asset = extractTokenByNetworkIdentifier(
              tokens,
              "picasso",
              paymentAsset.assetId
            );

            updateExistentialDeposit(paymentAsset.existentialDeposit);
            if (asset) {
              updateFeeToken(asset.id);
            }
          },
          parachainApi,
          account?.address,
          tokens.pica.picassoId
        );
        break;
      case "kusama":
        callbackGate(async (api) => {
          const ed = api.consts.balances.existentialDeposit.toString();
          updateExistentialDeposit(fromChainIdUnit(unwrapNumberOrHex(ed)));
          updateFeeToken("ksm");
        }, parachainApi);
        break;
      default:
        console.log(from);
        break;
    }
  }, [
    from,
    to,
    account,
    tokenId,
    parachainApi,
    updateExistentialDeposit,
    updateFeeToken,
    tokens,
  ]);

  return {
    balance,
    tokenId,
    from,
    to,
    existentialDeposit,
    feeToken: getFeeToken(from),
  };
};
