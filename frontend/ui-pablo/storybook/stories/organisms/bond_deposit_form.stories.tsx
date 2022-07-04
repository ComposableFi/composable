import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { DepositForm } from "@ui-pablo/app/components/Organisms/bonds/DepositForm";
import useBondOffer from "@ui-pablo/app/defi/hooks/bonds/useBondOffer";

const DepositFormStories = () => {
  const bond = useBondOffer("0")

  return (
    <Box>
      <DepositForm
      bond={bond}
        offerId={"0"}
      />
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
