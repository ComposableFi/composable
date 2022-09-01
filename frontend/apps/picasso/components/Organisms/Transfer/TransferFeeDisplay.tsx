import {
  fromChainIdUnit,
  humanBalance,
  toChainIdUnit,
  unwrapNumberOrHex,
} from "shared";
import { FeeDisplay } from "@/components";
import React, { useCallback, useEffect, useMemo } from "react";
import { useStore } from "@/stores/root";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useExecutor } from "substrate-react";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { AssetId } from "@/defi/polkadot/types";
import {
  getTransferCallKaruraPicasso,
  getTransferCallKusamaPicasso,
  getTransferCallPicassoKarura,
  getTransferCallPicassoKusama,
} from "@/components/Organisms/Transfer/xcmp";
import BigNumber from "bignumber.js";

export const TransferFeeDisplay = () => {
  const { tokenId } = useStore(({ transfers }) => transfers);
  const from = useStore((state) => state.transfers.networks.from);
  const to = useStore((state) => state.transfers.networks.to);
  const allProviders = useAllParachainProviders();
  const provider = allProviders[from];
  const executor = useExecutor();
  const account = useSelectedAccount();
  const assets = useStore(
    ({ substrateBalances }) => substrateBalances[from].assets
  );
  const symbol = useStore(
    ({ substrateBalances }) => substrateBalances[from].native.meta.symbol
  );
  const amount = useStore((state) => state.transfers.amount);
  const { hasFeeItem, feeItem } = useStore(({ transfers }) => transfers);
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const existentialDeposit = useStore(
    ({ substrateBalances }) => substrateBalances[from].native.existentialDeposit
  );
  const { updateFee, fee } = useStore(({ transfers }) => transfers);

  const feeItemId =
    hasFeeItem && feeItem.length > 0
      ? assets[feeItem as AssetId].meta.supportedNetwork[from]
      : null;

  const calculateFee = useCallback(() => {
    if (
      !provider.parachainApi ||
      !executor ||
      !account ||
      (hasFeeItem && feeItem.length === 0)
    ) {
      return null;
    }

    const api = provider.parachainApi;

    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    const TARGET_PARACHAIN_ID = SUBSTRATE_NETWORKS[to].parachainId;
    // Set amount to transfer
    const amountToTransfer = api.createType(
      "u128",
      toChainIdUnit(
        keepAlive && amount.gt(existentialDeposit)
          ? amount.minus(existentialDeposit)
          : amount
      ).toString()
    );
    const signerAddress = account.address;

    const getCall = async () => {
      const context = async () => {
        switch (`${from}-${to}`) {
          case "picasso-kusama":
            return await getTransferCallPicassoKusama(
              api,
              TARGET_ACCOUNT_ADDRESS,
              amountToTransfer,
              feeItemId,
              signerAddress,
              hasFeeItem
            );
          case "picasso-karura":
            return await getTransferCallPicassoKarura(
              api,
              TARGET_PARACHAIN_ID,
              TARGET_ACCOUNT_ADDRESS,
              hasFeeItem,
              signerAddress,
              amountToTransfer,
              feeItemId
            );
          case "kusama-picasso":
            return await getTransferCallKusamaPicasso(
              api,
              TARGET_PARACHAIN_ID,
              TARGET_ACCOUNT_ADDRESS,
              amountToTransfer,
              signerAddress
            );
          case "karura-picasso":
            return await getTransferCallKaruraPicasso(
              api,
              TARGET_PARACHAIN_ID,
              TARGET_ACCOUNT_ADDRESS,
              signerAddress,
              amountToTransfer
            );
          default:
            throw new Error("Invalid network");
        }
      };
      const { call, signer } = await context();

      executor.paymentInfo(call, account.address, signer).then((info) => {
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
      });
    };

    getCall();
  }, [
    provider.parachainApi,
    executor,
    account,
    from,
    to,
    amount,
    feeItemId,
    hasFeeItem,
    selectedRecipient,
    keepAlive,
    existentialDeposit,
    updateFee,
    feeItem.length,
  ]);

  useEffect(() => {
    calculateFee();
  }, [calculateFee]);

  return (
    <FeeDisplay
      label="Fee"
      feeText={`${humanBalance(fee.partialFee)} ${symbol}`}
      TooltipProps={{
        title: "Fee tooltip title",
      }}
    />
  );
};
