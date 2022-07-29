import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/auction_details.stories";

const { Default } = composeStories(stories);

test("renders Auction/AuctionDetails with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Token contract address")).toBeTruthy();
});
