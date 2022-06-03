import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-pablo/storybook/stories/organisms/staking_select_lock_period.stories";

const { Default } = composeStories(stories);

test("renders Staking/SelectLockPeriod with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Select lock period (multiplier)")).toBeTruthy();
  expect(screen.queryAllByText("Unlock date")).toBeTruthy();
});
