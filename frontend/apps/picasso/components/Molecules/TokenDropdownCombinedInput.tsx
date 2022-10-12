import React from "react";
import { TokenSelectProps } from "../Atom/TokenSelect";
import {
  DropdownCombinedInput,
  DropdownCombinedInputProps,
} from "./DropdownCombinedInput";
import { getToken } from "tokens";

export type TokenDropdownCombinedInputProps = {
  CombinedSelectProps?: TokenSelectProps;
} & Omit<DropdownCombinedInputProps, "CombinedSelectProps">;

export const TokenDropdownCombinedInput: React.FC<
  TokenDropdownCombinedInputProps
> = ({ CombinedSelectProps, children, ...rest }) => {
  const { options: tokenOptions, ...restSelectProps } =
    CombinedSelectProps || {};
  console.log(tokenOptions);
  const options = tokenOptions
    ? tokenOptions.map(({ tokenId, disabled }) => ({
        value: tokenId,
        icon: getToken(tokenId).icon,
        label: getToken(tokenId).symbol,
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
