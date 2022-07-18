import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import {
  UnstakeModal,
  UnstakeModalProps,
} from "pablo/components/Organisms/staking/UnstakeForm/UnstakeModal";

import BigNumber from "bignumber.js";

const UnstakeModalStories = (props: UnstakeModalProps) => {
  return (
    <Box>
      <UnstakeModal {...props} open={true} />
    </Box>
  );
};
export default {
  title: "organisms/staking/UnstakeModal",
  component: UnstakeModal,
};

const Template: ComponentStory<typeof UnstakeModal> = (args) => (
  <UnstakeModalStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  xPablo: {
    id: 357,
    tokenId: "pablo",
    locked: new BigNumber(34567),
    expiry: 1645345320000,
    multiplier: 1,
    amount: new BigNumber(23309),
    withdrawableAmount: new BigNumber(23309),
  },
};
