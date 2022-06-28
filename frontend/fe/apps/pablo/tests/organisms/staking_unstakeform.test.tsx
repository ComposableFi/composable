import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_unstakeform.stories";

const { Default } = composeStories(stories);

test("renders Staking/UnstakeForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Renew")).toBeTruthy();
  expect(screen.queryAllByText("Burn and unstake")).toBeTruthy();
});
