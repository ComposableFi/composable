import { MyStakingsTable } from "picasso/components";
import { ComponentStory, Story } from "@storybook/react";
import { TOKENS } from "tokens";

export default {
  title: "molecules/MyStakingsTable",
  component: MyStakingsTable
};

const Template: ComponentStory<typeof MyStakingsTable> = args => (
  <MyStakingsTable {...args} />
);

export const MyStakingsTableStory = Template.bind({});
MyStakingsTableStory.args = {
  assets: [
    {
      token: TOKENS["pica"],
      toToken: TOKENS["ksm"],
      price: 1.43,
      balance: 4534,
      value: 46187,
      change_24hr: 0.34
    },
    {
      token: TOKENS["ksm"],
      toToken: TOKENS["pica"],
      price: 189,
      balance: 42,
      value: 984.98,
      change_24hr: -0.12
    }
  ]
};
