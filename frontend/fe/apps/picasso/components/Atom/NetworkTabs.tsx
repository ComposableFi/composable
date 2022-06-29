import React from "react";
import {
  TabsProps as MuiTabsProps,
  Tabs as MuiTabs,
  Tab as MuiTab,
} from "@mui/material";
import { NetworkId } from "@/defi/types";
import { getNetwork } from "@/defi/Networks";
import Image from "next/image";

export type NetworkTabItem = {
  networkId: NetworkId;
  disabled?: boolean;
};

export type NetworkTabsProps = {
  items?: NetworkTabItem[];
  iconSize?: number;
  value?: number;
  onChange?: (_: React.SyntheticEvent, newValue: number) => any;
} & MuiTabsProps;

export const NetworkTabs: React.FC<NetworkTabsProps> = ({
  items,
  iconSize,
  value,
  onChange,
  ...rest
}) => {
  return (
    <MuiTabs value={value} onChange={onChange} variant="fullWidth" {...rest}>
      {items &&
        items.map((item) => (
          <MuiTab
            key={item.networkId}
            value={item.networkId}
            label={getNetwork(item.networkId).name}
            icon={
              <Image
                src={getNetwork(item.networkId).logo}
                alt={getNetwork(item.networkId).name}
                width={iconSize}
                height={iconSize}
              />
            }
            iconPosition="start"
            disabled={item.disabled}
          />
        ))}
    </MuiTabs>
  );
};

NetworkTabs.defaultProps = {
  iconSize: 24,
};
