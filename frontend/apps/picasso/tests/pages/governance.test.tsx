import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/templates/governance.stories";

const { GovernancePage } = composeStories(stories);

test("renders Governance page with default args", () => {
  render(<GovernancePage />);

  expect(screen.getByText("Voting")).toBeInTheDocument();
  expect(screen.getByText("Discussion")).toBeInTheDocument();
});
