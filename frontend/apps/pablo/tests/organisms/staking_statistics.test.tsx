import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_statistics.stories";

const { Default } = composeStories(stories);

test("renders Staking/Statistics with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Total PBLO locked")).toBeTruthy();
  expect(screen.queryAllByText("Total CHAOS APY")).toBeTruthy();
  expect(screen.queryAllByText("Total CHAOS minted")).toBeTruthy();
  expect(screen.queryAllByText("Average lock multiplier")).toBeTruthy();
  expect(screen.queryAllByText("Average lock time")).toBeTruthy();
});
