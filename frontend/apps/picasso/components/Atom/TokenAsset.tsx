import React from "react";
import { getToken, TokenId } from "tokens";
import { BaseAsset, BaseAssetProps } from "./BaseAsset";

export type TokenAssetProps = {
  tokenId: TokenId;
  iconOnly?: boolean;
} & BaseAssetProps;

export const TokenAsset: React.FC<TokenAssetProps> = ({
  tokenId,
  iconOnly,
  icon,
  label,
  ...rest
}) => {
  const token = getToken(tokenId);
  return (
    <BaseAsset
      icon={icon || token.icon}
      label={iconOnly ? "" : label || token.symbol}
      {...rest}
    />
  );
};

TokenAsset.defaultProps = {
  iconSize: 24,
};
