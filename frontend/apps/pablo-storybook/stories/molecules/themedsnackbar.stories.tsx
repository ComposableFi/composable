import { ThemeResponsiveSnackbar } from "pablo/components";
import { Story } from "@storybook/react";
import { SnackbarProvider } from "notistack";

export default {
  title: "molecules/ThemeResponsiveSnackbar",
  component: ThemeResponsiveSnackbar,
  argTypes: {
    displayName: "ThemeResponsiveSnackbar",
  },
};

const Template: Story<typeof ThemeResponsiveSnackbar> = (args) => (
  <SnackbarProvider maxSnack={4}>
    <ThemeResponsiveSnackbar
      variant="success"
      isClosable={true}
      description="This is a description message"
      message="Pablo"
      persist={false}
    />
  </SnackbarProvider>
);

export const Snackbars = Template.bind({});
Snackbars.args = {};
