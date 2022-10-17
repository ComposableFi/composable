import React from "react";
import { BigNumber } from "bignumber.js";
import { FLOAT_NUMBER, NUMBERS_ONE_DOT } from "shared";

type UseValidationType = {
  initialValue: BigNumber;
  maxDec: number;
  maxValue: BigNumber;
};

export function useValidation({
  maxValue,
  initialValue,
  maxDec,
}: UseValidationType) {
  const [valid, setValid] = React.useState<boolean>(true);
  const [stringValue, setStringValue] = React.useState(
    initialValue.eq(0) ? "" : initialValue.toFixed()
  );
  const [value, setValue] = React.useState<BigNumber>(initialValue);

  React.useEffect(() => {
    if (!value?.eq(0)) {
      setValid(true);
    } else {
      setValid(false);
    }
  }, [value]);

  const validate = (event: React.ChangeEvent<HTMLInputElement>) => {
    const eventValue = event.target.value;
    if (!eventValue.length) {
      setStringValue(eventValue);
      setValue(new BigNumber(0));
      setValid(false);
    }

    if (eventValue.match(FLOAT_NUMBER)) {
      const bignr = new BigNumber(eventValue);
      if ((bignr.decimalPlaces() || 0  ) > maxDec) {
        setValid(false);
        return;
      }

      const [, decimal] = eventValue.split(".");
      if (decimal && decimal.length > maxDec) {
        setValid(false);
        return;
      }

      if (bignr.eq(0)) {
        setStringValue(eventValue);
        setValue(bignr);
        setValid(false);
        return;
      }

      if (bignr.eq(value)) {
        setStringValue(eventValue);
        setValid(true);
        setValue(bignr);
        return;
      }

      if (maxValue && bignr.gt(maxValue)) {
        // or maybe change this to invalid value
        setStringValue(maxValue.toFixed());
        setValue(maxValue);
        setStringValue(maxValue.toFixed());
        setValid(false);
        return;
      }

      setStringValue(eventValue);
      setValid(true);
      setValue(bignr);
    } else {
      if (eventValue.match(NUMBERS_ONE_DOT)) {
        setStringValue(eventValue);
        setValid(false);
      }
    }
  };

  return {
    hasError: !valid,
    stringValue: stringValue,
    bignrValue: value,
    validate,
    setValue,
    setStringValue,
  };
}
