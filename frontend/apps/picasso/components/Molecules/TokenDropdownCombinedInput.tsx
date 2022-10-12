import React from "react";
import { TokenSelectProps } from "../Atom/TokenSelect";
import {
  DropdownCombinedInput,
  DropdownCombinedInputProps,
} from "./DropdownCombinedInput";

export type TokenDropdownCombinedInputProps = {
  CombinedSelectProps?: TokenSelectProps;
} & Omit<DropdownCombinedInputProps, "CombinedSelectProps">;

export const TokenDropdownCombinedInput: React.FC<
  TokenDropdownCombinedInputProps
> = ({ CombinedSelectProps, children, ...rest }) => {
  const { options: tokenOptions, ...restSelectProps } =
    CombinedSelectProps || {};
  const options = tokenOptions
    ? tokenOptions.map(({ tokenId, disabled, icon }) => ({
        value: tokenId,
        icon: icon,
        label: tokenId,
        disabled: disabled,
      }))
    : [];

  return (
    <DropdownCombinedInput
      CombinedSelectProps={{
        options: options,
        ...restSelectProps,
      }}
      {...rest}
    >
      {children}
    </DropdownCombinedInput>
  );
};
