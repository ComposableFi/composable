import React, { useCallback } from 'react';
import {
  useTheme, 
} from "@mui/material";
import { DropdownCombinedInput, DropdownCombinedInputProps } from '.';
import { useValidation } from '@/hooks/bignumber';
import BigNumber from 'bignumber.js';
import { debounce } from 'lodash';

export type DropdownCombinedBigNumberInputProps = {
  maxDecimals?: number,
  maxValue: BigNumber,
  setValid?: (value: boolean) => any,
} & DropdownCombinedInputProps;

export const DropdownCombinedBigNumberInput: React.FC<DropdownCombinedBigNumberInputProps> = ({
  value,
  setValue: setter,
  setValid,
  maxDecimals,
  maxValue,
  ...rest
}) => {
  const theme = useTheme();

  const maxDec = maxDecimals ? maxDecimals : 18;
  const {
    bignrValue,
    stringValue,
    hasError,
    validate,
    setValue,
    setStringValue,
  } = useValidation({
    initialValue: value as BigNumber,
    maxDec,
    maxValue,
  });

  React.useEffect(() => {
    setValid && setValid(!hasError);
  }, [hasError, setValid]);

  React.useEffect(() => {
    setter && setter(bignrValue);
  }, [bignrValue]);

  React.useEffect(() => {
    if (value !== bignrValue) {
      setValue(value as BigNumber);
      setStringValue((value as BigNumber).toFixed());
    }
  }, [value]);

  return (
    <DropdownCombinedInput
      onChange={validate}
      value={stringValue}
      placeholder='0.00'
      {...rest}
    />
  );
  
};
