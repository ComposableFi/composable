import { CircularProgressProps as MuiCircularProgressProps } from "@mui/material";
import { Story } from "@storybook/react";
import { SxProps, Box } from "@mui/material";
import { CircularProgress } from "pablo/components";

const CircularProgressStories = (props: MuiCircularProgressProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
  };

  return (
    <Box sx={boxStyle}>
      <CircularProgress {...props} />
    </Box>
  );
};

export default {
  title: "atoms/CircularProgress",
  component: CircularProgressStories,
};

const Template: Story<typeof CircularProgressStories> = (args) => (
  <CircularProgressStories {...args} />
);

export const CircularProgressStory = Template.bind({});
