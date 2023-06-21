import { getToken, TOKEN_IDS } from "tokens";
import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/token_asset.stories"; // import all stories from the stories file

const { TokenAssets } = composeStories(stories);

test("renders Token Asset With Default Args", () => {
  const tokenId = TOKEN_IDS[0];
  const token = getToken(tokenId);
  render(<TokenAssets />);
  expect(screen.getByText(token.symbol)).toBeInTheDocument();
  expect(screen.getByAltText(token.symbol)).toBeInTheDocument();
});
