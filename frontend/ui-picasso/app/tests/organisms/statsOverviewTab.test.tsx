import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-picasso/storybook/stories/organisms/statsOverviewTab.stories";

const { StatsOverviewTabStory } = composeStories(stories);

test("renders Network Tabs", () => {
  render(<StatsOverviewTabStory />);
  expect(screen.getByText("Daily active users")).toBeInTheDocument();
  expect(screen.getByText("Total transactions")).toBeInTheDocument();
});
