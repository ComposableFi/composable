import React, { useMemo } from "react";
import { BigNumber } from "bignumber.js";
import { fromChainIdUnit, toChainIdUnit } from "shared";

const FLOAT_NUMBER: RegExp = /^\d+(\.\d+)?$/;
const ALLOWED_INPUT: RegExp = /^\d*\.?\d*$/;

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
  const isDirty = useMemo(() => !initialValue.eq(value), [value, initialValue]);

  React.useEffect(() => {
    if (!value.eq(new BigNumber(stringValue))) {
      validate({
        target: {
          value: value.toString(),
        },
      } as any);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [maxDec, value]);

  React.useEffect(() => {
    if (!initialValue.eq(value)) {
      setValue(initialValue);
    }
  }, [initialValue]); // eslint-disable-line react-hooks/exhaustive-deps

  const validate: (
    event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void = (event) => {
    const eventValue = event.target.value;

    if (!eventValue.length) {
      setStringValue(eventValue);
      setValid(false);
    }
    if (eventValue.match(FLOAT_NUMBER)) {
      const bignr = fromChainIdUnit(
        toChainIdUnit(new BigNumber(eventValue), maxDec).integerValue(),
        maxDec
      );
      if ((bignr.decimalPlaces() || 0) > maxDec) {
        setValid(false);
        return;
      }

      if (bignr.eq(0)) {
        setStringValue(eventValue);
        setValue(new BigNumber(0));
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
        setValid(true);
        return;
      }

      setStringValue(eventValue);
      setValid(true);
      setValue(bignr);
    } else if (eventValue.match(ALLOWED_INPUT)) {
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
    isDirty,
  };
}
