import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/all_auctions_table.stories";

const { Default } = composeStories(stories);

test("renders AllAuctionsTable with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Token")).toBeTruthy();
  expect(screen.queryAllByText("Network")).toBeTruthy();
  expect(screen.queryAllByText("Auction Status")).toBeTruthy();
  expect(screen.queryAllByText("Price")).toBeTruthy();
});
