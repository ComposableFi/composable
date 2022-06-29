import { Story } from "@storybook/react";
import { IconButton } from "@mui/material";
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";
import { Alert, AlertProps } from "pablo/components";

const AlertStories = (props: AlertProps) => {
  return (
    <Alert {...props} />
  );
};

export default {
  title: "atoms/Alert",
  component: AlertStories,
};

const Template: Story<typeof AlertStories> = (args) => (
  <AlertStories {...args} />
);

export const AlertSuccess = Template.bind({});

AlertSuccess.args = {
  severity: "success",
  alertText: "Text element",
  alertTitle: "",
  onClose: () => {},
  underlined: true,
  action: (
    <IconButton>
      <OpenInNewRoundedIcon />
    </IconButton>
  )
}

export const AlertError = Template.bind({});

AlertError.args = {
  severity: "error",
  alertText: "Text element",
  onClose: () => {},
  underlined: true,
  action: (
    <IconButton>
      <OpenInNewRoundedIcon />
    </IconButton>
  )
}

export const AlertInfo = Template.bind({});

AlertInfo.args = {
  severity: "info",
  alertText: "Text element",
  onClose: () => {},
  underlined: true,
  action: (
    <IconButton>
      <OpenInNewRoundedIcon />
    </IconButton>
  )
}

export const AlertWarning = Template.bind({});

AlertWarning.args = {
  severity: "warning",
  alertText: "Text element",
  onClose: () => {},
  underlined: true,
  action: (
    <IconButton>
      <OpenInNewRoundedIcon />
    </IconButton>
  )
}
