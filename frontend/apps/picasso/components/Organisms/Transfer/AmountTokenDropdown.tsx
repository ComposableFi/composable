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
import { FC, useEffect, useState } from "react";
import { useValidation } from "@/components/Molecules/BigNumberInput/hooks";
import BigNumber from "bignumber.js";
import { Typography } from "@mui/material";
import {
  calculateTransferAmount,
  getAmountToTransfer,
} from "@/defi/polkadot/pallets/Transfer";
import { useTransfer } from "@/defi/polkadot/hooks";

export const AmountTokenDropdown: FC<{ disabled: boolean }> = ({
  disabled,
}) => {
  const updateAmount = useStore((state) => state.transfers.updateAmount);
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
  const tokens = useStore((state) => state.substrateTokens.tokens);
  const existentialDeposit = tokens[selectedToken].existentialDeposit[from];
  const decimals = tokens[selectedToken].decimals[from] ?? Number(0);
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const [stringValue, setStringValue] = useState<string>(amount.toString());
  const { validate, hasError } = useValidation({
    maxValue: balance,
    maxDec: 12,
    initialValue: new BigNumber(0),
  });
  const fee = useStore((state) => state.transfers.fee);
  const feeToken = useStore((state) => state.transfers.feeToken);
  const sourceGas = {
    fee: fee.partialFee,
    token: feeToken,
  };

  const handleMaxClick = () => {
    callbackGate(
      (api, _existentialDeposit) => {
        const amountToTransfer = calculateTransferAmount({
          amountToTransfer: balance,
          balance: balance,
          keepAlive,
          selectedToken,
          sourceExistentialDeposit: _existentialDeposit,
          sourceGas,
        });
        updateAmount(amountToTransfer);
        setStringValue(amountToTransfer.toString());
        validate({
          target: {
            value: amountToTransfer.toString(),
          },
        } as any);
      },
      fromProvider.parachainApi,
      existentialDeposit
    );
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
          disabled,
        }}
        InputProps={{
          sx: amountInputStyle,
          disabled,
        }}
        CombinedSelectProps={{
          options: tokenOptions.map((token) => ({
            ...token,
            disabled: isTokenBalanceZero(token.tokenId),
          })),
          value: selectedToken,
          setValue: updateSelectedToken,
          disabled,
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
