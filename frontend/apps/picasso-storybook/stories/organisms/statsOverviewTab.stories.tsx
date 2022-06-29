import { StatsOverviewTab } from 'picasso/components'
import { ComponentStory } from '@storybook/react'

export default {
  title: 'molecules/StatsOverviewTab',
  component: StatsOverviewTab,
}

const Template: ComponentStory<typeof StatsOverviewTab> = (args) => (
  <StatsOverviewTab {...args} />
)

export const StatsOverviewTabStory = Template.bind({})
StatsOverviewTabStory.args = {}
