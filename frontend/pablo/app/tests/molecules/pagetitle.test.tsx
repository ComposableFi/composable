import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/molecules/pagetitle.stories"; // import all stories from the stories file

const { PageTitleStory } = composeStories(stories);

test("It renders properly", () => {
  render(<PageTitleStory />);

  expect(screen.getByText("Overview")).toBeInTheDocument();
});

test("It accepts title as mandatory prop", () => {
  render(<PageTitleStory title="Anything" subtitle="Override" />);
  expect(screen.getByText("Anything")).toBeInTheDocument();
  expect(screen.getByText("Override")).toBeInTheDocument();
});
