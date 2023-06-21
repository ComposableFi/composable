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
  tokensToList: [
    "pica"
  ]
};
