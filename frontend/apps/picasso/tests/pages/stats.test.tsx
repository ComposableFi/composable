import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/templates/stats.stories";
import { useConnector } from "bi-lib";

const { StatsPage } = composeStories(stories);

test("renders Stats page with default args", () => {
  render(<StatsPage />);
  expect(useConnector).toBeCalled();
  expect(
    screen.getByText(
      "You will be able to see all Picasso's global information here."
    )
  ).toBeInTheDocument();
  expect(screen.getByText("Telemetry")).toBeInTheDocument();
});
