import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_stake_form.stories";

const { Default } = composeStories(stories);

test("renders PoolStakeForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Amount to stake")).toBeTruthy();
});
