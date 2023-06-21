import { Box, Button, ButtonProps, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
const supportedSizes = ["small", "medium", "large"] as const;
const supportedVariants = ["text", "outlined", "contained"] as const;

const ButtonsStories = (props: ButtonProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    width: "20rem",
    resize: "both",
    overflow: "auto",
  };

  return (
    <Box sx={boxStyle}>
      <Button variant={props.variant}>Master button</Button>
      <Button variant={props.variant} disabled>
        Master button
      </Button>
    </Box>
  );
};
export default {
  title: "atoms/Button",
  component: ButtonsStories,
  argTypes: {
    variant: {
      options: supportedVariants,
      control: {
        type: "radio",
      },
    },
  },
};

const Template: Story<typeof ButtonsStories> = (args) => (
  <ButtonsStories {...args} />
);

export const Buttons = Template.bind({});
Buttons.args = {
  variant: "contained",
};
