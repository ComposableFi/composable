import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bond_portfolio_chart.stories";

const { Default } = composeStories(stories);

test("renders Bonds/PortfolioChart with default args", () => {
  render(<Default />);
  expect(screen.queryAllByText("My portfolio")).toBeTruthy();
  expect(screen.queryAllByText("24h")).toBeTruthy();
  expect(screen.queryAllByText("1w")).toBeTruthy();
  expect(screen.queryAllByText("1m")).toBeTruthy();
  expect(screen.queryAllByText("1y")).toBeTruthy();
});
