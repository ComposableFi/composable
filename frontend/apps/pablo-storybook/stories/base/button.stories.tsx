import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import { Box, Typography, Button as MuiButton } from "@mui/material";

type Size = "large" | "medium" | "small";
type Variant = "contained" | "outlined" | "text";
const ButtonGuide: React.FC = () => {
  const boxStyles = {
    display: "flex",
    gap: 3,
    alignItems: "flex-end",
    border: "1px dotted black",
    borderRadius: "5px",
    p: 2,
    m: 1,
  };

  const sizes: Size[] = ['large', 'medium', 'small'];
  const variants: Variant[] = ['contained', 'outlined', 'text'];

  return (
    <>
      <Typography variant="h3">Style guide</Typography>
      <Typography variant="h4">Buttons (Active)</Typography>
      <Box
        sx={{
          display: "grid",
          width: "80%",
          gridTemplateColumns: "repeat(3, minmax(15rem, 1fr))",
          gridGap: "1rem",
        }}
      >
        {
          variants.map((variant) => {
            return (
              sizes.map((size) => (
                <MuiButton
                  key={`${variant}-${size}`}
                  variant={variant}
                  size={size}
                >
                  {variant}-{size}
                </MuiButton>  
              ))
            );
          })
        }
      </Box>

      <Typography variant="h4">Buttons (Disabled)</Typography>
      <Box
        sx={{
          display: "grid",
          width: "80%",
          gridTemplateColumns: "repeat(3, minmax(15rem, 1fr))",
          gridGap: "1rem",
        }}
      >
        {
          variants.map((variant) => {
            return (
              sizes.map((size) => (
                <MuiButton
                  key={`${variant}-${size}`}
                  variant={variant}
                  size={size}
                  disabled
                >
                  {variant}-{size}
                </MuiButton>  
              ))
            );
          })
        }
      </Box>
    </>
  );
};

export default {
  title: "Style/Button",
  component: ButtonGuide,
} as ComponentMeta<typeof ButtonGuide>;

const Template: ComponentStory<typeof ButtonGuide> = (args) => (
  <ButtonGuide {...args} />
);

export const Button = Template.bind({});
Button.args = {};