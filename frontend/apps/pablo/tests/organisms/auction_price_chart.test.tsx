import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/auction_price_chart.stories";

const { Default } = composeStories(stories);

test("renders Auction/PriceChart with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("predicted price")).toBeTruthy();
});
