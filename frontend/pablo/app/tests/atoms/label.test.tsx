import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/atoms/label.stories"; // import all stories from the stories file

const { TooltipLabels, TooltipLabelsWithBalance } = composeStories(stories);

test("renders Text Only Selects", () => {
  render(<TooltipLabels />);
  expect(screen.getByText("Label master here")).toBeInTheDocument();
});

test("Render <TooltipLabelsWithBalance />", () => {
  render(<TooltipLabelsWithBalance />);
  expect(screen.getByText("Balance")).toBeInTheDocument();
  expect(screen.getByText("Amount")).toBeInTheDocument();
});
