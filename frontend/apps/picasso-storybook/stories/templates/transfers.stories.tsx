import { ComponentStory, ComponentMeta } from "@storybook/react";
import Transfers from "picasso/pages/transfers";

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
  title: "Next/Transfers",
  component: Transfers,
} as ComponentMeta<typeof Transfers>;

const Template: ComponentStory<typeof Transfers> = (args) => (
  <Transfers {...args} />
);

export const TransfersPage = Template.bind({});
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
TransfersPage.args = {};
