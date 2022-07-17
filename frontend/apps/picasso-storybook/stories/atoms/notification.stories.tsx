import { Story } from "@storybook/react";
import { Notification, NotificationProps } from "picasso/components";

const NotificationStories = (props: NotificationProps) => {
  return (
    <Notification {...props} />
  );
};

export default {
  title: "atoms/Notification/Inline",
  component: NotificationStories,
};

const Template: Story<typeof NotificationStories> = (args) => (
  <NotificationStories {...args} />
);

export const NotificationSuccess = Template.bind({});

NotificationSuccess.args = {
  severity: "success",
  alertText: "Text element",
}

export const NotificationError = Template.bind({});

NotificationError.args = {
  severity: "error",
  alertText: "Text element",
}

export const NotificationInfo = Template.bind({});

NotificationInfo.args = {
  severity: "info",
  alertText: "Text element",
}

export const NotificationWarning = Template.bind({});

NotificationWarning.args = {
  severity: "warning",
  alertText: "Text element",
}
