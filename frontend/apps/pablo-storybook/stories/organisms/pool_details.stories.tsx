import { ComponentStory } from "@storybook/react";
import { PoolDetails } from "pablo/components/Organisms";
import { Asset } from "shared";
import BigNumber from "bignumber.js";

export default {
  title: "organisms/PoolDetails/Main",
  component: PoolDetails,
};

const Template: ComponentStory<typeof PoolDetails> = (args) => (
  <PoolDetails {...args} />
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
};
