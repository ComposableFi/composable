import React from "react";
import { BaseAsset, BaseAssetProps } from "./BaseAsset";
import { NetworkId } from "@/defi/types";
import { getNetwork } from "@/defi/Networks";

export type NetworkAssetProps = {
  networkId: NetworkId;
} & BaseAssetProps;

export const NetworkAsset: React.FC<NetworkAssetProps> = ({
  networkId,
  icon,
  label,
  ...rest
}) => {
  const network = getNetwork(networkId);
  return (
    <BaseAsset
      icon={icon || network.logo}
      label={label || network.name}
      {...rest}
    />
  );
};

NetworkAsset.defaultProps = {
  iconSize: 24,
};
