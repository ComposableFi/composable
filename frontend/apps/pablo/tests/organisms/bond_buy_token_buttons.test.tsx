import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bond_buy_token_buttons.stories";

const { Default } = composeStories(stories);

test("renders Bond/BuyButtons with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Create LP")).toBeTruthy();
});
