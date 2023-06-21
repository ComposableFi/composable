import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/add_liquidity_form.stories";

const { Default } = composeStories(stories);

test("renders AddLiquidityForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Add Liquidity")).toBeTruthy();
});
