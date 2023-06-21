import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import {
  RenewModal,
  RenewModalProps,
} from "pablo/components/Organisms/staking/UnstakeForm/RenewModal";

import BigNumber from "bignumber.js";

const RenewModalStories = (props: RenewModalProps) => {
  return (
    <Box>
      <RenewModal {...props} open={true} />
    </Box>
  );
};
export default {
  title: "organisms/staking/RenewModal",
  component: RenewModal,
};

const Template: ComponentStory<typeof RenewModal> = (args) => (
  <RenewModalStories {...args} />
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
