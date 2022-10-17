import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/organisms/statsOverviewTab.stories";

const { StatsOverviewTabStory } = composeStories(stories);

test("renders Network Tabs", () => {
  render(<StatsOverviewTabStory />);
  expect(screen.getByText("Picasso market cap")).toBeInTheDocument();
  expect(screen.getByText("Picasso circulating supply")).toBeInTheDocument();
  expect(screen.getByText("0 PICA")).toBeInTheDocument();
  expect(screen.getByText("$0.00")).toBeInTheDocument();
});
