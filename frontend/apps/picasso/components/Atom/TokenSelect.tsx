import React from "react";
import { getToken, TokenId } from "tokens";
import { Select, SelectProps } from "./Select";

type TokenOption = {
  tokenId: TokenId;
  disabled?: boolean;
};

export type TokenSelectProps = {
  options?: TokenOption[];
} & Omit<SelectProps, "options">;

export const TokenSelect: React.FC<TokenSelectProps> = ({
  options,
  ...rest
}) => {
  const selectOptions = options
    ? options.map(option => ({
        value: option.tokenId,
        icon: getToken(option.tokenId).icon,
        label: getToken(option.tokenId).symbol,
        disabled: option.disabled
      }))
    : [];

  return <Select options={selectOptions} {...rest} />;
};
