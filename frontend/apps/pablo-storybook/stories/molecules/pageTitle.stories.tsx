import { ComponentStory } from "@storybook/react";
import { PageTitle } from "pablo/components";

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
