import { ComponentStory, ComponentMeta } from "@storybook/react";
import Home from "picasso/pages/index";

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
  title: "templates/Home",
  component: Home,
} as ComponentMeta<typeof Home>;

const Template: ComponentStory<typeof Home> = (args) => <Home {...args} />;

export const HomePage = Template.bind({});
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
HomePage.args = {};
