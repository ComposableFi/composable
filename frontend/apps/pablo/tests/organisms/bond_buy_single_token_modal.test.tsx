import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bond_buy_single_token_modal.stories";

const { Default } = composeStories(stories);

test("renders Bond/BuySingleTokenModal with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Swap")).toBeTruthy();
  expect(screen.queryAllByText("From")).toBeTruthy();
  expect(screen.queryAllByText("To")).toBeTruthy();
});
