import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/organisms/statsTelemetryTab.stories";

const { StatsTelemetryTabStory } = composeStories(stories);

test("renders Network Tabs", () => {
  render(<StatsTelemetryTabStory />);
  expect(screen.getByText("Average Time")).toBeInTheDocument();
});
