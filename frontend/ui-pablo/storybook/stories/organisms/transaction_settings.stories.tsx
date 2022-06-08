import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { TransactionSettings, TransactionSettingsProps } from "@ui-pablo/nextjs/components";
import { useAppDispatch } from "@ui-pablo/nextjs/hooks/store";
import { openTransactionSettingsModal } from "@ui-pablo/nextjs/stores/ui/uiSlice";


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
