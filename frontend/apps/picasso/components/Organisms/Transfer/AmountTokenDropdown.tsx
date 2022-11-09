import { TokenDropdownCombinedInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import { callbackGate, humanBalance } from "shared";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import {
  subscribeDefaultTransferToken,
  subscribeTokenOptions,
} from "@/stores/defi/polkadot/transfers/subscribers";
import { FC, useEffect } from "react";
import { useValidation } from "@/components/Molecules/BigNumberInput/hooks";
import { Typography } from "@mui/material";
import { calculateTransferAmount } from "@/defi/polkadot/pallets/Transfer";
import { useTransfer } from "@/defi/polkadot/hooks";
import BigNumber from "bignumber.js";

export const AmountTokenDropdown: FC<{ disabled: boolean }> = ({
  disabled,
}) => {
  const updateAmount = useStore((state) => state.transfers.updateAmount);
  const amount = useStore((state) => state.transfers.amount);
  const { balance, tokenId } = useExistentialDeposit();
  const { from, fromProvider, to } = useTransfer();
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
  const { validate, hasError, stringValue, bignrValue, setValue } =
    useValidation({
      maxValue: balance,
      maxDec: 12,
      initialValue: amount,
    });
  const fee = useStore((state) => state.transfers.fee);
  const feeToken = useStore((state) => state.transfers.feeToken);
  const sourceGas = {
    fee: fee.partialFee,
    token: feeToken,
  };
  const setFormError = useStore((state) => state.transfers.setFormError);

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
        setValue(amountToTransfer);
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
    // On network or token change, reset the amount
    setValue(new BigNumber(0));
  }, [from, to, selectedToken]);

  useEffect(() => {
    setFormError(hasError);
  }, [hasError]);

  // Update the amount based on user input
  useEffect(() => {
    if (!bignrValue.eq(amount)) {
      updateAmount(bignrValue);
    }
  }, [bignrValue.toString()]);

  // Update internal value based on external amount changes. (post transfer hooks, etc)
  useEffect(() => {
    if (!amount.eq(bignrValue)) {
      setValue(amount);
    }
  }, [amount.toString()]);

  return (
    <>
      <TokenDropdownCombinedInput
        buttonLabel="Max"
        value={stringValue}
        onChange={validate}
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
