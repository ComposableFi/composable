import React from 'react';
import { ComponentStory, ComponentMeta } from '@storybook/react';
import Home from 'pablo/pages';

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
    title: 'Pages/Homepage',
    component: Home,
} as ComponentMeta<typeof Home>;

const Template: ComponentStory<typeof Home> = (args) => <Home {...args} />;

export const Homepage = Template.bind({});
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
Homepage.args = {}
