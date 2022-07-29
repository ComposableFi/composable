import React from "react";
import { 
  NetworkTabsProps,
  NetworkTabItem,
  NetworkTabs, 
} from "picasso/components";
import { 
  Box, 
  SxProps,
} from "@mui/material";
import { NETWORK_IDS } from "picasso/defi/Networks";
import { Story } from "@storybook/react";

const networkItems: NetworkTabItem[] = [
  {
    networkId: NETWORK_IDS[0],
  },
  {
    networkId:  NETWORK_IDS[1],
  },
  {
    networkId:  NETWORK_IDS[2],
  }
];

const NetworkTabsStories = (props: NetworkTabsProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 4,
    resize: "both",
    overflow: "auto",
  };

  const [value, setValue] = React.useState<number>(NETWORK_IDS[0]);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Box sx={boxStyle}>
      <NetworkTabs {...props} value={value} onChange={handleChange}  />
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
}
const Template: Story<typeof NetworkTabsStories> = (args) => (
  <NetworkTabsStories {...args} />
);

export const DefaultNetworkTabs = Template.bind({});
DefaultNetworkTabs.args = defaultArgs;
