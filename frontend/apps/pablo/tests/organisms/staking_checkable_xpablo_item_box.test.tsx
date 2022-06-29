import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_checkable_xpablo_item_box.stories";

const { Default } = composeStories(stories);

test("renders Staking/CheckableXPabloItemBox with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("fNFT 357")).toBeTruthy();
});
