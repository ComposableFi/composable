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
    nftId: "357",
    lockedPrincipalAsset: new BigNumber(34567),
    expiryDate: "26-09-2022",
    multiplier: "1",
    isExpired: false,
    xTokenBalance: new BigNumber(0)
  },
};
