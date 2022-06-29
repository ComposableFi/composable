import { ComponentStory, ComponentMeta } from "@storybook/react";
import Governance from "picasso/pages/governance";

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
  title: "Next/Governance",
  component: Governance,
} as ComponentMeta<typeof Governance>;

const Template: ComponentStory<typeof Governance> = (args) => (
  <Governance {...args} />
);

export const GovernancePage = Template.bind({});
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
GovernancePage.args = {};
