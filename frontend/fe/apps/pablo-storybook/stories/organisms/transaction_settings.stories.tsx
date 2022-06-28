import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { TransactionSettings, TransactionSettingsProps } from "pablo/components";
import { useAppDispatch } from "pablo/hooks/store";
import { openTransactionSettingsModal } from "pablo/stores/ui/uiSlice";


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
