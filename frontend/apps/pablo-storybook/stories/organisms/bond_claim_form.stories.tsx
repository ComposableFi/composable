import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { ClaimForm } from "pablo/components/Organisms/bonds/ClaimForm";
import { useAppSelector } from "pablo/hooks/store";

const ClaimFormStories = () => {
  const bond = useAppSelector((state) => state.bonds.selectedBond);
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
