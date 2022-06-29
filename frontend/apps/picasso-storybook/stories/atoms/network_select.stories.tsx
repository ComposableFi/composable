import { NetworkSelect, NetworkSelectProps } from "picasso/components";
import { NETWORK_IDS } from "picasso/defi/Networks";
import { SUBSTRATE_NETWORK_IDS } from "picasso/defi/polkadot/Networks";
import { NetworkId } from "picasso/defi/types";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import { useState } from "react";

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
    props.substrateNetwork ? SUBSTRATE_NETWORK_IDS[0] : NETWORK_IDS[0]
  );

  return (
    <Box sx={boxStyle}>
      <NetworkSelect value={value} setValue={setValue} {...props} />
      <NetworkSelect value={NETWORK_IDS[0]} {...props} disabled />
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
  options: NETWORK_IDS.map((networkId) => ({ networkId: networkId })),
};

export const SubstrateNetworkSelect = Template.bind({});
SubstrateNetworkSelect.args = {
  searchable: true,
  substrateNetwork: true,
  options: SUBSTRATE_NETWORK_IDS.map((networkId) => ({ networkId: networkId })),
};
