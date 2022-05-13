import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { StakeUnstakeTabPanelProps, StakeUnstakeTabPanel } from "@ui-template/nextjs/components";

const StakeUnstakeTabPanelStories = (props: StakeUnstakeTabPanelProps) => {
  return (
    <Box>
      <StakeUnstakeTabPanel {...props} />
    </Box>
  );
};
export default {
  title: "organisms/StakeUnstakeTabPanel",
  component: StakeUnstakeTabPanel,
};

const Template: ComponentStory<typeof StakeUnstakeTabPanel> = (args) => (
  <StakeUnstakeTabPanelStories {...args} />
);

export const StakeTabPanel = Template.bind({});
StakeTabPanel.args = {
  activeTab: "staking",
};

export const UnstakeTabPanel = Template.bind({});
UnstakeTabPanel.args = {
  activeTab: "unstaking",
};
