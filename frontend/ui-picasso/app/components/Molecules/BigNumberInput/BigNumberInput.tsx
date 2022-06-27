import React, { FC } from "react";
import BigNumber from "bignumber.js";
import { Input } from "../../Atom";
import { Typography, useTheme } from "@mui/material";
import { InputProps } from "../../Atom";
import { useValidation } from "./hooks";

type BigNumberInputProps = InputProps & {
  value: BigNumber;
  isValid: (value: boolean) => any;
  setter: (value: BigNumber) => any;
  maxDecimals?: number;
  maxValue: BigNumber;
};

export const BigNumberInput: FC<BigNumberInputProps> = ({
  value,
  isValid,
  setter,
  maxDecimals,
  maxValue,
  disabled = false,
  ...restInputProps
}) => {
  const theme = useTheme();
  const maxDec = maxDecimals ? maxDecimals : 18;
  const { bignrValue, stringValue, hasError, validate, setValue } =
    useValidation({
      initialValue: value,
      maxDec,
      maxValue,
    });

  React.useEffect(() => {
    isValid?.(!hasError);
  }, [hasError, isValid]);

  React.useEffect(() => {
    setter && setter(bignrValue);
  }, [bignrValue]);

  React.useEffect(() => {
    if (value !== bignrValue) {
      setValue(value);
    }
  }, [value]);

  return (
    <>
      <Input
        {...restInputProps}
        variant="outlined"
        value={stringValue}
        setValue={setValue}
        onChange={validate}
        disabled={disabled}
      />
      {!disabled && hasError && (
        <Typography
          sx={{ color: theme.palette.error.main, mt: 2 }}
          variant="h6"
        >
          Please insert a correct amount
        </Typography>
      )}
    </>
  );
};
