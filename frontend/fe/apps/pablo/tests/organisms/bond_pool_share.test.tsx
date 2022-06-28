import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bond_pool_share.stories";

const { Default } = composeStories(stories);

test("renders Bond/PoolShare with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Share of pool")).toBeTruthy();
});
