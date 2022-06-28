import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/overview_liquidity_providers_box.stories";

const { Default } = composeStories(stories);

test("renders Overview/LiquidityProvidersBox with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Liquidity provider positions")).toBeTruthy();
  expect(screen.queryAllByText("Assets")).toBeTruthy();
  expect(screen.queryAllByText("Price")).toBeTruthy();
  expect(screen.queryAllByText("Amount")).toBeTruthy();
  expect(screen.queryAllByText("Value")).toBeTruthy();
  expect(screen.queryAllByText("APR")).toBeTruthy();
});
