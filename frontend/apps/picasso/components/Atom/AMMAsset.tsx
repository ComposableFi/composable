import React from "react";
import { BaseAsset, BaseAssetProps } from "./BaseAsset";
import { AMM_ID } from "@/defi/types";
import { getAMM } from "@/defi/AMMs";

export type AMMAssetProps = {
  id: AMM_ID;
} & BaseAssetProps;

export const AMMAsset: React.FC<AMMAssetProps> = ({ id, ...rest }) => {
  const { icon, label } = getAMM(id);

  return <BaseAsset icon={icon} label={label} {...rest} />;
};
