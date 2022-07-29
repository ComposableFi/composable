import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { FormTitleProps, FormTitle } from "pablo/components";

const FormTitleStories = (props: FormTitleProps) => {
  return (
    <Box>
      <FormTitle {...props} />
    </Box>
  );
};
export default {
  title: "organisms/FormTitle",
  component: FormTitle,
};

const Template: ComponentStory<typeof FormTitle> = (args) => (
  <FormTitleStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  title: "Form Title",
  onSettingHandler: () => {},
  onBackHandler: () => {},
};
