import { MyAssetsTable } from "picasso/components";
import { ComponentStory, Story } from "@storybook/react";
import { TOKENS } from "tokens";

export default {
  title: "molecules/MyAssetsTable",
  component: MyAssetsTable
};

const Template: ComponentStory<typeof MyAssetsTable> = args => (
  <MyAssetsTable {...args} />
);

export const MyAssetsTableStory = Template.bind({});
MyAssetsTableStory.args = {
  assets: [
    {
      tokenId: "pica",
      price: 1.43,
      balance: "4534",
      value: 46187,
      icon: TOKENS["pica"].icon,
      change_24hr: 0.34,
      symbol: "PICA",
      decimalsToDisplay: 4
    },
    {
      tokenId: "ksm",
      price: 189,
      balance: "42",
      icon: TOKENS["ksm"].icon,
      value: 46187,
      change_24hr: -0.12,
      symbol: "KSM",
      decimalsToDisplay: 4
    }
  ]
};
