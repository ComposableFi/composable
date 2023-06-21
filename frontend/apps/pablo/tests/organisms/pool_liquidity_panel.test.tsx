import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_liquidity_panel.stories";

const { Default } = composeStories(stories);

test.skip("renders PoolLiquidityPanel with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Add liquidity")).toBeTruthy();
  expect(screen.queryAllByText("Remove liquidity")).toBeTruthy();
  expect(screen.queryAllByText("Total value locked")).toBeTruthy();
  expect(screen.queryAllByText("My position")).toBeTruthy();
  expect(screen.queryAllByText("Pool share")).toBeTruthy();
});
