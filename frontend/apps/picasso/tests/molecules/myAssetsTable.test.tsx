import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/myAssetsTable.stories"; // import all stories from the stories file

const { MyAssetsTableStory } = composeStories(stories);

test("It renders properly", () => {
  render(<MyAssetsTableStory />);

  expect(screen.getByText("PICA")).toBeInTheDocument();
});
