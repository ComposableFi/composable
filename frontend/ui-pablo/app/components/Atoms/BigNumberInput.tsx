import React from 'react';
import { Input, InputProps } from '@/components/Atoms';
import BigNumber from 'bignumber.js';
import { useValidation } from '@/hooks/bignumber';

export type BigNumberInputProps = {
  maxDecimals?: number,
  maxValue: BigNumber,
  setValid?: (value: boolean) => any,
} & InputProps;

export const BigNumberInput: React.FC<BigNumberInputProps> = ({
  value,
  setValue: setter,
  setValid,
  maxDecimals,
  maxValue,
  ...rest
}) => {
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
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [bignrValue]);

  React.useEffect(() => {
    if (!(value as BigNumber).eq(bignrValue)) {
      setValue(value as BigNumber);
      setStringValue((value as BigNumber).toFixed());
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [value]);

  return (
    <Input
      onChange={validate}
      value={stringValue}
      placeholder='0.00'
      {...rest}
    />
  );

};
