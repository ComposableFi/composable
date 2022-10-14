import React from "react";
import { BigNumber } from "bignumber.js";

const FLOAT_NUMBER: RegExp = /^\d+(\.\d+)?$/;

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
  const [stringValue, setStringValue] = React.useState("");
  const [value, setValue] = React.useState<BigNumber>(initialValue);

  React.useEffect(() => {
    setStringValue(value.toFixed());
    if (!value.eq(0)) {
      setValid(true);
    }
  }, [value]);

  React.useEffect(() => {
    if (!initialValue.eq(value)) {
      setValue(initialValue);
    }
  }, [initialValue]); // eslint-disable-line react-hooks/exhaustive-deps

  const validate: (
    event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void = (
    event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const eventValue = event.target.value;

    if (!eventValue.length) {
      setStringValue(eventValue);
      setValid(false);
    }

    if (eventValue.match(FLOAT_NUMBER)) {
      const bignr = new BigNumber(eventValue);
      if ((bignr.decimalPlaces() || 0) > maxDec) {
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
      setStringValue(eventValue);
      setValid(false);
    }
  };

  return {
    hasError: !valid,
    stringValue: stringValue,
    bignrValue: value,
    validate,
    setValue,
  };
}
