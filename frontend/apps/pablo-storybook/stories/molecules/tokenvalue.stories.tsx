import { ComponentStory } from "@storybook/react";
import { TOKENS } from "tokens";
import { TokenValue } from "pablo/components/Molecules/TokenValue";
import { Asset } from "shared";

export default {
  title: "molecules/TokenValue",
  component: TokenValue,
};

const Template: ComponentStory<typeof TokenValue> = (args) => (
  <TokenValue {...args} />
);

export const Default = Template.bind({});
Default.args = {
  token: new Asset(
    TOKENS.pblo.symbol,
    TOKENS.pblo.symbol,
    TOKENS.pblo.icon,
    TOKENS.pblo.id
  ),
  value: "500.00",
};
