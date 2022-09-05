import { MyBondsTable } from "picasso/components";
import { ComponentStory } from "@storybook/react";
import { TOKENS } from "tokens";

export default {
  title: "molecules/MyBondsTable",
  component: MyBondsTable,
};

const Template: ComponentStory<typeof MyBondsTable> = (args) => (
  <MyBondsTable {...args} />
);

export const MyBondsTableStory = Template.bind({});
MyBondsTableStory.args = {
  assets: [
    {
      token: TOKENS["ksm"],
      toToken: TOKENS["pica"],
      claimable: 543,
      pending: 123,
      vesting_time: "4D 2H 43M",
    },
    {
      token: TOKENS["pica"],
      toToken: TOKENS["ksm"],
      claimable: 543,
      pending: 123,
      vesting_time: "4D 2H 43M",
    },
  ],
};
