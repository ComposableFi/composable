import React, { useMemo } from "react";
import { NetworkId } from "@/defi/types";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { getNetwork } from "@/defi/Networks";
import { getSubstrateNetwork } from "@/defi/polkadot/Networks";
import { Select, SelectProps } from "./Select";

type NetworkOption = {
  networkId: NetworkId | SubstrateNetworkId;
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
