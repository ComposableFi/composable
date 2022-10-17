import { TokenDropdownCombinedInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import { humanBalance } from "shared";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";
import { useTransfer } from "@/defi/polkadot/hooks";
import {
  subscribeDefaultTransferToken,
  subscribeTokenOptions,
} from "@/stores/defi/polkadot/transfers/subscribers";
import { useEffect, useState } from "react";
import { useValidation } from "@/components/Molecules/BigNumberInput/hooks";
import BigNumber from "bignumber.js";
import { Typography } from "@mui/material";

export const AmountTokenDropdown = () => {
  const updateAmount = useStore((state) => state.transfers.updateAmount);
  const amount = useStore((state) => state.transfers.amount);
  const { balance, tokenId } = useExistentialDeposit();
  const { from } = useTransfer();
  const tokenOptions = useStore((state) => state.transfers.tokenOptions);
  const selectedToken = useStore((state) => state.transfers.selectedToken);
  const updateSelectedToken = useStore(
    (state) => state.transfers.updateSelectedToken
  );
  const isTokenBalanceZero = useStore(
    (state) => state.transfers.isTokenBalanceZero
  );

  const [stringValue, setStringValue] = useState<string>(amount.toString());
  const { validate, hasError } = useValidation({
    maxValue: balance,
    maxDec: 12,
    initialValue: new BigNumber(0),
  });
  const handleMaxClick = () => {
    updateAmount(balance);
    setStringValue(balance.toString());
    validate({
      target: {
        value: balance.toString(),
      },
    } as any);
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
