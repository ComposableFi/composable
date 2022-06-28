import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { WalletBreakdownBox } from "pablo/components/Organisms/overview/WalletBreakdownBox";

const WalletBreakdownBoxStories = ({}) => {
  return (
    <Box>
      <WalletBreakdownBox />
    </Box>
  );
};
export default {
  title: "organisms/overview/WalletBreakdownBox",
  component: WalletBreakdownBox,
};

const Template: ComponentStory<typeof WalletBreakdownBox> = (args) => (
  <WalletBreakdownBoxStories />
);

export const Default = Template.bind({});
Default.args = {};
