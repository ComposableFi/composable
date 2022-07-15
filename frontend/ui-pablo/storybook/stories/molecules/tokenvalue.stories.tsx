import { ComponentStory } from "@storybook/react";
import { TOKENS } from "@/defi/Tokens";
import { TokenValue } from "@ui-pablo/app/components/Molecules/TokenValue";

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
