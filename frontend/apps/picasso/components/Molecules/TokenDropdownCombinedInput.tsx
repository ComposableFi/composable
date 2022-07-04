import React from "react";
import {
  Button,
  ButtonProps,
  InputAdornment,
  TextField,
  TextFieldProps,
  useTheme,
  Typography
} from "@mui/material";
import { TokenSelect } from "../Atom/TokenSelect";
import { TokenSelectProps } from "../Atom/TokenSelect";
import { SelectProps } from "../Atom/Select";
import {
  DropdownCombinedInputProps,
  DropdownCombinedInput
} from "./DropdownCombinedInput";
import { getToken } from "tokens";

export type TokenDropdownCombinedInputProps = {
  CombinedSelectProps?: TokenSelectProps;
} & Omit<DropdownCombinedInputProps, "CombinedSelectProps">;

export const TokenDropdownCombinedInput: React.FC<TokenDropdownCombinedInputProps> = ({
  CombinedSelectProps,
  children,
  ...rest
}) => {
  const { options: tokenOptions, ...restSelectProps } =
    CombinedSelectProps || {};
  const options = tokenOptions
    ? tokenOptions.map(({ tokenId, disabled }) => ({
        value: tokenId,
        icon: getToken(tokenId).icon,
        label: getToken(tokenId).symbol,
        disabled: disabled
      }))
    : [];

  return (
    <DropdownCombinedInput
      CombinedSelectProps={{
        options: options,
        ...restSelectProps
      }}
      {...rest}
    >
      {children}
    </DropdownCombinedInput>
  );
};
