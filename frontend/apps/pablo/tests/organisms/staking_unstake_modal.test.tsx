import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_unstake_modal.stories";

const { Default } = composeStories(stories);

test("renders Staking/RenewModal with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Burn and unstake you position")).toBeTruthy();
  expect(screen.queryAllByText("Withdrawable PBLO")).toBeTruthy();
  expect(screen.queryAllByText("Initial PBLO deposit")).toBeTruthy();
  expect(screen.queryAllByText("Burn and unstake")).toBeTruthy();
});
