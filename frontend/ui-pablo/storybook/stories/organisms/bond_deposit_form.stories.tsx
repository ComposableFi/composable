import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { DepositForm } from "@ui-pablo/app/components/Organisms/bonds/DepositForm";
import { useDepositSummary } from "../../../app/store/hooks/bond/useDepositSummary";
import { useSupplySummary } from "../../../app/store/hooks/bond/useSupplySummary";

const DepositFormStories = () => {
  const supplySummary = useSupplySummary({ offerId: 1 });
  const depositSummary = useDepositSummary({ offerId: 1 });

  if (supplySummary === "no-summary" || depositSummary === "no-summary")
    return null;

  return (
    <Box>
      <DepositForm
        depositSummary={depositSummary}
        supplySummary={supplySummary}
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
