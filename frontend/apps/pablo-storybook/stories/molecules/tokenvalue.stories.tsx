import { ComponentStory } from "@storybook/react";
import { TOKENS } from "pablo/defi/Tokens";
import { TokenValue } from "pablo/components/Molecules/TokenValue";

export default {
  title: "molecules/TokenValue",
  component: TokenValue,
};

const Template: ComponentStory<typeof TokenValue> = (args) => (
  <TokenValue {...args} />
);

export const Default = Template.bind({});
Default.args = {
  token: TOKENS.pablo,
  value: "500.00",
};
