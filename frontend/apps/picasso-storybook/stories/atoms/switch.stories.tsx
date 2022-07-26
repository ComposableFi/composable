import { Story } from "@storybook/react";
import { Switch } from "@mui/material";

export default {
  title: "atoms/Switch",
  component: Switch,
  disabled: {
    options: [true, false],
  },
  checked: {
    options: [true, false],
  },
};

const Template: Story<typeof Switch> = (args) => <Switch {...args} />;

export const DefaultSwitch = Template.bind({});
DefaultSwitch.args = {
  disabled: false,
  checked: true,
};
