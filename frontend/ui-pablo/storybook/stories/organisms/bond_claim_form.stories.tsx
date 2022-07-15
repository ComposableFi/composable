import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { ClaimForm } from "@ui-pablo/app/components/Organisms/bonds/ClaimForm";
import useBondOffer from "@ui-pablo/app/defi/hooks/bonds/useBondOffer";

const ClaimFormStories = () => {
  const bond = useBondOffer("0");
  return (
    <Box>
      <ClaimForm bond={bond} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/ClaimForm",
  component: ClaimForm,
};

const Template: ComponentStory<typeof ClaimForm> = (args) => (
  <ClaimFormStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};
