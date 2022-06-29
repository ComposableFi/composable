import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/textSwitch.stories";

const { TextSwitchesNoTooltip, TextSwitchesWithTooltip } =
  composeStories(stories);

test("render <TextSwitch /> without icon properly", () => {
  const { container } = render(<TextSwitchesNoTooltip />);

  expect(screen.getByText("Text element")).toBeInTheDocument();
  expect(container.getElementsByClassName("MuiSwitch-root").length).toBe(1);
});

test("render <TextSwitch /> with icon properly", () => {
  const { container } = render(<TextSwitchesWithTooltip />);

  expect(screen.getByText("Text element")).toBeInTheDocument();
  expect(screen.getByTestId("InfoOutlinedIcon")).toBeInTheDocument();
  expect(container.getElementsByClassName("MuiSwitch-root").length).toBe(1);
});
