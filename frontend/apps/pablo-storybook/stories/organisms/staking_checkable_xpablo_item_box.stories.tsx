import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import {
  CheckableXPabloItemBox,
  CheckableXPabloItemBoxProps,
} from "pablo/components/Organisms/staking/UnstakeForm/CheckableXPabloItemBox";

import BigNumber from "bignumber.js";

const CheckableXPabloItemBoxStories = (props: CheckableXPabloItemBoxProps) => {
  return (
    <Box>
      <CheckableXPabloItemBox {...props} />
    </Box>
  );
};
export default {
  title: "organisms/staking/CheckableXPabloItemBox",
  component: CheckableXPabloItemBox,
};

const Template: ComponentStory<typeof CheckableXPabloItemBox> = (args) => (
  <CheckableXPabloItemBoxStories {...args} />
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
