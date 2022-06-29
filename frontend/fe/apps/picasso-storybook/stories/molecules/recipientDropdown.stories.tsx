import { ComponentMeta, ComponentStory } from "@storybook/react";

import { RecipientDropdown } from "picasso/components";

export default {
  title: "molecules/RecipientDropdown",
  component: RecipientDropdown,
  expanded: {
    options: [true, false],
  },
} as ComponentMeta<typeof RecipientDropdown>;

const Template: ComponentStory<typeof RecipientDropdown> = (args) => (
  <RecipientDropdown {...args} />
);

export const DefaultRecipientDropdown = Template.bind({});
DefaultRecipientDropdown.args = {
  expanded: false,
  value: "select2",
  options: [
    {
      value: "select1",
      label: "Select 1",
      icon: "/tokens/eth-mainnet.svg",
    },
    {
      value: "select2",
      label: "Select 2",
      icon: "/tokens/eth-mainnet.svg",
    },
    {
      value: "select3",
      label: "Select 3",
      icon: "/tokens/eth-mainnet.svg",
    },
  ],
};
