import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/pages/homepage.stories"; // import all stories from the stories file

const { Homepage } = composeStories(stories);

test("renders homepage with default args", () => {
  render(<Homepage />);

  expect(screen.queryAllByText("Overview")).toBeTruthy();
});
