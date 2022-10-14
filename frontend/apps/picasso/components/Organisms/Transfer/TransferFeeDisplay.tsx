import {
  callbackGate,
  fromChainIdUnit,
  humanBalance,
  unwrapNumberOrHex,
} from "shared";
import { FeeDisplay } from "@/components";
import React, { useCallback, useEffect, useMemo } from "react";
import { useStore } from "@/stores/root";
import { useExecutor } from "substrate-react";
import { useTransfer } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";
import {
  getAmountToTransfer,
  getApiCallAndSigner,
  getDestChainFee,
} from "@/defi/polkadot/pallets/Transfer";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import { getPaymentAsset } from "@/defi/polkadot/pallets/AssetTxPayment";
import { AssetMetadata } from "@/defi/polkadot/Assets";
import { Stack } from "@mui/material";

export const TransferFeeDisplay = () => {
  const { amount, from, to, balance, account, fromProvider } = useTransfer();
  const executor = useExecutor();
  const assets = useStore(
    ({ substrateBalances }) => substrateBalances.assets[from].assets
  );
  const feeItem = useStore((state) => state.transfers.feeItem);
  const hasFeeItem = useStore((state) => state.transfers.hasFeeItem);
  const setFeeItem = useStore((state) => state.transfers.setFeeItem);
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const { existentialDeposit, feeToken } = useExistentialDeposit();
  const fee = useStore((state) => state.transfers.fee);
  const destFee = getDestChainFee(from, to);
  const updateFee = useStore((state) => state.transfers.updateFee);

  const symbol = useMemo(() => {
    let out;
    if (hasFeeItem && feeItem.length > 0) {
      out = feeItem;
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
      async (api, exec, acc, hasFeeItem) => {
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
          api,
          sourceChain: from,
          targetChain: to,
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
          weight: unwrapNumberOrHex(info.weight.toString()),
        } as {
          class: string;
          partialFee: BigNumber;
          weight: BigNumber;
        });
      },
      fromProvider.parachainApi,
      executor,
      account,
      hasFeeItem && feeItem.length === 0
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    account,
    amount,
    feeItem,
    feeItemId,
    from,
    fromProvider.parachainApi,
    to,
  ]);

  useEffect(() => {
    calculateFee();
  }, [calculateFee, amount, from]);

  useEffect(() => {
    const asset: Promise<AssetMetadata> = callbackGate(
      (api, walletAddress) =>
        getPaymentAsset({
          api,
          walletAddress,
          network: from,
        }),
      fromProvider.parachainApi,
      account?.address
    );
    asset.then((paymentAsset) => {
      if ("assetId" in paymentAsset) {
        setFeeItem(paymentAsset.assetId);
      }
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fromProvider.parachainApi, account?.address, from]);

  return (
    <Stack direction="column" gap={4}>
      <FeeDisplay
        label="Fee"
        feeText={`${humanBalance(fee.partialFee)} ${symbol.toUpperCase()}`}
        TooltipProps={{
          title: "Fee tooltip title",
        }}
      />
      <FeeDisplay
        label="Destination chain fee"
        feeText={`${destFee.fee.toFormat()} ${destFee.symbol.symbol}`}
        TooltipProps={{
          title:
            "Destination transaction fee is approximate and might change due to network conditions",
        }}
      />
    </Stack>
  );
};
