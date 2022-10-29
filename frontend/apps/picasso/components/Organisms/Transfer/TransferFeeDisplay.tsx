import {
  callbackGate,
  fromChainIdUnit,
  humanBalance,
  unwrapNumberOrHex,
} from "shared";
import { FeeDisplay } from "@/components";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";
import { useExecutor, useSigner } from "substrate-react";
import { useTransfer } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

import {
  getAmountToTransfer,
  getXCMTransferCall,
  getDestChainFee,
} from "@/defi/polkadot/pallets/Transfer";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import { getPaymentAsset } from "@/defi/polkadot/pallets/AssetTxPayment";
import { Stack } from "@mui/material";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import BigNumber from "bignumber.js";

export const TransferFeeDisplay = () => {
  const signer = useSigner();
  const executor = useExecutor();
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const feeItem = useStore((state) => state.transfers.feeItem);

  const setFeeItem = useStore((state) => state.transfers.setFeeItem);

  const { amount, from, to, balance, account, fromProvider, transferToken } = useTransfer();
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const { existentialDeposit, feeToken } = useExistentialDeposit();
  const fee = useStore((state) => state.transfers.fee);
  const destFee = getDestChainFee(from, to, tokens);
  const updateFee = useStore((state) => state.transfers.updateFee);
  const token = useStore(state => state.transfers.selectedToken);


  const calculateFee = useCallback(() => {
    callbackGate(
      async (api, exec, acc, _signer) => {
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
          tokens,
          tokenId: token
        });
        
        try {
          const call = await getXCMTransferCall({
            api,
            targetAccountAddress: TARGET_ACCOUNT_ADDRESS,
            amountToTransfer,
            feeToken,
            transferToken: transferToken,
            targetParachainId: TARGET_PARACHAIN_ID,
            from,
            to
          });
  
          const info = await exec.paymentInfo(call, acc.address, _signer);
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
        } catch (err) {
          console.error('[TransferFeeDisplay] ', err);
        }
      },
      fromProvider.parachainApi,
      executor,
      account,

      signer
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    account,
    amount,
    feeItem,
    tokens,
    from,
    fromProvider.parachainApi,
    to,
    signer
  ]);

  useEffect(() => {
    calculateFee();
  }, [calculateFee, amount, from]);

  useEffect(() => {
    if (fromProvider.parachainApi && account) {
      getPaymentAsset({
        api: fromProvider.parachainApi,
        walletAddress: account.address,
        network: from,
        tokens
      }).then((token: TokenMetadata) => {
        setFeeItem(token.id);
      })
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fromProvider.parachainApi, account?.address, from]);

  return (
    <Stack direction="column" gap={4}>
      <FeeDisplay
        label="Fee"
        feeText={`${humanBalance(fee.partialFee)} ${tokens[feeItem].symbol}`}
        TooltipProps={{
          title: "Fee tooltip title",
        }}
      />
      <FeeDisplay
        label="Destination chain fee"
        feeText={`${destFee.fee.toFormat()} ${destFee.token.symbol}`}
        TooltipProps={{
          title:
            "Destination transaction fee is approximate and might change due to network conditions",
        }}
      />
    </Stack>
  );
};
