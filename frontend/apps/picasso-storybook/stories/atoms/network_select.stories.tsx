import { NetworkSelect, NetworkSelectProps } from "picasso/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import { useState } from "react";
import { getSubstrateNetwork, SubstrateNetworkId } from "shared";
import config, { getNetwork } from "picasso/constants/config";

const NetworkSelectsStories = (props: NetworkSelectProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
    padding: 2,
  };

  const [value, setValue] = useState(
    props.substrateNetwork
      ? getSubstrateNetwork("picasso" as SubstrateNetworkId)
      : getNetwork(1)
  );

  return (
    <Box sx={boxStyle}>
      <NetworkSelect value={value} setValue={setValue} {...props} />
      <NetworkSelect value={config.evm.networkIds} {...props} disabled />
    </Box>
  );
};
export default {
  title: "atoms/NetworkSelect",
  component: NetworkSelect,
};

const Template: Story<typeof NetworkSelectsStories> = (args) => (
  <NetworkSelectsStories {...args} />
);

export const NetworkSelects = Template.bind({});
NetworkSelects.args = {
  searchable: true,
  options: config.evm.networkIds.map((networkId) => ({ networkId: networkId })),
};

export const SubstrateNetworkSelect = Template.bind({});
SubstrateNetworkSelect.args = {
  searchable: true,
  substrateNetwork: true,
  options: [],
};
