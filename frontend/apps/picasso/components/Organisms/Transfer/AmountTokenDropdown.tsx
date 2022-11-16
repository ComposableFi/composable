import { TokenDropdownCombinedInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import { callbackGate, humanBalance } from "shared";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import {
  subscribeDefaultTransferToken,
  subscribeTokenOptions,
} from "@/stores/defi/polkadot/transfers/subscribers";
import React, { FC, useEffect, useMemo } from "react";
import { useValidation } from "@/components/Molecules/BigNumberInput/hooks";
import { Tooltip, Typography } from "@mui/material";
import { calculateTransferAmount } from "@/defi/polkadot/pallets/Transfer";
import { useTransfer } from "@/defi/polkadot/hooks";
import BigNumber from "bignumber.js";
import { InfoTwoTone } from "@mui/icons-material";
import { FEE_MULTIPLIER } from "shared/defi/constants";

export const AmountTokenDropdown: FC<{ disabled: boolean }> = ({
  disabled,
}) => {
  const updateAmount = useStore(
    (state) => state.transfers.updateAmount,
    () => true
  );
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
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const fee = useStore((state) => state.transfers.fee);
  const feeToken = useStore((state) => state.transfers.feeToken);
  const sourceGas = useMemo(() => {
    return {
      fee: fee.partialFee.multipliedBy(FEE_MULTIPLIER),
      token: feeToken,
    };
  }, [fee.partialFee, feeToken]);
  const setFormError = useStore(
    (state) => state.transfers.setFormError,
    () => true
  );

  const maxAmountToTransfer = useMemo(() => {
    return callbackGate(
      (api, _existentialDeposit) => {
        return calculateTransferAmount({
          amountToTransfer: balance,
          balance: balance,
          keepAlive,
          selectedToken,
          sourceExistentialDeposit: _existentialDeposit,
          sourceGas,
        });
      },
      fromProvider.parachainApi,
      existentialDeposit
    );
  }, [
    balance,
    existentialDeposit,
    fromProvider.parachainApi,
    keepAlive,
    selectedToken,
    sourceGas,
  ]);

  const { validate, hasError, stringValue, bignrValue, setValue } =
    useValidation({
      maxValue: maxAmountToTransfer,
      maxDec: tokens[selectedToken].decimals[from] ?? 12,
      initialValue: amount,
    });

  const handleMaxClick = () => {
    setValue(maxAmountToTransfer);
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
  }, [from, to, selectedToken, setValue]);

  useEffect(() => {
    setFormError(hasError);
  }, [hasError, setFormError]);

  // Update the amount based on user input
  useEffect(() => {
    if (!bignrValue.eq(amount)) {
      updateAmount(bignrValue);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [bignrValue]);

  // Update internal value based on external amount changes. (post transfer hooks, etc.)
  useEffect(() => {
    if (!amount.eq(bignrValue)) {
      setValue(amount);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
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
            balanceText: (
              <>
                <Typography variant="inputLabel" ml={0.5}>
                  {humanBalance(balance) + " " + tokens[tokenId].symbol}
                </Typography>
                <Tooltip
                  arrow
                  placement="top"
                  title={`${balance.toFixed()} ${tokens[tokenId].symbol}`}
                  color="primary"
                  sx={{
                    fontSize: "1rem",
                  }}
                >
                  <InfoTwoTone />
                </Tooltip>
              </>
            ),
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
