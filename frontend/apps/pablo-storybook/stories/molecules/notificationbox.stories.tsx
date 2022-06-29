import { ComponentStory } from "@storybook/react";
import { NotificationBox } from "pablo/components";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";

export default {
  title: "molecules/NotificationBox",
  component: NotificationBox,
  argTypes: {
    type: {
      control: {
        type: "select",
        label: "Type",
        options: ["info", "success", "error", "warning"],
      },
    },
  },
};

const Template: ComponentStory<typeof NotificationBox> = (args) => (
  <NotificationBox {...args} />
);

export const NotificationBoxStory = Template.bind({});
NotificationBoxStory.args = {
  type: "warning",
  icon: <InfoOutlinedIcon color="primary" fontSize="small" />,
  mainText: "Main Text",
  subText: "Sub Text Here.",
};

export const NoSubTextStory = Template.bind({});
NoSubTextStory.args = {
  type: "warning",
  icon: <InfoOutlinedIcon color="primary" fontSize="small" />,
  mainText: "Main Text",
};

export const NoMainTextStory = Template.bind({});
NoMainTextStory.args = {
  type: "warning",
  icon: <InfoOutlinedIcon color="primary" fontSize="small" />,
  subText: "Sub Text Here.",
};

export const NoTextStory = Template.bind({});
NoTextStory.args = {
  type: "warning",
  icon: <InfoOutlinedIcon color="primary" fontSize="small" />,
};
