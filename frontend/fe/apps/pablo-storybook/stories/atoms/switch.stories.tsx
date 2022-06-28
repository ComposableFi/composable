import { Switch as MuiSwitch, SwitchProps } from "@mui/material";
import { Story } from "@storybook/react";
import { SxProps, Box } from "@mui/material";

const Switch = (props: SwitchProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
  };

  return (
    <Box sx={boxStyle}>
      <MuiSwitch {...props} />
    </Box>
  );
};

export default {
  title: "atoms/Switch",
  component: MuiSwitch,
  disabled: {
    options: [true, false],
  },
  checked: {
    options: [true, false],
  },
};

const Template: Story<SwitchProps> = (args) => <Switch {...args} />;

export const SwitchTemplate = Template.bind({});
SwitchTemplate.args = {
  checked: true,
  disabled: false,
};
