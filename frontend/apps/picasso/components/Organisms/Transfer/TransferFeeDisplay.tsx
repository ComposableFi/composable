import {
  callbackGate,
  fromChainIdUnit,
  humanBalance,
  unwrapNumberOrHex
} from "shared";
import { FeeDisplay } from "@/components";
import React, { useCallback, useEffect, useMemo } from "react";
import { useStore } from "@/stores/root";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useExecutor } from "substrate-react";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";
import {
  getAmountToTransfer,
  getApiCallAndSigner
} from "@/defi/polkadot/pallets/Transfer";
import { useExistentialDeposit } from "@/components/Organisms/Transfer/hooks";

export const TransferFeeDisplay = () => {
  const from = useStore(state => state.transfers.networks.from);
  const to = useStore(state => state.transfers.networks.to);
  const allProviders = useAllParachainProviders();
  const provider = allProviders[from];
  const executor = useExecutor();
  const account = useSelectedAccount();
  const assets = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from].assets
  );
  const amount = useStore(state => state.transfers.amount);
  const { hasFeeItem, feeItem } = useStore(({ transfers }) => transfers);
  const selectedRecipient = useStore(
    state => state.transfers.recipients.selected
  );
  const keepAlive = useStore(state => state.transfers.keepAlive);
  const { existentialDeposit, feeToken, balance } = useExistentialDeposit();
  const { updateFee, fee } = useStore(({ transfers }) => transfers);

  const symbol = useMemo(() => {
    let out;
    if (hasFeeItem && feeItem.length > 0) {
      out = feeItem as AssetId;
    } else if ("assetId" in feeToken) {
      out = feeToken.assetId;
    } else {
      out = feeToken.id;
    }
    return out;
  }, [feeItem, feeToken, hasFeeItem]);

  const feeItemId = useMemo(() => {
    return assets[symbol as AssetId].meta.supportedNetwork[from];
  }, [assets, from, symbol]);

  const calculateFee = useCallback(() => {
    callbackGate(
      async (api, exec, acc) => {
        const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
          ? selectedRecipient
          : acc.address;

        const TARGET_PARACHAIN_ID = SUBSTRATE_NETWORKS[to].parachainId;

        // Set amount to transfer
        const amountToTransfer = getAmountToTransfer({
          balance,
          amount,
          existentialDeposit,
          keepAlive,
          api
        });

        const signerAddress = acc.address;

        const { call, signer } = await getApiCallAndSigner(
          api,
          TARGET_ACCOUNT_ADDRESS,
          amountToTransfer,
          feeItemId,
          signerAddress,
          TARGET_PARACHAIN_ID,
          from,
          to,
          hasFeeItem
        );

        const info = await exec.paymentInfo(call, acc.address, signer);
        updateFee({
          class: info.class.toString(),
          partialFee: fromChainIdUnit(
            unwrapNumberOrHex(info.partialFee.toString())
          ),
          weight: unwrapNumberOrHex(info.weight.toString())
        } as {
          class: string;
          partialFee: BigNumber;
          weight: BigNumber;
        });
        console.log(
          JSON.stringify({
            class: info.class.toString(),
            partialFee: fromChainIdUnit(
              unwrapNumberOrHex(info.partialFee.toString())
            ),
            weight: unwrapNumberOrHex(info.weight.toString())
          })
        );
      },
      provider.parachainApi,
      executor,
      account,
      hasFeeItem && feeItem.length === 0
    );
  }, [
    account,
    amount,
    balance,
    executor,
    existentialDeposit,
    feeItem.length,
    feeItemId,
    from,
    keepAlive,
    provider.parachainApi,
    selectedRecipient,
    to,
    updateFee
  ]);

  useEffect(() => {
    calculateFee();
  }, [calculateFee]);

  return (
    <FeeDisplay
      label="Fee"
      feeText={`${humanBalance(fee.partialFee)} ${symbol.toUpperCase()}`}
      TooltipProps={{
        title: "Fee tooltip title"
      }}
    />
  );
};
