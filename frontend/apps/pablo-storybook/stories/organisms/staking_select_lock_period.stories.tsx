import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import {
  SelectLockPeriod,
  SelectLockPeriodProps,
} from "pablo/components/Organisms/staking/StakeForm/SelectLockPeriod";

const SelectLockPeriodStories = (props: SelectLockPeriodProps) => {
  return (
    <Box>
      <SelectLockPeriod {...props} />
    </Box>
  );
};
export default {
  title: "organisms/staking/SelectLockPeriod",
  component: SelectLockPeriod,
};

const Template: ComponentStory<typeof SelectLockPeriod> = (args) => (
  <SelectLockPeriodStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  periodItems: [],
  multiplier: 0,
};
