import {
  PoolShare,
  PoolShareProps,
} from "pablo/components/Organisms/bonds/PoolShare";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import BigNumber from "bignumber.js";
import { Asset } from "shared";

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
  pool: {
    kind: "dualAssetConstantPool",
    poolId: new BigNumber("0"),
    config: {
      lpToken: 106,
      owner: "0abcd",
      assetsWeights: {
        1: 0,
        4: 0,
      },
      assets: [
        new Asset("", "", "", "pica", undefined),
        new Asset("", "", "", "ksm", undefined),
      ],
      feeConfig: {
        feeRate: 0,
        ownerFeeRate: 0,
        protocolFeeRate: 0,
      },
    },
  },
  amounts: [new BigNumber(0), new BigNumber(1)],
  input: [
    new Asset("", "", "", "pica", undefined),
    new Asset("", "", "", "ksm", undefined),
  ],
};
