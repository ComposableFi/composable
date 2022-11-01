import React from "react";
import { BaseAsset, BaseAssetProps } from "./BaseAsset";

export type TokenAssetProps = {
  iconOnly?: boolean;
} & BaseAssetProps;

export const TokenAsset: React.FC<TokenAssetProps> = ({
  iconOnly,
  icon,
  label,
  ...rest
}) => {
  return (
    <BaseAsset
      icon={icon}
      label={iconOnly ? "" : label}
      {...rest}
    />
  );
};

TokenAsset.defaultProps = {
  iconSize: 24,
};
