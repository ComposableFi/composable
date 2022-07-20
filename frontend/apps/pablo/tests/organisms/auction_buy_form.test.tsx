import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/auction_buy_form.stories";

const { Default } = composeStories(stories);

test("renders Auction/BuyForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Launch Token")).toBeTruthy();
});
