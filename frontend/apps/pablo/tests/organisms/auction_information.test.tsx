import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/auction_information.stories";

const { Default } = composeStories(stories);

test("renders Auction/AuctionInformation with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Duration")).toBeTruthy();
});
