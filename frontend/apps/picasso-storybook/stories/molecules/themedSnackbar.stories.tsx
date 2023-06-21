import { Story } from "@storybook/react";
import { SnackbarProvider, useSnackbar, VariantType } from "notistack";
import { Button } from "@mui/material";

import { ThemeResponsiveSnackbar } from "picasso/components";

const ThemeResponsiveSnackbarProvider = ({
  children,
}: {
  children: React.ReactChild;
}) => (
  <SnackbarProvider
    maxSnack={4}
    Components={{
      info: ThemeResponsiveSnackbar,
      success: ThemeResponsiveSnackbar,
      error: ThemeResponsiveSnackbar,
      warning: ThemeResponsiveSnackbar,
    }}
    disableWindowBlurListener={true}
    anchorOrigin={{
      vertical: "bottom",
      horizontal: "center",
    }}
    autoHideDuration={null}
  >
    {children}
  </SnackbarProvider>
);

interface SnackbarDisplayProps {
  variant: VariantType;
}

const ThemeResponsiveSnackbarDisplay = ({ variant }: SnackbarDisplayProps) => {
  const { enqueueSnackbar } = useSnackbar();

  const handleButtonClick = () => {
    enqueueSnackbar("Title", {
      description: "This is the message field",
      variant,
      isClosable: true,
      url: "https://coinmarketcap.com",
    });
  };

  return (
    <Button variant="contained" onClick={handleButtonClick}>
      Open snackbar
    </Button>
  );
};

export default {
  title: "molecules/ThemeResponsiveSnackbar",
  component: ThemeResponsiveSnackbarProvider,
  argTypes: {
    displayName: "ThemeResponsiveSnackbar",
  },
};

const Template: Story<SnackbarDisplayProps> = (args) => (
  <ThemeResponsiveSnackbarProvider>
    <ThemeResponsiveSnackbarDisplay {...args} />
  </ThemeResponsiveSnackbarProvider>
);

export const Success = Template.bind({});
Success.args = {
  variant: "success",
};

export const Error = Template.bind({});
Error.args = {
  variant: "error",
};

export const Warning = Template.bind({});
Warning.args = {
  variant: "warning",
};

export const Info = Template.bind({});
Info.args = {
  variant: "info",
};
