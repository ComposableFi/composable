import { MyBondingsTable } from "picasso/components";
import { ComponentStory, Story } from "@storybook/react";
import { TOKENS } from "tokens";

export default {
  title: "molecules/MyBondingsTable",
  component: MyBondingsTable
};

const Template: ComponentStory<typeof MyBondingsTable> = args => (
  <MyBondingsTable {...args} />
);

export const MyBondingsTableStory = Template.bind({});
MyBondingsTableStory.args = {
  assets: [
    {
      token: TOKENS["ksm"],
      toToken: TOKENS["pica"],
      claimable: 543,
      pending: 123,
      vesting_time: "4D 2H 43M"
    },
    {
      token: TOKENS["pica"],
      toToken: TOKENS["ksm"],
      claimable: 543,
      pending: 123,
      vesting_time: "4D 2H 43M"
    }
  ]
};
