import { useStore } from "@/stores/root";
import { useEffect, useMemo } from "react";
import { getTransferToken } from "@/components/Organisms/Transfer/xcmp";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import BigNumber from "bignumber.js";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { useSelectedAccount } from "@/defi/polkadot/hooks";

export const useExistentialDeposit = () => {
  const tokenId = useStore((state) => state.transfers.tokenId);
  const from = useStore((state) => state.transfers.networks.from);
  const to = useStore((state) => state.transfers.networks.to);
  const allProviders = useAllParachainProviders();
  const account = useSelectedAccount();
  const updateFeeToken = useStore((state) => state.transfers.updateFeeToken);
  const getFeeToken = useStore((state) => state.transfers.getFeeToken);

  const { native, assets } = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from]
  );

  const nativeTo = useStore(
    ({ substrateBalances }) => substrateBalances.assets[to].native
  );

  const { updateExistentialDeposit, existentialDeposit } = useStore(
    (state) => state.transfers
  );

  const isNativeToNetwork = useMemo(() => {
    const transferableTokenId = getTransferToken(from, to);
    return assets[transferableTokenId].meta.supportedNetwork[from] === 1;
  }, [assets, from, to]);

  const balance = isNativeToNetwork ? native.balance : assets[tokenId].balance;

  const parachainApi = allProviders[from]?.parachainApi;

  /**
   * Fetch transfer token based on user config (only on karura and picasso)
   * for kusama transfer token is ksm
   *
   * If no transfer fee token is specified, fallback to native token
   */
  useEffect(() => {
    switch (from) {
      case "karura":
        if (["kusd", "ausd"].includes(tokenId)) {
          updateExistentialDeposit(new BigNumber(1));
        }
        break;
      case "picasso":
        callbackGate(
          async function updateTransferFeeRequirements(api, address) {
            const result: any = await api.query.assetTxPayment.paymentAssets(
              api.createType("AccountId32", address)
            );
            if (result.isNone) {
              // Fetch native asset's ED
              const ed = await api.query.currencyFactory.assetEd(
                assets[tokenId].meta.supportedNetwork[from]
              );
              const existentialString = ed.toString();
              const existentialValue = fromChainIdUnit(
                new BigNumber(existentialString)
              );
              updateExistentialDeposit(
                existentialValue.isNaN() ? new BigNumber(0) : existentialValue
              );
              updateFeeToken(1);
              return;
            }
            const [assetId, existentialDeposit] = result.toJSON();
            updateExistentialDeposit(
              fromChainIdUnit(unwrapNumberOrHex(existentialDeposit))
            );
            updateFeeToken(Number(assetId));
          },
          parachainApi,
          account?.address
        );
        break;
      case "kusama":
        callbackGate(async (api) => {
          console.log("yes signer");
          const ed = api.consts.balances.existentialDeposit.toString();
          updateExistentialDeposit(fromChainIdUnit(unwrapNumberOrHex(ed)));
          updateFeeToken(Number(1));
        }, parachainApi);
        console.log("KUSAMA");
        break;
      default:
        console.log(from);
        break;
    }
  }, [from, to, account]);

  return {
    balance,
    tokenId,
    isNativeToNetwork,
    from,
    to,
    assets,
    native,
    existentialDeposit,
    feeToken: getFeeToken(from),
  };
};
