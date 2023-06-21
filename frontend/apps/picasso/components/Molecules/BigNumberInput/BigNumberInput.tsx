import React, { FC, useEffect } from "react";
import BigNumber from "bignumber.js";
import { Input, InputProps } from "../../Atom";
import { Typography, useTheme } from "@mui/material";
import { useValidation } from "./hooks";

type BigNumberInputProps = InputProps & {
  value: BigNumber;
  isValid?: (value: boolean) => any;
  setter: (value: BigNumber) => any;
  maxDecimals?: number;
  maxValue: BigNumber;
  onChange?: (value: BigNumber) => any;
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

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    validate(event);

    isValid?.(!hasError);
  };

  useEffect(() => {
    if (hasError) {
      const newValue = stringValue.length
        ? new BigNumber(stringValue)
        : new BigNumber(0);
      !newValue.isNaN() && setter(newValue);
    } else {
      setter(bignrValue);
    }
  }, [stringValue, hasError, setter, bignrValue]);

  return (
    <>
      <Input
        {...restInputProps}
        variant="outlined"
        value={stringValue}
        setValue={setValue}
        onChange={handleChange}
        disabled={disabled}
      />
      {!disabled && hasError && (
        <Typography
          sx={{ color: theme.palette.error.main, mt: 2 }}
          variant="subtitle1"
        >
          Please insert a correct amount
        </Typography>
      )}
    </>
  );
};
