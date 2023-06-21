import { BaseAsset, BaseAssetProps } from "./BaseAsset";
import { FC } from "react";

export type TokenAssetProps = {
  iconOnly?: boolean;
} & BaseAssetProps;

export const TokenAsset: FC<TokenAssetProps> = ({
  iconOnly,
  icon,
  label,
  ...rest
}) => {
  return <BaseAsset icon={icon} label={iconOnly ? "" : label} {...rest} />;
};

TokenAsset.defaultProps = {
  iconSize: 24,
};
