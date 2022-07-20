import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_staking_panel.stories";

const { Default } = composeStories(stories);

test("renders PoolStakingPanel with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Stake")).toBeTruthy();
  expect(screen.queryAllByText("Unstake")).toBeTruthy();
});
