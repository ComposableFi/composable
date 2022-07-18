import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import { Box, Typography } from "@mui/material";
import { paletteOptions } from "@/styles/theme";


const ColorGuide: React.FC = () => {

  return (
    <>
      <Typography variant="h3">Styleguide</Typography>
      <Typography variant="h4">Colors</Typography>
      <Box
        sx={{
          display: "grid",
          width: "80%",
          gridTemplateColumns: "repeat(3, minmax(15rem, 1fr))",
          gridGap: "1rem",
        }}
      >
        {Object.entries(paletteOptions.dark).map(([name, value]) => {
          return (
            Object.entries(value).map(([key, color]: [string, string]) => (
              <Box
                key={`${name}-${key}`}
                sx={{
                  borderRadius: "1rem",
                  display: "flex",
                  height: "100px",
                  alignItems: "center",
                  justifyContent: "center",
                  padding: "1rem",
                  background: color,
                  color: "white",
                }}
              >
                {name} - {key}({color})
              </Box>
            )    
          ));
        })}
      </Box>
    </>
  );
};

export default {
  title: "Style/Colors",
  component: ColorGuide,
} as ComponentMeta<typeof ColorGuide>;

const Template: ComponentStory<typeof ColorGuide> = (args) => (
  <ColorGuide {...args} />
);

export const Colors = Template.bind({});
Colors.args = {};