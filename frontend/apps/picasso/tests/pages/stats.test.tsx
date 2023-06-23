import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/templates/stats.stories";
import { useConnector } from "bi-lib";

const { StatsPage } = composeStories(stories);

test("renders Stats page with default args", () => {
  render(<StatsPage />);
  expect(useConnector).toBeCalled();
  expect(
    screen.getByText("All of Picasso's global information at a glance.")
  ).toBeInTheDocument();
  expect(screen.getByText("Telemetry")).toBeInTheDocument();
});