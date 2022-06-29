import { ComponentStory, ComponentMeta } from '@storybook/react'
import Stats from 'picasso/pages/stats'

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
  title: 'Next/Stats',
  component: Stats,
} as ComponentMeta<typeof Stats>

const Template: ComponentStory<typeof Stats> = (args) => <Stats {...args} />

export const StatsPage = Template.bind({})
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
StatsPage.args = {}
