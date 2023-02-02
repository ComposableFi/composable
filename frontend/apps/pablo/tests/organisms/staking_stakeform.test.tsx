import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_stakeform.stories";

const { Default } = composeStories(stories);

test("renders Staking/StakeForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Enter amount to stake")).toBeTruthy();
  expect(screen.queryAllByText("Select lock period")).toBeTruthy();
  expect(screen.queryAllByText("Unlock date")).toBeTruthy();
  expect(screen.queryAllByText("Stake and mint")).toBeTruthy();
});
