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
    nftId: "357",
    lockedPrincipalAsset: new BigNumber(34567),
    expiryDate: "26-09-2022",
    multiplier: "1",
    isExpired: false,
    xTokenBalance: new BigNumber(0)
  },
};
