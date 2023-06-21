import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_claimable_rewards.stories";

const { Default } = composeStories(stories);

test("renders Staking/ClaimableRewards with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Claimable rewards")).toBeTruthy();
  expect(screen.queryAllByText("Claim all")).toBeTruthy();
});
