import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/atoms/base_asset.stories"; // import all stories from the stories file

const { DefaultBaseAsset } = composeStories(stories);

test("renders Base Asset With Default Args", () => {
  render(<DefaultBaseAsset />);
  expect(screen.getByText("ETH")).toBeInTheDocument();
  expect(screen.getByAltText("ETH")).toBeInTheDocument();
});
