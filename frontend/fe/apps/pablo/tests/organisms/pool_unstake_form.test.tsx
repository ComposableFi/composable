import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_unstake_form.stories";

const { Default } = composeStories(stories);

test("renders PoolUnstakeForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Amount to unstake")).toBeTruthy();
});
