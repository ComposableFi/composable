import React, { useMemo } from "react";
import { Select, SelectProps } from "./Select";
import { getSubstrateNetwork, SubstrateNetworkId } from "shared";
import { NetworkId } from "@/constants/types";
import { getNetwork } from "@/constants/config";

type NetworkOption = {
  networkId: SubstrateNetworkId | NetworkId;
  disabled?: boolean;
};

export type NetworkSelectProps = {
  options?: NetworkOption[];
  substrateNetwork?: boolean;
} & Omit<SelectProps, "options">;

const createNetworkSelectOptions = (
  options?: NetworkOption[],
  substrateNetwork: boolean = false
) => {
  return options
    ? options.map((option) => {
        const network = substrateNetwork
          ? getSubstrateNetwork(option.networkId as SubstrateNetworkId)
          : getNetwork(option.networkId as NetworkId);

        return {
          value: option.networkId,
          icon: network.logo,
          label: network.name,
          disabled: option.disabled,
        };
      })
    : [];
};

export const NetworkSelect: React.FC<NetworkSelectProps> = ({
  options,
  substrateNetwork = false,
  ...rest
}) => {
  const networkSelectOptions = useMemo(
    () => createNetworkSelectOptions(options, substrateNetwork),
    [options, substrateNetwork]
  );

  return <Select options={networkSelectOptions} {...rest} />;
};
