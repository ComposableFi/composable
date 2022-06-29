import { PageTitle } from "picasso/components";
import { ComponentStory } from "@storybook/react";

export default {
  title: "molecules/PageTitle",
  component: PageTitle,
  argTypes: {
    title: {
      control: {
        type: "text",
        label: "Title",
      },
    },
  },
};

const Template: ComponentStory<typeof PageTitle> = (args) => (
  <PageTitle {...args} />
);

export const PageTitleStory = Template.bind({});
PageTitleStory.args = {
    title: "Overview",
    subtitle: "You will be able to check on your positions here.",
};

export const PageTitleWithAlignCenter = Template.bind({});
PageTitleWithAlignCenter.args = {
    title: "Pass another text here",
    subtitle: "You will be able to check on your positions here.",
}
