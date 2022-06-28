import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { DepositForm } from "../../../../apps/pablo/components/Organisms/bonds/DepositForm";
import { useAppSelector } from "@/hooks/store";

const DepositFormStories = () => {
  const bond = useAppSelector((state) => state.bonds.selectedBond);
  return (
    <Box>
      <DepositForm bond={bond} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/DepositForm",
  component: DepositForm,
};

const Template: ComponentStory<typeof DepositForm> = (args) => (
  <DepositFormStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};
