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
    id: 357,
    tokenId: "pablo",
    locked: new BigNumber(34567),
    expiry: 1645345320000,
    multiplier: 1,
    amount: new BigNumber(23309),
    withdrawableAmount: new BigNumber(23309),
  },
};
