import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { TransactionSettings, TransactionSettingsProps } from "@ui-pablo/app/components";
import { useAppDispatch } from "@ui-pablo/app/hooks/store";
import { openTransactionSettingsModal } from "@ui-pablo/app/stores/ui/uiSlice";


const TransactionSettingsStories = (props: TransactionSettingsProps) => {
  const dispatch = useAppDispatch();
  dispatch(openTransactionSettingsModal());
  return (
    <Box>
      <TransactionSettings {...props} />
    </Box>
  );
};
export default {
  title: "organisms/TransactionSettings",
  component: TransactionSettings,
};

const Template: ComponentStory<typeof TransactionSettings> = (args) => (
  <TransactionSettingsStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  applyCallback: () => {},
  closeCallback: () => {},
};
