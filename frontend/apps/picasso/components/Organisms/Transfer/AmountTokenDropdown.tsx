import { TokenDropdownCombinedInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import {
  callbackGate,
  fromChainIdUnit,
  humanBalance,
  unwrapNumberOrHex,
} from "shared";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import {
  subscribeDefaultTransferToken,
  subscribeTokenOptions,
} from "@/stores/defi/polkadot/transfers/subscribers";
import { useEffect, useState } from "react";
import { useValidation } from "@/components/Molecules/BigNumberInput/hooks";
import BigNumber from "bignumber.js";
import { Typography } from "@mui/material";
import {
  getAmountToTransfer,
  getDestChainFee,
} from "@/defi/polkadot/pallets/Transfer";
import { useTransfer } from "@/defi/polkadot/hooks";

export const AmountTokenDropdown = () => {
  const updateAmount = useStore((state) => state.transfers.updateAmount);
  const tokens = useStore((state) => state.substrateTokens.tokens);
  const amount = useStore((state) => state.transfers.amount);
  const { balance, tokenId } = useExistentialDeposit();
  const { from, fromProvider } = useTransfer();
  const tokenOptions = useStore((state) => state.transfers.tokenOptions);
  const selectedToken = useStore((state) => state.transfers.selectedToken);
  const updateSelectedToken = useStore(
    (state) => state.transfers.updateSelectedToken
  );
  const isTokenBalanceZero = useStore(
    (state) => state.transfers.isTokenBalanceZero
  );
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const { existentialDeposit } = useExistentialDeposit();
  const [stringValue, setStringValue] = useState<string>(amount.toString());
  const { validate, hasError } = useValidation({
    maxValue: balance,
    maxDec: 12,
    initialValue: new BigNumber(0),
  });

  const handleMaxClick = () => {
    callbackGate((api) => {
      const amountToTransfer = getAmountToTransfer({
        amount: balance,
        api,
        balance,
        existentialDeposit,
        keepAlive: keepAlive,
        sourceChain: from,
        targetChain: "picasso",
        tokens,
      });
      const amount = fromChainIdUnit(
        unwrapNumberOrHex(amountToTransfer.toString())
      );
      updateAmount(amount);
      setStringValue(amount.toString());
      validate({
        target: {
          value: amount.toString(),
        },
      } as any);
    }, fromProvider.parachainApi);
  };

  useEffect(() => {
    const unsubscribeTokenOptions = subscribeTokenOptions();
    const unsubscribeDefaultTransferToken = subscribeDefaultTransferToken();

    return () => {
      unsubscribeTokenOptions();
      unsubscribeDefaultTransferToken();
    };
  }, []);

  useEffect(() => {
    updateAmount(new BigNumber(hasError ? 0 : stringValue));
  }, [hasError, stringValue, updateAmount]);

  return (
    <>
      <TokenDropdownCombinedInput
        buttonLabel="Max"
        value={stringValue}
        onChange={(e) => {
          validate(e);
          setStringValue(e.target.value);
        }}
        LabelProps={{
          mainLabelProps: {
            label: "Amount",
          },
          balanceLabelProps: {
            label: "Balance:",
            balanceText: humanBalance(balance) + " " + tokenId.toUpperCase(),
          },
        }}
        ButtonProps={{
          onClick: handleMaxClick,
        }}
        InputProps={{
          sx: amountInputStyle,
        }}
        CombinedSelectProps={{
          options: tokenOptions.map((token) => ({
            ...token,
            disabled: isTokenBalanceZero(token.tokenId),
          })),
          value: selectedToken,
          setValue: updateSelectedToken,
        }}
      />
      {hasError && (
        <Typography variant="caption" color="error.main">
          Please input a valid number
        </Typography>
      )}
    </>
  );
};
