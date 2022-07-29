import { PoolShare, PoolShareProps } from "pablo/components/Organisms/bonds/PoolShare";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import BigNumber from "bignumber.js";

const PoolShareStories = (props: PoolShareProps) => {
  return (
    <Box>
      <PoolShare {...props} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/PoolShare",
  component: PoolShare,
};

const Template: ComponentStory<typeof PoolShare> = (args) => (
  <PoolShareStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  tokenId1: 'pica',
  tokenId2: 'ksm',
  price: new BigNumber(0.1),
  revertPrice: new BigNumber(10),
  share: new BigNumber(3.3),
};
