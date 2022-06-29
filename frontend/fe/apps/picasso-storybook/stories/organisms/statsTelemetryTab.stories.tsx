import { StatsTelemetryTab } from 'picasso/components'
import { ComponentStory } from '@storybook/react'

export default {
  title: 'molecules/StatsTelemetryTab',
  component: StatsTelemetryTab,
}

const Template: ComponentStory<typeof StatsTelemetryTab> = (args) => (
  <StatsTelemetryTab {...args} />
)

export const StatsTelemetryTabStory = Template.bind({})
StatsTelemetryTabStory.args = {}
