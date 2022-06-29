import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/organisms/statsGovernanceTab.stories";

const { StatsGovernanceTabStory } = composeStories(stories);

test("renders Network Tabs", () => {
  render(<StatsGovernanceTabStory />);
  expect(screen.getByText("Proposals passed")).toBeInTheDocument();
});
