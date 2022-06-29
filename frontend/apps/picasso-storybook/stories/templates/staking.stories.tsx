import { ComponentStory, ComponentMeta } from "@storybook/react";
import Staking from "picasso/pages/staking";

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
  title: "Next/Staking",
  component: Staking,
} as ComponentMeta<typeof Staking>;

const Template: ComponentStory<typeof Staking> = (args) => (
  <Staking {...args} />
);

export const StakingPage = Template.bind({});
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
StakingPage.args = {};
