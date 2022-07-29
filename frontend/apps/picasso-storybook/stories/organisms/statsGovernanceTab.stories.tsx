import { StatsGovernanceTab } from 'picasso/components'
import { ComponentStory } from '@storybook/react'

export default {
  title: 'molecules/StatsGovernanceTab',
  component: StatsGovernanceTab,
}

const Template: ComponentStory<typeof StatsGovernanceTab> = (args) => (
  <StatsGovernanceTab {...args} />
)

export const StatsGovernanceTabStory = Template.bind({})
StatsGovernanceTabStory.args = {}
