import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_rewards_panel.stories";

const { Default } = composeStories(stories);

test("renders PoolRewardsPanel with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Your deposits")).toBeTruthy();
  expect(screen.queryAllByText("Your rewards")).toBeTruthy();
  expect(screen.queryAllByText("Claim rewards")).toBeTruthy();
});
