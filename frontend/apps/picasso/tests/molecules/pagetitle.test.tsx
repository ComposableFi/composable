import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/pageTitle.stories"; // import all stories from the stories file

const { PageTitleStory, PageTitleWithAlignCenter } = composeStories(stories);

test("It renders properly", () => {
  render(<PageTitleStory />);

  expect(screen.getByText("Overview")).toBeInTheDocument();
});

test("It accepts title as mandatory prop", () => {
  render(<PageTitleStory title="Custom" />);

  expect(screen.getByText("Custom")).toBeInTheDocument();
});

test("it runs with other args", () => {
  render(<PageTitleWithAlignCenter />);

  expect(screen.getByText("Pass another text here")).toBeInTheDocument();
});
