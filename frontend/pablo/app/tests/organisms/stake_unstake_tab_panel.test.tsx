import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/organisms/stake_unstake_tab_panel.stories";

const { StakeTabPanel, UnstakeTabPanel } = composeStories(stories);

test("renders StakeTabPanel with default args", () => {
  render(<StakeTabPanel />);

  expect(screen.queryAllByText("Approve")).toBeTruthy();
});

test("renders UnstakeTabPanel with default args", () => {
  render(<UnstakeTabPanel />);

  expect(screen.queryAllByText("Unstake")).toBeTruthy();
});
