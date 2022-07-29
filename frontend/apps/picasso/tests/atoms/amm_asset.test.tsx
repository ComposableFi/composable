import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/amm_asset.stories"; // import all stories from the stories file

const { Default } = composeStories(stories);

test("renders Base Asset With Default Args", () => {
  render(<Default />);
  expect(screen.getByText("Uniswap")).toBeInTheDocument();
  expect(screen.getByAltText("Uniswap")).toBeInTheDocument();
});
