import { AlertBox, Link } from "picasso/components";
import { ComponentStory } from "@storybook/react";
import OpenInNewRounded from "@mui/icons-material/OpenInNewRounded";
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline';

export default {
  title: "molecules/AlertBox",
  component: AlertBox,
};

const Template: ComponentStory<typeof AlertBox> = (args) => (
  <AlertBox {...args}>
    Alert message here!
  </AlertBox>  
);

export const AlertBoxes = Template.bind({});
AlertBoxes.args = {
  underlined: true,
  dismissible: true,
  icon: (
    <CheckCircleOutlineIcon />
  ),
  link: (
    <Link 
      key="Crowdloan" 
      underline="none" 
      color="primary" 
      href="/Users/easteregg/Public"
      target="_blank"
    >
      <OpenInNewRounded />
    </Link>
  )
}
