import React from "react";
import { getToken } from "tokens";
import { Select, SelectProps } from "./Select";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";

export type TokenSelectProps = {
  options?: TokenOption[];
} & Omit<SelectProps, "options">;

export const TokenSelect: React.FC<TokenSelectProps> = ({
  options,
  ...rest
}) => {
  const selectOptions = options
    ? options.map((option) => ({
        value: option.tokenId,
        icon: getToken(option.tokenId).icon,
        label: getToken(option.tokenId).symbol,
        disabled: option.disabled,
      }))
    : [];

  return <Select options={selectOptions} {...rest} />;
};
