import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/overview_wallet_breakdown_box.stories";

const { Default } = composeStories(stories);

test("renders Overview/WalletBreakdownBox with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Wallet Breakdown")).toBeTruthy();
  expect(screen.queryAllByText("Assets")).toBeTruthy();
  expect(screen.queryAllByText("Price")).toBeTruthy();
  expect(screen.queryAllByText("Amount")).toBeTruthy();
  expect(screen.queryAllByText("Value")).toBeTruthy();
});
