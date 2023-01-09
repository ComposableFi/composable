import React from "react";
import {
  Tab as MuiTab,
  Tabs as MuiTabs,
  TabsProps as MuiTabsProps,
} from "@mui/material";
import Image from "next/image";
import { NetworkId } from "@/constants/types";
import { getNetwork } from "@/constants/config";

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
