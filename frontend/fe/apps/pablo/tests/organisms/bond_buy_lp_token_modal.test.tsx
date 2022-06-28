import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bond_buy_lp_token_modal.stories";

const { Default } = composeStories(stories);

test("renders Bond/BuyLPTokenModal with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Create")).toBeTruthy();
  expect(screen.queryAllByText("Approve")).toBeTruthy();
  expect(screen.queryAllByText("Price and pool share")).toBeTruthy();
});
