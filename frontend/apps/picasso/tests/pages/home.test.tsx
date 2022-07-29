import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/templates/home.stories"; // import all stories from the stories file

const { HomePage } = composeStories(stories);

test("renders homepage with default args", () => {
  render(<HomePage />);

  expect(screen.queryAllByText("Overview")).toBeTruthy();
});
