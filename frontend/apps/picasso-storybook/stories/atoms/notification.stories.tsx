import { Story } from "@storybook/react";
import { Notification } from "picasso/components";

export default {
  title: "atoms/Notification/Inline",
  component: Notification,
};

const Template: Story<typeof Notification> = (args) => (
  <Notification {...args} />
);

export const NotificationSuccess = Template.bind({
  severity: "success",
  alertText: "Text element",
});

export const NotificationError = Template.bind({
  severity: "error",
  alertText: "Text element",
});

export const NotificationInfo = Template.bind({
  severity: "info",
  alertText: "Text element",
});

export const NotificationWarning = Template.bind({
  severity: "warning",
  alertText: "Text element",
});
