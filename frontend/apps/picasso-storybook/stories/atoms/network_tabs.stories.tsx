import {
  NetworkTabItem,
  NetworkTabs,
  NetworkTabsProps,
} from "picasso/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import config from "picasso/constants/config";
import { SyntheticEvent, useState } from "react";

const networkItems: NetworkTabItem[] = [
  {
    networkId: config.evm.networkIds[0],
  },
  {
    networkId: config.evm.networkIds[1],
  },
  {
    networkId: config.evm.networkIds[2],
  },
];

const NetworkTabsStories = (props: NetworkTabsProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 4,
    resize: "both",
    overflow: "auto",
  };

  const [value, setValue] = useState<number>(config.evm.networkIds[0]);

  const handleChange = (_: SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Box sx={boxStyle}>
      <NetworkTabs {...props} value={value} onChange={handleChange} />
    </Box>
  );
};
export default {
  title: "atoms/NetworkTabs",
  component: NetworkTabs,
};

const defaultArgs = {
  items: networkItems,
  iconSize: 24,
};
const Template: Story<typeof NetworkTabsStories> = (args) => (
  <NetworkTabsStories {...args} />
);

export const DefaultNetworkTabs = Template.bind({});
DefaultNetworkTabs.args = defaultArgs;
