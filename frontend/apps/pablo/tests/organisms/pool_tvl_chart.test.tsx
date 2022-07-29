import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_tvl_chart.stories";

const { Default } = composeStories(stories);

test("renders PoolTVLChart with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("TVL")).toBeTruthy();
  expect(screen.queryAllByText("1w")).toBeTruthy();
  expect(screen.queryAllByText("1m")).toBeTruthy();
  expect(screen.queryAllByText("1y")).toBeTruthy();
  expect(screen.queryAllByText("All")).toBeTruthy();
});
