import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/all_liquidity_table.stories";

const { Default } = composeStories(stories);

test("renders AllLiquidityTable with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Pool")).toBeTruthy();
});
