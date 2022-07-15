import { BuyButtons } from "@ui-pablo/app/components/Organisms";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import useBondOffer from "@ui-pablo/app/defi/hooks/bonds/useBondOffer";

const BuyButtonsStories = () => {
  const bond = useBondOffer("0");
  return (
    <Box>
      <BuyButtons bond={bond} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/BuyButtons",
  component: BuyButtons,
};

const Template: ComponentStory<typeof BuyButtons> = (args) => (
  <BuyButtonsStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};
