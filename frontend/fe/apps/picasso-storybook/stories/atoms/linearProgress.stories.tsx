import {
  LinearProgress as MuiLinearProgress,
  LinearProgressProps as MuiLinearProgressProps,
} from "@mui/material";
import { Story } from "@storybook/react";
import { SxProps, Box } from "@mui/material";

const LinearProgressStories = (props: MuiLinearProgressProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
  };

  return (
    <Box sx={boxStyle}>
      <MuiLinearProgress {...props} />
    </Box>
  );
};

export default {
  title: "atoms/LinearProgress",
  component: LinearProgressStories,
  progress: "number",
};

const Template: Story<typeof LinearProgressStories> = (args) => (
  <LinearProgressStories {...args} />
);

export const LinearProgress = Template.bind({});
LinearProgress.args = {
  progress: 40,
};
