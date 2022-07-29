import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import { Box } from "@mui/material";
import { brandPalette } from "picasso/styles/theme";

const ColorGuide: React.FC = () => {
  const boxStyles = {
    display: "flex",
    gap: 3,
    alignItems: "flex-end",
    border: '1px dotted black',
    borderRadius: '5px',
    p: 2,
    m: 1
  };

  return (
    <>
      <h1>Styleguide</h1>
      <h2>Colors</h2>
      <Box
        sx={{
          display: "grid",
          width: "80%",
          gridTemplateColumns: "repeat(3, minmax(15rem, 1fr))",
          gridGap: "1rem",
        }}
      >
        {Object.entries(brandPalette).map(([name, value]) => {
          return Object.entries(value).map(([key, color]: [string, string]) => (
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
              {name} - {key}
            </Box>
          ));
        })}
      </Box>
    </>
  );
};

/*
 * As per Component story format, default export contains meta information
 * about the component.
 * @see https://storybook.js.org/docs/react/writing-stories/components#component-story
 */
export default {
  title: "Style/Colors",
  component: ColorGuide,
} as ComponentMeta<typeof ColorGuide>;

const Template: ComponentStory<typeof ColorGuide> = (args) => (
  <ColorGuide {...args} />
);

export const Colors = Template.bind({});
// More on composing args: https://storybook.js.org/docs/react/writing-stories/args#args-composition
Colors.args = {};
