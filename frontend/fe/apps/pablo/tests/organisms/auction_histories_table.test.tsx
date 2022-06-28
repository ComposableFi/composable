import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/auction_histories_table.stories";

const { Default } = composeStories(stories);

test("renders AuctionHistoriesTable with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Time")).toBeTruthy();
  expect(screen.queryAllByText("Type")).toBeTruthy();
  expect(screen.queryAllByText("Input")).toBeTruthy();
  expect(screen.queryAllByText("Output")).toBeTruthy();
});
